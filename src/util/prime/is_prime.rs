type Impl = fn(i64) -> bool;

pub fn naive_trial_division(num: i64) -> bool {
    if num == 2 {
        return true;
    }
    let max = i64::isqrt(num);
    let mut factor_found = false;
    for factor in std::iter::once(2).chain((3..=max).step_by(2)) {
        if num % factor == 0 {
            factor_found = true;
            break;
        }
    }

    !factor_found
}

#[cfg(test)]
mod tests {
    const TESTS: [(i64,bool); 10] = [
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (6, false),
        (7, true),
        (8, false),
        (9, false),
        (10, false),
        (11, true),
    ];

    fn test(func: super::Impl) {
        for tc in TESTS {
            assert_eq!(func(tc.0), tc.1)
        }
    }

    #[test]
    fn naive_trial_division() {
        test(super::naive_trial_division);
    }
}