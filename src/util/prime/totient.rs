use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use crate::util;
use crate::util::integer;

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

#[cfg(test)]
mod tests {
    use crate::util::prime;
    use super::{totient_brute_force, totient_from_prime_factorization_using_sieve};

    #[test]
    fn test_totient_from_prime_factorization() {
        let primes = prime::seq::sieve_up_to(5_000_000).collect::<Vec<_>>();

        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(1, &primes)), 1);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(2, &primes)), 1);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(3, &primes)), 2);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(4, &primes)), 2);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(5, &primes)), 4);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(6, &primes)), 2);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(7, &primes)), 6);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(8, &primes)), 4);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(9, &primes)), 6);

        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(499998, &primes)), 165336);
        assert_eq!(totient_from_prime_factorization_using_sieve(&prime::prime_factorization(4999980, &primes)), 1322688);
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