use crate::util::prime;

pub fn iterator() -> i64 {
    prime::iter::naive_trial_division().nth(10_000).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator() {
        assert_eq!(super::iterator(), 104743)
    }
}