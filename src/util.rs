
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if b < a {
        (b,a) = (a,b);
    }

    while a != 0 {
        (a,b) = (b%a,a);
    }

    b
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

pub fn fibonacci_sequence() -> impl FnMut() -> Option<i64> {    // need the impl keyword here, or I get errors. What does that do? What are the other options here?
    let (mut curr, mut next): (i64, i64) = (0,1);
    move || -> Option<i64> {
        (curr, next) = (next, curr + next);
        Some(curr)
    }
}

pub fn primes() -> impl FnMut() -> Option<i64> {
    let mut next_candidate: i64 = 2;
    move || -> Option<i64> {
        if next_candidate == 2 {
            next_candidate = 3;
            return Some(2)
        }

        loop {
            let max = i64::isqrt(next_candidate);
            let mut factor_found = false;
            for factor in std::iter::once(2).chain((3..=max).step_by(2)) {
                if next_candidate % factor == 0 {
                    factor_found = true;
                    break;
                }
            }

            if !factor_found {
                let next_prime = next_candidate;
                next_candidate += 2;
                return Some(next_prime);
            }

            next_candidate += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcd() {
        assert_eq!(super::gcd(1,1), 1);
        assert_eq!(super::gcd(2,1), 1);
        assert_eq!(super::gcd(1,2), 1);
        assert_eq!(super::gcd(2,3), 1);
        assert_eq!(super::gcd(2,2), 2);
        assert_eq!(super::gcd(4,6), 2);
        assert_eq!(super::gcd(5,7), 1);
        assert_eq!(super::gcd(6,35), 1);
        assert_eq!(super::gcd(8,12), 4);
        assert_eq!(super::gcd(24,18), 6);
        assert_eq!(super::gcd(2*2*3*3*5*7*11,2*3*3*3*7*7*13), 2*3*3*7);
    }

    #[test]
    fn lcm() {
        assert_eq!(super::lcm(1,1), 1);
        assert_eq!(super::lcm(2,1), 2);
        assert_eq!(super::lcm(1,2), 2);
        assert_eq!(super::lcm(2,3), 6);
        assert_eq!(super::lcm(4,6), 12);
        assert_eq!(super::lcm(5,7), 35);
        assert_eq!(super::lcm(8,12), 24);
        assert_eq!(super::lcm(2*2*3*5*11,2*2*3*3*5*7*11*13), 2*2*3*3*5*7*11*13);
    }
}