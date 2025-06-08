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
}