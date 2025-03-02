#[allow(dead_code)]
pub fn euler2_loop() -> i32 {
    let mut prev = 0;
    let mut curr = 1;
    let mut sum = 0;

    while curr < 4_000_000 {
        if curr % 2 == 0 {
            sum += curr
        }

        let next = curr + prev;
        prev = curr;
        curr = next;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler2_loop() {
        assert_eq!(super::euler2_loop(), 4_613_732);
    }
}