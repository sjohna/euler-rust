use std::cmp::Reverse;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use crate::util::prime;

pub fn priority_queue() -> i32 {
    #[derive(Eq, Hash, PartialEq)]
    struct PrimeStuff {
        square: i64,
        cube: i64,
        fourth: i64,
    }

    let mut next_prime_map = HashMap::<i64,i64>::new();
    let mut prime_iter = prime::iter::naive_trial_division();

    let first_prime = prime_iter.next().unwrap();

    let mut pq = PriorityQueue::<PrimeStuff, Reverse<i64>>::new();
    pq.push(PrimeStuff {square: first_prime, cube: first_prime, fourth: first_prime}, Reverse(i64::pow(first_prime,2) + i64::pow(first_prime,3) + i64::pow(first_prime,4)));

    let mut total = 0;

    // need to not double count numbers than can be reached in multiple ways
    let mut last_product = Reverse(-1);

    while !pq.is_empty() {
        let (ps, product) = pq.pop().unwrap();
        if product.0 > 50_000_000 {
            break
        }

        if last_product != product {
            total += 1;
            last_product = product;
        }


        let next_square = *next_prime_map.entry(ps.square).or_insert_with(|| prime_iter.next().unwrap());
        let next_cube = *next_prime_map.entry(ps.cube).or_insert_with(|| prime_iter.next().unwrap());
        let next_fourth = *next_prime_map.entry(ps.fourth).or_insert_with(|| prime_iter.next().unwrap());

        pq.push(PrimeStuff{ square: next_square, cube: ps.cube, fourth: ps.fourth}, Reverse(i64::pow(next_square,2) + i64::pow(ps.cube,3) + i64::pow(ps.fourth,4)));
        pq.push(PrimeStuff{ square: ps.square, cube: next_cube, fourth: ps.fourth}, Reverse(i64::pow(ps.square,2) + i64::pow(next_cube,3) + i64::pow(ps.fourth,4)));
        pq.push(PrimeStuff{ square: ps.square, cube: ps.cube, fourth: next_fourth}, Reverse(i64::pow(ps.square,2) + i64::pow(ps.cube,3) + i64::pow(next_fourth,4)));
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn priority_queue() {
        assert_eq!(super::priority_queue(), 1097343)
    }
}