use std::iter::once;

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

#[cfg(test)]
mod tests {
    #[test]
    fn euler3_all_together() {
        assert_eq!(super::euler3_all_together(), 6857);
    }
}