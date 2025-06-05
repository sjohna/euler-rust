use std::collections::HashSet;

pub fn pow_cycle_lengths(modulus: i64) -> Vec<i64> {
    let mut ret = vec![0; modulus as usize];

    for base in 0..modulus {
        let mut curr = base * base;
        curr %= modulus;
        let mut length = 1;
        while curr != base {
            curr *= base;
            curr %= modulus;
            length += 1;
        }

        ret[base as usize] = length;
    }

    ret
}

pub fn pow_cycle(n: i64, modulus: i64) -> Vec<i64> {
    let mut seen = HashSet::new();
    let mut powers = Vec::new();

    let mut power = 1;

    while !seen.contains(&power) {
        powers.push(power);
        seen.insert(power);

        power *= n;
        power %= modulus;
    }

    powers
}

pub fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if exp == 0 {
        return 1;
    }

    base %= modulus;

    let mut powers = vec![];

    let mut curr_power = base;
    let mut curr_exp = 1;

    while curr_exp <= exp {
        powers.push(curr_power);
        curr_power *= curr_power;
        curr_power %= modulus;
        curr_exp <<= 1;
    }

    let powers = powers.as_slice();

    curr_exp >>= 1;

    let mut curr_exp_index = (powers.len() - 1) as i64;
    let mut curr = 1;
    while exp > 0 {
        if curr_exp <= exp {
            exp -= curr_exp;
            curr *= powers[curr_exp_index as usize];
            curr %= modulus;
        }
        curr_exp >>=1;
        curr_exp_index -= 1;
    }

    curr
}

#[cfg(test)]
mod tests {
    use crate::util::integer::{pow_cycle_lengths, pow_mod};

    #[test]
    fn test_pow_cycle_lengths() {
        assert_eq!(pow_cycle_lengths(1), vec![1]);
        assert_eq!(pow_cycle_lengths(10), vec![1,1,4,4,2,1,1,4,4,2]);
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(2, 0, 10), 1);
        assert_eq!(pow_mod(2, 1, 10), 2);
        assert_eq!(pow_mod(2, 2, 10), 4);
        assert_eq!(pow_mod(2, 3, 10), 8);
        assert_eq!(pow_mod(2, 4, 10), 6);
        assert_eq!(pow_mod(2, 5, 10), 2);
        assert_eq!(pow_mod(12, 4, 10), 6);
        assert_eq!(pow_mod(10, 9, 10), 0);
        assert_eq!(pow_mod(10, 1, 100), 10);
        assert_eq!(pow_mod(10, 2, 100), 0);
        assert_eq!(pow_mod(10, 5, 100), 0);
        assert_eq!(pow_mod(10, 123454321, 100), 0);

        // let's check Fermat's little theorem
        assert_eq!(pow_mod(12, 17, 17), 12);
        assert_eq!(pow_mod(12, 97, 97), 12);
        assert_eq!(pow_mod(456, 541, 541), 456);
    }
}