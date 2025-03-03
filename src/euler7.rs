use crate::util;

pub fn euler7_iterator() -> i64 {
    std::iter::from_fn(util::primes()).skip(10_000).next().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_euler7_iterator() {
        assert_eq!(super::euler7_iterator(), 104743)
    }
}