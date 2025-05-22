use num_bigint::BigInt;
use crate::util::choose;

pub fn bigint() -> i32{
    let mut total = 0;
    let threshold = BigInt::from(1_000_000);

    for n in 1..=100 {
        for r in 0..=n {
            let val = choose(n,r);
            if val > threshold {
                total += 1;
            }
        }
    }

    total
}

pub fn nobigint() -> i32 {
    let mut total = 0;

    for n in 1..=100 {
        let mut choose = 1;
        for r in 1..=n/2 {  // go to half, rounded up
            choose *= n-r+1;
            choose /= r;
            if choose > 1_000_000 {
                total += n - 2*r + 1;   // want to get a better intuitive sense of this: had to get this right by trial and error
                break;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn bigint() {
        assert_eq!(super::bigint(), 4075);
    }

    #[test]
    fn nobigint() {
        assert_eq!(super::nobigint(), 4075);
    }
}