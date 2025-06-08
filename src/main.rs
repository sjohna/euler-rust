#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod util;
mod problems;

fn main() {
    println!("{:?}",util::time(|| 3 + 2))
}