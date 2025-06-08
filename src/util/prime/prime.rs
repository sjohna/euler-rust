use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use crate::util;
use crate::util::integer;

pub fn prime_factorization(mut n: i64, primes: &[i64]) -> Vec<(i64, u32)> {
    let mut m = Vec::<(i64,u32)>::new();

    let mut primes_index = 0;
    while primes_index < primes.len() && n > 1 {
        let curr_prime = primes[primes_index];
        let mut curr_prime_count = 0;
        while n % curr_prime == 0 {
            n /= curr_prime;
            curr_prime_count += 1;
        }

        if curr_prime_count > 0 {
            m.push((curr_prime, curr_prime_count));
        }

        primes_index += 1;
    }

    m
}

pub fn totient_brute_force(n: i64) -> i64 {
    let mut count = 0;
    for i in 1..=n {
        if integer::gcd(n,i) == 1 {
            count += 1;
        }
    }

    count
}

static totient_cache: LazyLock<Mutex<HashMap<i64,i64>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn totient_from_prime_factorization_using_sieve(prime_factorization: &[(i64, u32)]) -> i64 {
    let mut sieve_length = 1;
    let mut rest = 1;
    for factor in prime_factorization {
        sieve_length *= factor.0;
        rest *= factor.0.pow(factor.1-1);
    }

    if totient_cache.lock().unwrap().contains_key(&sieve_length) {
        return totient_cache.lock().unwrap().get(&sieve_length).unwrap() * rest;
    }

    if prime_factorization.len() <= 16 {
        // use inclusion/exclusion
        let mut total = 0;
        for set in util::power_set::<i64>(prime_factorization.iter().map(|e| e.0).collect::<Vec<i64>>().as_slice()) {
            let prod = set.iter().product::<i64>();
            if set.len() % 2 == 0 {
                total += sieve_length / prod;
            } else {
                total -= sieve_length / prod;
            }
        }

        totient_cache.lock().unwrap().insert(sieve_length, total as i64);
        return total * rest;
    }

    let mut sieve = vec![true; (sieve_length+1) as usize];

    for factor in prime_factorization {
        for index in (factor.0..=sieve_length).step_by(factor.0 as usize) {
            sieve[index as usize] = false;
        }
    }

    let count_in_sieve = sieve.into_iter().skip(1).filter(|v| *v).count();  // why does the type of v change between .iter() and .into_iter()?

    totient_cache.lock().unwrap().insert(sieve_length, count_in_sieve as i64);

    count_in_sieve as i64 * rest
}

// returns prime factorization of product of numbers in [min,max]
// aka the prime factorization of factorial(max) / factorial(min-1)
pub fn prime_factorization_of_product_of_range(min: i64, max: i64, primes: &[i64]) -> Vec<(i64,i64)> {
    let mut ret = Vec::<(i64,i64)>::new();

    for (index, prime) in primes.into_iter().enumerate() {
        for power in 1.. {
            let prime_power = prime.pow(power);
            if prime_power > max {
                break;
            }

            let mut first_hit = min;
            if min % prime_power != 0 {
                first_hit += prime_power - min % prime_power;
            }

            if first_hit > max {
                continue;
            }

            let range = max - first_hit;
            let num: i64 = range / prime_power + 1;

            if power == 1 {
                ret.push((*prime, num));
            } else {
                let i = ret.len()-1;
                ret[i].1 += num;
            }
        }
    }

    ret
}

pub fn smallest_prime_factor_up_to(n: i64, primes: Vec<i64>) -> Vec<i64> {
    let mut ret = vec![0; n as usize + 1];

    for p in primes {
        for i in (p..=n).step_by(p as usize) {   // TODO: is it more efficient to write a basic for loop?
            if ret[i as usize] == 0 {
                ret[i as usize] = p;
            }
        }
    }

    ret
}

pub fn factors_iter(prime_factors: &[(i64, u32)]) -> impl Iterator<Item = i64> {
    let mut powers = vec![0; prime_factors.len()];
    let mut done = false;

    std::iter::from_fn(move || {
        if done {
            return None;
        }

        let mut product = 1;
        for i in 0..prime_factors.len() {
            product *= i64::pow(prime_factors[i].0, powers[i])
        }

        powers[0] += 1;
        for i in 0..prime_factors.len() {
            if powers[i] > prime_factors[i].1 {
                powers[i] = 0;
                if i < prime_factors.len() - 1 {
                    powers[i+1] += 1;
                } else {
                    done = true;
                }
            }
        }

        return Some(product);
    })
}

#[cfg(test)]
mod tests {
    use crate::util::prime;
    use crate::util::prime::{factors_iter, prime_factorization, prime_factorization_of_product_of_range, totient_brute_force, totient_from_prime_factorization_using_sieve};

    #[test]
    fn test_prime_factorization() {
        let primes = prime::seq::sieve_up_to(100_000).collect::<Vec<i64>>();

        let prime_factors = prime_factorization(2, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 1)]);

        let prime_factors = prime_factorization(3, primes.as_slice());
        assert_eq!(prime_factors, vec![(3, 1)]);

        let prime_factors = prime_factorization(4, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 2)]);

        let prime_factors = prime_factorization(6, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 1), (3, 1)]);

        let prime_factors = prime_factorization(12, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 2), (3, 1)]);

        let prime_factors = prime_factorization(2*2*2*3*17*17*97*97*541, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 3), (3, 1), (17, 2), (97, 2), (541,1)]);
    }

    #[test]
    fn test_factors_iter() {
        let primes = prime::seq::sieve_up_to(100_000).collect::<Vec<i64>>();

        let prime_factors = prime_factorization(2, primes.as_slice());
        let factors = factors_iter(prime_factors.as_slice()).collect::<Vec<i64>>();
        assert_eq!(factors, vec![1, 2]);

        let prime_factors = prime_factorization(3, primes.as_slice());
        let factors = factors_iter(prime_factors.as_slice()).collect::<Vec<i64>>();
        assert_eq!(factors, vec![1, 3]);

        let prime_factors = prime_factorization(4, primes.as_slice());
        let factors = factors_iter(prime_factors.as_slice()).collect::<Vec<i64>>();
        assert_eq!(factors, vec![1, 2, 4]);

        let prime_factors = prime_factorization(6, primes.as_slice());
        let factors = factors_iter(prime_factors.as_slice()).collect::<Vec<i64>>();
        assert_eq!(factors, vec![1, 2, 3, 6]);

        let prime_factors = prime_factorization(12, primes.as_slice());
        let mut factors = factors_iter(prime_factors.as_slice()).collect::<Vec<i64>>();
        factors.sort();
        assert_eq!(factors, vec![1, 2, 3, 4, 6, 12]);

        let prime_factors = prime_factorization(30, primes.as_slice());
        let mut factors = factors_iter(prime_factors.as_slice()).collect::<Vec<i64>>();
        factors.sort();
        assert_eq!(factors, vec![1, 2, 3, 5, 6, 10, 15, 30]);
    }

    #[test]
    fn test_prime_factorization_of_product_of_range() {
        let primes = prime::seq::sieve_up_to(100_000).collect::<Vec<i64>>();

        let prime_factors = prime_factorization_of_product_of_range(1, 5, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 3), (3,1), (5,1)]);

        let prime_factors = prime_factorization_of_product_of_range(1, 6, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 4), (3,2), (5,1)]);

        let prime_factors = prime_factorization_of_product_of_range(5, 10, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 5), (3,3), (5,2), (7,1)]);

        let prime_factors = prime_factorization_of_product_of_range(8, 10, primes.as_slice());
        assert_eq!(prime_factors, vec![(2, 4), (3,2), (5,1)]);

        let prime_factors = prime_factorization_of_product_of_range(9, 9, primes.as_slice());
        assert_eq!(prime_factors, vec![(3,2)]);
    }

    #[test]
    fn test_totient_from_prime_factorization() {
        let primes = prime::seq::sieve_up_to(5_000_000).collect::<Vec<_>>();

        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(1, &primes)), 1);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(2, &primes)), 1);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(3, &primes)), 2);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(4, &primes)), 2);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(5, &primes)), 4);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(6, &primes)), 2);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(7, &primes)), 6);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(8, &primes)), 4);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(9, &primes)), 6);

        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(499998, &primes)), 165336);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime_factorization(4999980, &primes)), 1322688);
    }

    #[test]
    fn test_totient_brute_force() {
        assert_eq!(totient_brute_force(1), 1);
        assert_eq!(totient_brute_force(2), 1);
        assert_eq!(totient_brute_force(3), 2);
        assert_eq!(totient_brute_force(4), 2);
        assert_eq!(totient_brute_force(5), 4);
        assert_eq!(totient_brute_force(6), 2);
        assert_eq!(totient_brute_force(7), 6);
        assert_eq!(totient_brute_force(8), 4);
        assert_eq!(totient_brute_force(9), 6);

        assert_eq!(totient_brute_force(499998), 165336);
        assert_eq!(totient_brute_force(4999980), 1322688);
    }
}