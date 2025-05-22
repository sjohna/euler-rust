use std::ops::Mul;
use std::time::{Duration, SystemTime};
use num_bigint::BigInt;

pub fn fibonacci_sequence() -> impl FnMut() -> Option<i64> {
    let (mut curr, mut next): (i64, i64) = (0,1);
    move || -> Option<i64> {
        (curr, next) = (next, curr + next);
        Some(curr)
    }
}

pub fn triangular_number(n: i64) -> i64 {
    (n * (n + 1)) / 2
}

pub fn pentagonal_number(n: i64) -> i64 {
    (n * (3*n - 1)) / 2
}

pub fn choose(n: i32, r: i32) -> BigInt {
    factorial(n) / (factorial(r) * factorial(n-r))
}

pub fn factorial(n: i32) -> BigInt {
    let mut fac = BigInt::from(1);
    for i in 1..=n {
        fac *= BigInt::from(i);
    }

    fac
}

pub fn root_digit_sum(n: i32, digits: i32) -> i32 {
    let mut root = BigInt::from(0);

    for magnitude in 0..digits {
        let target = BigInt::from(n) * BigInt::from(10).pow((2 * magnitude) as u32);
        root *= 10;

        // binary search to find the next digit
        let mut min = 0;
        let mut max = 9;
        while max - min > 0 {
            let next_digit = (max + min + 1) / 2;
            let candidate = &root + next_digit; // why do I need a & on root here?

            let candidate_squared = &candidate * &candidate;

            if candidate_squared > target {   // and here?
                max = next_digit-1;
            } else {
                min = next_digit;
            }
        }

        root += min;
    }

    digit_sum(root)
}

pub fn digit_sum(n: BigInt) -> i32 {
    let str = n.to_string();   // why the second toString() here?
    let mut sum = 0;
    for digit in str.chars() {
        sum += digit as i32 - '0' as i32;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn triangular_number() {
        assert_eq!(super::triangular_number(1), 1);
        assert_eq!(super::triangular_number(2), 3);
        assert_eq!(super::triangular_number(3), 6);
        assert_eq!(super::triangular_number(4), 10);
        assert_eq!(super::triangular_number(5), 15);
        assert_eq!(super::triangular_number(6), 21);
        assert_eq!(super::triangular_number(7), 28);
        assert_eq!(super::triangular_number(8), 36);
        assert_eq!(super::triangular_number(9), 45);
        assert_eq!(super::triangular_number(10), 55);
        assert_eq!(super::triangular_number(100), 5050);
        assert_eq!(super::triangular_number(1000), 500500);
    }

    #[test]
    fn pentagonal_number() {
        assert_eq!(super::pentagonal_number(1), 1);
        assert_eq!(super::pentagonal_number(2), 5);
        assert_eq!(super::pentagonal_number(3), 12);
        assert_eq!(super::pentagonal_number(4), 22);
        assert_eq!(super::pentagonal_number(5), 35);
        assert_eq!(super::pentagonal_number(6), 51);
        assert_eq!(super::pentagonal_number(7), 70);
        assert_eq!(super::pentagonal_number(8), 92);
        assert_eq!(super::pentagonal_number(9), 117);
        assert_eq!(super::pentagonal_number(10), 145);
    }
}

pub fn table<T: ToString + Copy,V: std::fmt::Display>(inputs: &[T], func: fn(T) -> V) {
    let mut input_strings = Vec::new();
    for input in inputs {
        input_strings.push(input.to_string());
    }

    let max_length = input_strings.iter().map(|s| s.len()).max().unwrap();

    for i in 0..inputs.len() {
        print!("{:<max_length$} | ", input_strings[i]);
        let (output, elapsed) = time(func, inputs[i]);
        print!("{} ({:?})\n", output, elapsed);
    }
}

pub fn time<T,V>(func: fn(T) -> V, input: T) -> (V, Duration) {
    let start = SystemTime::now();
    let output = func(input);
    let elapsed = start.elapsed().unwrap();
    (output, elapsed)
}

// doesn't handle raising to a zero power
pub fn pow_seq<T: Mul<Output = T> + Copy>(base: T, min: usize, max: usize) -> Vec<T> {
    let mut pow = base;
    for _ in 1..min {
        pow = pow * base;
    }

    let mut ret = vec![pow];

    for _ in min..max {
        pow = pow * base;
        ret.push(pow);
    }

    ret
}