use super::is_prime;

pub fn naive_trial_division() -> impl Iterator<Item = i64> {
    std::iter::from_fn(naive_trial_division_fn())
}

pub fn naive_trial_division_fn() -> impl FnMut() -> Option<i64> {
    let mut next_candidate: i64 = 2;
    move || -> Option<i64> {
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
    }
}
