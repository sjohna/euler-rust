use crate::util::integer;

type Impl = fn(i64) -> i64;

pub const canonical: Impl = closed_form;

pub fn brute_force(n: i64) -> i64 {
    (1..n).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn closed_form(n: i64) -> i64 {
    let max = n-1;
    3*integer::triangular_number(max/3) + 5*integer::triangular_number(max/5) - 15*integer::triangular_number(max/15)
}

#[cfg(test)]
mod tests {
    const TESTS: [(i64, i64); 1] = [
        (1000,233168),
    ];

    fn test(func: super::Impl) {
        for tc in TESTS {
            assert_eq!(func(tc.0), tc.1);
        }
    }

    #[test]
    fn brute_force() {
        test(super::brute_force);
    }

    #[test]
    fn closed_form() {
        test(super::closed_form);
    }
}