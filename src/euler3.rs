use std::iter::once;
use crate::util;

pub fn euler3_all_together() -> i64 {
    // calculate primes and then divide them out of the number all at once

    let mut num : i64 = 600851475143;

    // no factors of 2 in the number, so start with 3
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

pub fn euler3_iterator() -> i64 {
    let mut num : i64 = 600851475143;
    let mut primes = std::iter::from_fn(util::primes());    // if I don't make this mut, I get an error on the next line: cannot borrow immutable local variable primes as mutable. What gives?

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
    #[test]
    fn euler3_all_together() {
        assert_eq!(super::euler3_all_together(), 6857);
    }

    #[test]
    fn euler3_iterator() {
        assert_eq!(super::euler3_iterator(), 6857);
    }
}