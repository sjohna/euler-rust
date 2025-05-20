use crate::util;

pub fn iterator() -> i64 {
    std::iter::from_fn(util::primes()).nth(10_000).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator() {
        assert_eq!(super::iterator(), 104743)
    }
}