#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use problems::euler69;

mod util;
mod problems;

fn main() {
    println!("{:?}", problems::euler84::simulate(4));
}