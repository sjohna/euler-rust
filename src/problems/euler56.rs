use num_bigint::{BigInt, ToBigInt};

pub fn loops() -> i64 {
    let mut max_sum = 0;
    for a in 1..=100 {
        for b in 1..=100 {
            let a_big = a.to_bigint().unwrap();
            let pow = a_big.pow(b);
            let digit_sum = digit_sum(pow);

            if digit_sum > max_sum {
                max_sum = digit_sum;
            }
        }
    }

    max_sum
}

fn digit_sum(n: BigInt) -> i64 {
    let num_str = n.to_string();
    let mut sum = 0;
    for c in num_str.chars() {
        sum += c as i64 - '0' as i64;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler56_loops() {
        assert_eq!(super::loops(), 972);
    }
}