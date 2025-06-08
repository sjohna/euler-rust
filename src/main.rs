#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod util;
mod problems;

fn main() {
    util::list(&[1,2,3,4,5,6,7,8,9,10], util::integer::factorial::<i64>);
    println!();

    util::table((1..=20).collect::<Vec<_>>().as_slice(), &(1..=20).collect::<Vec<_>>().as_slice(), util::integer::lcm)
}