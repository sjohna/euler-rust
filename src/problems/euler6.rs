use std::iter::once;

pub fn iters() -> i64 {
    let square = |n: i64| n*n;
    once((1..=100).sum()).map(square).last().unwrap() - (1..=100).map(square).sum::<i64>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn iters() {
        assert_eq!(super::iters(), 25164150);
    }
}