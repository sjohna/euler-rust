use priority_queue::PriorityQueue;

pub fn loops() -> i32 {
    let mut max_palindrome = 0;
    for a in 100..1000 {
        for b in a..1000 {
            let product = a * b;
            if is_palindrome(product) && product > max_palindrome {
                max_palindrome = product;
            }
        }
    }

    max_palindrome
}

pub fn priority_queue() -> i32 {
    #[derive(Eq, Hash, PartialEq)]
    struct Multiplicands {
        a: i32,
        b: i32,
    }

    let mut pq = PriorityQueue::<Multiplicands, i32>::new();

    pq.push(Multiplicands { a: 999, b: 999 }, 999*999);
    while !pq.is_empty() {
        let (mults, product) = pq.pop().unwrap();
        if is_palindrome(product) {
            return product
        }

        pq.push(Multiplicands { a: mults.a, b: mults.b-1 }, mults.a * (mults.b-1));
        if mults.a == mults.b {
            pq.push(Multiplicands { a: mults.a-1, b: mults.b-1 }, (mults.a-1) * (mults.b-1));
        }
    }

    // shouldn't happen...
    -1
}

fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s.chars().rev().collect::<String>() == s
}

#[cfg(test)]
mod tests {
    #[test]
    fn loops() {
        assert_eq!(super::loops(), 906609);
    }

    #[test]
    fn priority_queue() {
        assert_eq!(super::priority_queue(), 906609)
    }
}