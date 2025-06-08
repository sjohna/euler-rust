use std::ops::Add;

pub fn fibonacci() -> impl Iterator<Item = i64> {
    let (mut curr, mut next): (i64, i64) = (0,1);

    std::iter::from_fn(    move || -> Option<i64> {
        (curr, next) = (next, curr + next);
        Some(curr)
    })
}

pub fn partial_sums<T: Add<Output = T> + Default + Copy>() -> impl FnMut(T) -> T {
    let mut sum = T::default();
    move |v| {
        sum = sum + v;
        sum
    }
}