use std::ops::{Div, Mul};

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

pub fn triangular_number(n: i64) -> i64 {
    (n * (n + 1)) / 2
}

pub fn pentagonal_number(n: i64) -> i64 {
    (n * (3*n - 1)) / 2
}

pub fn is_palindrome<T: ToString>(n: T) -> bool {
    let s = n.to_string();
    s.chars().rev().collect::<String>() == s
}

pub fn factorial<T: Mul<Output = T> + From<i64>>(n: i64) -> T {
    let mut fac = T::from(1);
    for i in 1_i64..=n {
        fac = fac * T::from(i);
    }

    fac
}

pub fn choose<T: Mul<Output = T> + Div<Output = T> + From<i64>>(n: i64, r: i64) -> T {
    factorial::<T>(n) / (factorial::<T>(r) * factorial::<T>(n-r))
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcd() {
        assert_eq!(super::gcd(1, 1), 1);
        assert_eq!(super::gcd(2, 1), 1);
        assert_eq!(super::gcd(1, 2), 1);
        assert_eq!(super::gcd(2, 3), 1);
        assert_eq!(super::gcd(2, 2), 2);
        assert_eq!(super::gcd(4, 6), 2);
        assert_eq!(super::gcd(5, 7), 1);
        assert_eq!(super::gcd(6, 35), 1);
        assert_eq!(super::gcd(8, 12), 4);
        assert_eq!(super::gcd(24, 18), 6);
        assert_eq!(super::gcd(2 * 2 * 3 * 3 * 5 * 7 * 11, 2 * 3 * 3 * 3 * 7 * 7 * 13), 2 * 3 * 3 * 7);
    }

    #[test]
    fn lcm() {
        assert_eq!(super::lcm(1, 1), 1);
        assert_eq!(super::lcm(2, 1), 2);
        assert_eq!(super::lcm(1, 2), 2);
        assert_eq!(super::lcm(2, 3), 6);
        assert_eq!(super::lcm(4, 6), 12);
        assert_eq!(super::lcm(5, 7), 35);
        assert_eq!(super::lcm(8, 12), 24);
        assert_eq!(super::lcm(2 * 2 * 3 * 5 * 11, 2 * 2 * 3 * 3 * 5 * 7 * 11 * 13), 2 * 2 * 3 * 3 * 5 * 7 * 11 * 13);
    }

    #[test]
    fn triangular_number() {
        assert_eq!(super::triangular_number(1), 1);
        assert_eq!(super::triangular_number(2), 3);
        assert_eq!(super::triangular_number(3), 6);
        assert_eq!(super::triangular_number(4), 10);
        assert_eq!(super::triangular_number(5), 15);
        assert_eq!(super::triangular_number(6), 21);
        assert_eq!(super::triangular_number(7), 28);
        assert_eq!(super::triangular_number(8), 36);
        assert_eq!(super::triangular_number(9), 45);
        assert_eq!(super::triangular_number(10), 55);
        assert_eq!(super::triangular_number(100), 5050);
        assert_eq!(super::triangular_number(1000), 500500);
    }

    #[test]
    fn pentagonal_number() {
        assert_eq!(super::pentagonal_number(1), 1);
        assert_eq!(super::pentagonal_number(2), 5);
        assert_eq!(super::pentagonal_number(3), 12);
        assert_eq!(super::pentagonal_number(4), 22);
        assert_eq!(super::pentagonal_number(5), 35);
        assert_eq!(super::pentagonal_number(6), 51);
        assert_eq!(super::pentagonal_number(7), 70);
        assert_eq!(super::pentagonal_number(8), 92);
        assert_eq!(super::pentagonal_number(9), 117);
        assert_eq!(super::pentagonal_number(10), 145);
    }
}