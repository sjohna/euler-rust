use std::collections::HashSet;

// TODO: maybe a type for these
pub fn for_ratio(mut a: i64, mut b: i64) -> Vec<i64> {
    let mut ret = vec![];

    while b != 0 {
        ret.push(a/b);
        (a,b) = (b,a%b)
    }

    ret
}