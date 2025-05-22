#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod util;
mod problems;

fn main() {
    util::table(&util::pow_seq::<i64>(10,1,10), |n: i64| { n })
}