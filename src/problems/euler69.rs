use crate::util::prime;
use crate::util::prime::totient;

pub fn sieve(n: i64) -> i64 {
    let primes = prime::sieve_up_to(n);

    let totients = totient::sieve_up_to(n, &primes);

    totients.iter().enumerate().skip(1).map(|(i,v)| (i, i as f64 / *v as f64)).max_by(|a, b| f64::total_cmp(&a.1, &b.1)).unwrap().0 as i64
}