use crate::util;

pub fn euler5_iter() -> i64 {
    (1..=20).reduce(util::lcm).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler5_iter() {
        assert_eq!(super::euler5_iter(), 232792560);
    }
}