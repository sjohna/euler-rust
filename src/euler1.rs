#[allow(dead_code)]
pub fn euler1_loop() -> i32 {
    let mut sum = 0;

    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler1_loop() {
        assert_eq!(euler1_loop(), 233168);
    }
}