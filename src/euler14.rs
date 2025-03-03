use std::collections::HashMap;

pub fn euler14_iter() -> i64 {
    let mut sequence_lengths = HashMap::<i64, i64>::new();

    sequence_lengths.insert(1, 1);

    (1..=1_000_000).map(|n| (n, sequence_length(&mut sequence_lengths, n))).reduce(|x,y| if x.1 > y.1 { x } else { y } ).unwrap().0
}

fn sequence_length(map: &mut HashMap::<i64, i64>, key: i64) -> i64 {
    if map.contains_key(&key) {
        return map[&key];
    }

    let length = 1 + sequence_length(map, next_value(key));

    map.insert(key, length);

    map[&key]
}

fn next_value(n: i64) -> i64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::euler14::euler14_iter;

    #[test]
    fn test_euler14_iter() {
        assert_eq!(euler14_iter(), 837799);
    }
}