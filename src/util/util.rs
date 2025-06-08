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
            let candidate = &root + next_digit;

            let candidate_squared = &candidate * &candidate;

            if candidate_squared > target {
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
    let str = n.to_string();
    let mut sum = 0;
    for digit in str.chars() {
        sum += digit as i32 - '0' as i32;
    }

    sum
}

// TODO: can I make this take any iterable thing?
// TODO: function or closure
pub fn list<T: ToString + Copy,V: std::fmt::Display>(inputs: &[T], func: fn(T) -> V) {
    let mut input_strings = Vec::new();
    for input in inputs {
        input_strings.push(input.to_string());
    }

    let max_length = input_strings.iter().map(|s| s.len()).max().unwrap();

    for i in 0..inputs.len() {
        print!("{:<max_length$} | ", input_strings[i]);
        let (output, elapsed) = time(|| func(inputs[i]));
        print!("{} ({:?})\n", output, elapsed);
    }
}

// TODO: function or closure
pub fn table<T1: ToString + Copy, T2: ToString + Copy, V: ToString>(rows: &[T1], cols: &[T2], func: fn(T1, T2) -> V) {
    let mut table = Vec::new();

    let mut headers = Vec::new();
    headers.push("".to_string());
    for c in cols {
        headers.push(c.to_string())
    }

    table.push(headers);

    for r in rows {
        let mut row = Vec::new();
        row .push(r.to_string());
        for c in cols {
            row.push(func(*r, *c).to_string());
        }

        table.push(row);
    }

    let mut col_widths = vec![0; cols.len()+1];

    for c_index in 0..cols.len()+1 {
        for r_index in 0..rows.len()+1 {
            if table[r_index][c_index].len() > col_widths[c_index] {
                col_widths[c_index] = table[r_index][c_index].len();
            }
        }
    }

    for r_index in 0..rows.len()+1 {
        for c_index in 0..cols.len()+1 {
            if c_index > 0 {
                print!("  ");
            }
            let width = col_widths[c_index];
            print!("{:>width$}", table[r_index][c_index]);
        }

        println!();
    }
}

pub fn time_of<F, V>(mut func: F) -> impl FnMut() -> (V, Duration) where F: FnMut() -> V {
    move || {
        let start = SystemTime::now();
        let output = func();
        let elapsed = start.elapsed().unwrap();
        (output, elapsed)
    }
}

pub fn time<F,V>(mut func: F) -> (V, Duration) where F: FnMut() -> V {
    let start = SystemTime::now();
    let output = func();
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

pub fn power_set<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let mut powerset = Vec::<Vec<T>>::new();
    powerset.push(vec![]);

    for item in items {
        let mut new_powerset = powerset.clone();
        for set in powerset.iter() {
            let mut new_set = set.clone();
            new_set.push(item.clone());
            new_powerset.push(new_set);
        }

        powerset = new_powerset;
    }

    powerset
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