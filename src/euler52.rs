use std::collections::HashMap;

pub fn euler52_naive_loop() -> i64 {

    for magnitude in 5.. {  // need 6 digits
        let base = 10i64.pow(magnitude);
        for num in base.. {
            let mut multiples = Vec::<i64>::new();
            for i in 1..=6 {
                multiples.push(i * num);
            }

            if multiples[5] > base * 10 {   // too long
                break;
            }

            let digits = to_hash_map(&multiples[0].to_string());

            if multiples.into_iter().all(|m| to_hash_map(&m.to_string()) == digits) {
                return num;
            }
        }
    }

    // shouldn't happen
    -1
}

fn to_hash_map(s: &str) -> HashMap<char, i32> {
    let mut ret = HashMap::<char, i32>::new();
    for c in s.chars() {
        *ret.entry(c).or_default() += 1;    // seems like a more idiomatic way to set this value.
    }

    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler52_naive_loop() {
        assert_eq!(super::euler52_naive_loop(), 142857);
    }
}