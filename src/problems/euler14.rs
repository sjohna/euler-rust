use std::collections::HashMap;

pub fn iter() -> i64 {
    let mut sequence_lengths = HashMap::<i64, i64>::new();

    sequence_lengths.insert(1, 1);

    let sequence_length = |n: i64| sequence_length(&mut sequence_lengths, n);

    (1..=1_000_000_i64).map(on_enumeration(sequence_length)).max_by_key(|x| x.1).unwrap().0
}

fn on_enumeration<T: Clone,R>(mut func: impl FnMut(T) -> R) -> impl FnMut(T) -> (T,R) {
    move |t: T| {
        (t.clone(), func(t))
    }
}

fn sequence_length(cache: &mut HashMap::<i64, i64>, n: i64) -> i64 {
    if cache.contains_key(&n) {
        return cache[&n];
    }

    let length = 1 + sequence_length(cache, next_value(n));

    cache.insert(n, length);

    cache[&n]
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
    #[test]
    fn iter() {
        assert_eq!(super::iter(), 837799);
    }
}