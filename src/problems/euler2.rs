use crate::util::integer;

type Impl = fn(i64) -> i64;

pub fn simple_loop(threshold: i64) -> i64 {
    let mut prev = 0;
    let mut curr = 1;
    let mut sum = 0;

    while curr < threshold {
        if curr % 2 == 0 {
            sum += curr
        }

        let next = curr + prev;
        prev = curr;
        curr = next;
    }

    sum
}

pub fn custom_iterator_function(threshold: i64) -> i64 {
    integer::seq::fibonacci().take_while(|n| *n < threshold).filter(|n| n % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    const TESTS: [(i64,i64);1] = [(4_000_000, 4_613_732)];

    fn test(func: super::Impl) {
        for tc in TESTS {
            assert_eq!(func(tc.0), tc.1);
        }
    }

    #[test]
    fn simple_loop() {
        test(super::simple_loop);
    }

    #[test]
    fn custom_iterator_function() {
        test(super::custom_iterator_function);
    }
}