use crate::util;

pub fn iter() -> i64 {
    (1..=20).reduce(util::lcm).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter() {
        assert_eq!(super::iter(), 232792560);
    }
}