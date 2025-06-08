use crate::util::integer;
use crate::util::integer::IntegerExt;

pub fn brute_force(n: i64) -> i64 {
    let mut count = 0;
    for i in 1..=n {
        if integer::gcd(n,i) == 1 {
            count += 1;
        }
    }

    count
}

pub fn from_prime_factorization(mut n: i64, prime_factorization: &[(i64, u32)]) -> i64 {
    let mut totient = n;

    for (prime, _) in prime_factorization {
        totient -= totient / *prime;
        n = n.divide_out(*prime);
    }

    totient
}

#[cfg(test)]
mod tests {
    use crate::util::prime;
    use super::{brute_force, from_prime_factorization};

    #[test]
    fn test_totient_from_prime_factorization() {
        let primes = prime::seq::sieve_up_to(5_000_000).collect::<Vec<_>>();

        assert_eq!(from_prime_factorization(1, &prime::prime_factorization(1, &primes)), 1);
        assert_eq!(from_prime_factorization(2, &prime::prime_factorization(2, &primes)), 1);
        assert_eq!(from_prime_factorization(3, &prime::prime_factorization(3, &primes)), 2);
        assert_eq!(from_prime_factorization(4, &prime::prime_factorization(4, &primes)), 2);
        assert_eq!(from_prime_factorization(5, &prime::prime_factorization(5, &primes)), 4);
        assert_eq!(from_prime_factorization(6, &prime::prime_factorization(6, &primes)), 2);
        assert_eq!(from_prime_factorization(7, &prime::prime_factorization(7, &primes)), 6);
        assert_eq!(from_prime_factorization(8, &prime::prime_factorization(8, &primes)), 4);
        assert_eq!(from_prime_factorization(9, &prime::prime_factorization(9, &primes)), 6);

        assert_eq!(from_prime_factorization(499998, &prime::prime_factorization(499998, &primes)), 165336);
        assert_eq!(from_prime_factorization(4999980, &prime::prime_factorization(4999980, &primes)), 1322688);
    }

    #[test]
    fn test_totient_brute_force() {
        assert_eq!(brute_force(1), 1);
        assert_eq!(brute_force(2), 1);
        assert_eq!(brute_force(3), 2);
        assert_eq!(brute_force(4), 2);
        assert_eq!(brute_force(5), 4);
        assert_eq!(brute_force(6), 2);
        assert_eq!(brute_force(7), 6);
        assert_eq!(brute_force(8), 4);
        assert_eq!(brute_force(9), 6);

        assert_eq!(brute_force(499998), 165336);
        assert_eq!(brute_force(4999980), 1322688);
    }
}