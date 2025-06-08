use crate::util::prime;
use crate::util::prime::totient;
use crate::util::integer::IntegerExt;

pub fn iter(n: i64) -> i64 {
    let primes = prime::sieve_up_to(n);
    let totients = totient::sieve_up_to(n, &primes);

    totients.iter().enumerate().skip(2).filter(|(n, phi)| n.sorted_digit_string() == phi.sorted_digit_string())
        .map(|(n, phi)| (n, n as f64 / *phi as f64)).min_by(|a, b| f64::total_cmp(&a.1,&b.1)).unwrap().0 as i64
}