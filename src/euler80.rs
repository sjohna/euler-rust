use crate::util;

pub fn euler80_iter() -> i32 {
    (1..=100).filter(|n| i32::isqrt(*n).pow(2) != *n).map(|n| util::root_digit_sum(n,100)).sum()  // why do I need a * here? wtf?
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler80_iter() {
        assert_eq!(super::euler80_iter(), 40886);
    }
}