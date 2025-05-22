use std::fmt::Debug;
use std::iter;
use std::ops::Add;
use num_bigint::{BigInt, ToBigInt};

pub fn brute_force(max: i64) -> i64 {
    let mut dx = (0_i64, 0.to_bigint().unwrap());
    for d in 1..=max {
        if i64::isqrt(d) * i64::isqrt(d) == d {
            continue
        }

        let roots = square_roots_of_unity(d);
        let deltas = delta_mod(&roots, d).iter().map(|&x| x.to_bigint().unwrap()).collect::<Vec::<BigInt>>();
        let mut check_sequence = iter::from_fn(skip_sequence(roots[0].to_bigint().unwrap(), &deltas));

        let x = check_sequence.find(|x: &BigInt| {
            let y_squared = (x*x - 1) / d;
            let y = BigInt::sqrt(&y_squared);
            if y <= 0.to_bigint().unwrap() {
                return false
            }
            //println!("? {}^2 - {} * {} = 1", x, d, y_squared);
            if x*x - d*&y*&y == 1.to_bigint().unwrap() {
                //println!("! {}^2 - {} * {}^2 = 1", x, d, y);

                true
            } else {
                false
            }
        }).unwrap();

        println!("{:?}", (d, &x));
        if x > dx.1 {
            dx = (d,x);
        }
    }

    dx.0
}

fn square_roots_of_unity(modulus: i64) -> Vec<i64> {
    let mut ret = Vec::<i64>::new();

    for n in 1..modulus {
        if n*n % modulus == 1 {
            ret.push(n);
        }
    }

    ret
}

fn delta_mod(values: &[i64], modulus: i64) -> Vec<i64> {
    let mut ret = vec![];

    for i in 1..values.len() {
        ret.push(values[i] - values[i-1]);
    }

    ret.push(values[0]+modulus - values[values.len()-1]);

    ret
}

fn skip_sequence<T: Add<Output = T> + Clone + Debug>(start: T, skips: &[T]) -> impl FnMut() -> Option<T> {
    let mut next = start.clone();
    let mut next_index = 0;
    move || -> Option<T> {
        let curr = next.clone();
        next = next.clone() + skips[next_index].clone();
        next_index = (next_index + 1) % skips.len();
        Some(curr.clone())
    }
}