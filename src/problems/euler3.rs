use std::iter::once;
use crate::util::prime;

type Impl = fn(i64) -> i64;

pub fn all_together(mut num: i64) -> i64 {
    // divide out all factors of 2
    while num % 2 == 0 {
        num /= 2
    }

    // divide out primes 3 and up
    let mut next_prime = 3;
    loop {
        while num % next_prime == 0 {
            num /= next_prime;
        }

        if num == 1 {
            return next_prime;
        }

        // find the next prime
        let mut next_candidate = next_prime + 2;
        loop {
            let max = i64::isqrt(next_candidate);

            let mut factor_found = false;
            for factor in once(2).chain((3..max).step_by(2)) {  // check 2 but skip other evens
                if next_candidate % factor == 0 {
                    factor_found = true;
                    break;
                }
            }

            if !factor_found {
                next_prime = next_candidate;
                break;
            }

            next_candidate += 2;
        }
    }
}

pub fn iterator(mut num: i64) -> i64 {
    let mut primes = prime::seq::naive_trial_division();

    let mut curr_prime = primes.next().unwrap();

    loop {
        while num % curr_prime == 0 {
            num /= curr_prime;
        }

        if num == 1 {
            return curr_prime;
        }

        curr_prime = primes.next().unwrap();
    }
}

#[cfg(test)]
mod tests {
    const TESTS: [(i64,i64); 1] = [(600851475143, 6857)];

    fn test(func: super::Impl) {
        for tc in TESTS {
            assert_eq!(func(tc.0), tc.1);
        }
    }

    #[test]
    fn all_together() {
        test(super::all_together);
    }

    #[test]
    fn iterator() {
        test(super::iterator);
    }
}