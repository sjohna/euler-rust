#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use problems::euler69;

mod util;
mod problems;

fn main() {
    println!("{:?}", euler69::sieve(1_000_000));
}