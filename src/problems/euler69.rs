use crate::util::prime;
use crate::util::prime::totient;

pub fn sieve(n: i64) -> i64 {
    let primes = prime::seq::sieve_up_to(n).collect::<Vec<_>>();

    let totients = totient::sieve_up_to(n, &primes);

    totients.iter().enumerate().skip(1).map(|(i,v)| (i, i as f64 / *v as f64)).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0 as i64
}