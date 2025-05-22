#![allow(dead_code)]

mod util;
mod problems;

fn main() {
    println!("{}", problems::euler1::brute_force(1000));
}