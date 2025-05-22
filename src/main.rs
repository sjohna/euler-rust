#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod util;
mod problems;

fn main() {
    println!("{}",problems::euler66::brute_force(1000))
}