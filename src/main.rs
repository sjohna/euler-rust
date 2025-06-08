#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use problems::euler69;

mod util;
mod problems;

fn main() {
    println!("{:?}", problems::euler70::iter(10_000_000));
}