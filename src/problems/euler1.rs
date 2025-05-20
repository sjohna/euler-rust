type Impl = fn(i64) -> i64;

pub fn simple_loop(n: i64) -> i64 {
    let mut sum = 0;

    for n in 1..n {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }

    sum
}

pub fn iterator(n: i64) -> i64 {
    (1..n).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn custom_iterator_function(max: i64) -> i64 {
    std::iter::from_fn(natural_numbers()).take_while(|n| *n < max).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

fn natural_numbers() -> impl FnMut() -> Option<i64> {
    let mut num = 0;
    move || -> Option<i64> {
        num += 1;
        Some(num)
    }
}

#[cfg(test)]
mod tests {
    const TESTS: [(i64, i64); 1] = [
        (1000,233168),
    ];

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
    fn iterator() {
        test(super::iterator);
    }

    #[test]
    fn custom_iterator_function() {
        test(super::custom_iterator_function);
    }
}