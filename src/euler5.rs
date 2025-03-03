use crate::util;

#[allow(dead_code)]
pub fn euler5_iter() -> i64 {
    (1..=20).reduce(|a, b| util::lcm(a, b)).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler5_iter() {
        assert_eq!(super::euler5_iter(), 232792560);
    }
}