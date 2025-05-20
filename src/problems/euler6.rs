use std::iter::once;

pub fn iters() -> i64 {
    once((1..=100).sum()).map(|n: i64| n*n).last().unwrap() - (1..=100).map(|n: i64| n*n).sum::<i64>()  // not sure why I need this last type annotation on sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn iters() {
        assert_eq!(super::iters(), 25164150);
    }
}