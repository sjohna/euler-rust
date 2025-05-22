use crate::util::prime;

pub fn iterator(n: usize) -> i64 {
    prime::seq::naive_trial_division().nth(n-1).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator() {
        assert_eq!(super::iterator(10_001), 104743)
    }
}