use crate::util;

pub fn iter() -> i32 {
    (1..=100).filter(|n| i32::isqrt(*n).pow(2) != *n).map(|n| util::root_digit_sum(n,100)).sum()  // why do I need a * here? wtf?
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter() {
        assert_eq!(super::iter(), 40886);
    }
}