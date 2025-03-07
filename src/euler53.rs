use num_bigint::BigInt;
use crate::util::choose;    // why is this crate::util::, and not just util::, like the previous use?

pub fn euler53_bigint() -> i32{
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

pub fn euler53_nobigint() -> i32 {
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
    fn euler53_bigint() {
        assert_eq!(super::euler53_bigint(),4075);
    }

    #[test]
    fn euler53_nobigint() {
        assert_eq!(super::euler53_nobigint(),4075);
    }
}