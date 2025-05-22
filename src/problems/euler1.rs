use crate::util;

type Impl = fn(i64) -> i64;

pub fn brute_force(n: i64) -> i64 {
    (1..n).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

fn natural_numbers() -> impl FnMut() -> Option<i64> {
    let mut num = 0;
    move || -> Option<i64> {
        num += 1;
        Some(num)
    }
}

fn closed_form(n: i64) -> i64 {
    let max = n-1;
    3*util::triangular_number(max/3) + 5*util::triangular_number(max/5) - 15*util::triangular_number(max/15)
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