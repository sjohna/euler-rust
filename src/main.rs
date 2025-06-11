#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use problems::euler69;
use crate::util::continued_fraction;

mod util;
mod problems;

fn main() {
    println!("{:?}", continued_fraction::for_ratio(1234543,187132491));
}