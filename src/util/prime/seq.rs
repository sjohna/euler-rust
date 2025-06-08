use super::is_prime;

pub fn naive_trial_division() -> impl Iterator<Item = i64> {
    let mut next_candidate: i64 = 2;

    std::iter::from_fn(move || -> Option<i64> {
        if next_candidate == 2 {
            next_candidate = 3;
            return Some(2)
        }

        loop {
            if is_prime::naive_trial_division(next_candidate) {
                let next_prime = next_candidate;
                next_candidate += 2;
                return Some(next_prime);
            }

            next_candidate += 2;
        }
    })
}

pub fn sieve_up_to(n: i64) -> impl Iterator<Item = i64> {
    let mut sieve = vec![false; (n+1).try_into().unwrap()];
    sieve[0] = true;
    sieve[1] = true;

    let mut start_index = 0;

    std::iter::from_fn(move || -> Option<i64> {
        while start_index < sieve.len() {
            if !sieve[start_index] {
                break;
            }

            start_index += 1;
        }

        if start_index >= sieve.len() {
            return None
        }

        let next_prime = start_index;

        for i in (start_index*2..sieve.len()).step_by(next_prime) {
            sieve[i] = true;
        }

        start_index += 1;
        Some(next_prime.try_into().unwrap())
    })
}

#[cfg(test)]
mod tests {
    const first_ten_primes: [i64; 10] = [2,3,5,7,11,13,17,19,23,29];

    #[test]
    fn naive_trial_division_first_ten() {
        let first_ten = super::naive_trial_division().take(10).collect::<Vec<_>>();
        assert_eq!(first_ten, first_ten_primes);
    }

    #[test]
    fn sieve_up_to_first_ten() {
        let first_ten = super::sieve_up_to(29).take(10).collect::<Vec<_>>();
        assert_eq!(first_ten, first_ten_primes);
    }

    #[test]
    fn naive_trial_division_10000th() {
        assert_eq!(super::naive_trial_division().nth(9999).unwrap(), 104729);
    }

    #[test]
    fn sieve_up_to_millionth() {
        assert_eq!(super::sieve_up_to(50_000_000).nth(999_999).unwrap(), 15485863);
    }
}