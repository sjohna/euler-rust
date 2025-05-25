use std::ops::Mul;
use std::time::{Duration, SystemTime};
use num_bigint::BigInt;

pub fn graph_of<T: Clone,R>(mut func: impl FnMut(T) -> R) -> impl FnMut(T) -> (T, R) {
    move |t: T| {
        (t.clone(), func(t))
    }
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

// pub trait SelectAt {
//     type T;
//     fn select_at(&self, indices: &[usize]) -> &[Self::T];
// }
//
// impl SelectAt for dyn Iterator<Item = Self::T> {
//     type T = Iterator::Item;
//
//     fn select_at(&mut self, indices: &[usize]) -> &[Self::T] {
//         let mut iter = self;
//         let mut total = 0;
//         let mut ret = vec![];
//         for i in indices {
//             let skip_here = i - 1 - total;
//             iter = self.skip(skip_here).into_iter();
//             ret.push(self.next().unwrap());
//             total += skip_here;
//         }
//
//         ret;
//     }
// }