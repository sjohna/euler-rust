use crate::util::integer;

pub fn iter() -> i64 {
    (1..=20).reduce(integer::lcm).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter() {
        assert_eq!(super::iter(), 232792560);
    }
}