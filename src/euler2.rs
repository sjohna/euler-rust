use crate::util;

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

pub fn euler2_custom_iterator_function() -> i64 {
    std::iter::from_fn(util::fibonacci_sequence()).take_while(|n| n < &4_000_000).filter(|n| n % 2 == 0).sum()  // I don't need the explicit type annotation for sum here like I do in problem 1. Why?
    // also: why do I need an '&' in the take_while ("n < &4_000_000"), but not in the filter (I *can* do "n % &2 == 0", but I don't get a compiler error if I don't)
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler2_loop() {
        assert_eq!(super::euler2_loop(), 4_613_732);
    }

    #[test]
    fn euler2_custom_iterator_function() {
        assert_eq!(super::euler2_custom_iterator_function(), 4_613_732);
    }
}