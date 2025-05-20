pub fn simple_loop() -> i32 {
    let mut sum = 0;

    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }

    sum
}

pub fn iterator() -> i32 {
    (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn custom_iterator_function() -> i32 {
    std::iter::from_fn(natural_numbers()).take_while(|n| n < &1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()  // what is the & I need before the literal 1000 in the take_while?
}

fn natural_numbers() -> impl FnMut() -> Option<i32> {   // this needs to be FnMut instead of Fn. What's the difference?
    let mut num = 0;
    move || -> Option<i32> {    // I need `move` here or I get heinous compiler errors. What's this doing?
        num += 1;
        Some(num)
    }
}

#[cfg(test)]
mod tests {
    // TODO: make these take a number, do tests on array of test tuples or structs
    #[test]
    fn simple_loop() {
        assert_eq!(super::simple_loop(), 233168);
    }

    #[test]
    fn iterator() {
        assert_eq!(super::iterator(), 233168);
    }

    #[test]
    fn custom_iterator_function() {
        assert_eq!(super::custom_iterator_function(), 233168);
    }
}