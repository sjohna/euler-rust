use std::collections::HashSet;

pub fn naive_loop() -> i32 {
    let mut desired_set = HashSet::<char>::new();
    for c in "123456789".chars() {
        desired_set.insert(c);
    }
    let desired_set = desired_set;

    let mut max_pandigital = -1;

    for n in 1..9999 {
        let mut num_str = String::new();

        for i in 1..=9 {
            num_str.push_str(&(n*i).to_string());
            if num_str.len() == 9 {
                match to_hash_set(&num_str) {   // pattern matching is neat
                    None => {}  // do I need an explicit case here?
                    Some(set) => {
                        if set == desired_set {
                            let num = str::parse::<i32>(&num_str).unwrap();
                            if num > max_pandigital {
                                max_pandigital = num;
                            }
                        }
                    }
                }
            }
        }
    }

    max_pandigital
}

fn to_hash_set(s: &str) -> Option<HashSet<char>> {  // this doesn't have a :: before the type after HashSet, what gives?
    let mut set = HashSet::<char>::new();

    for c in s.chars() {
        if set.contains(&c) {   // why does this need a &?
            return None;
        }

        set.insert(c);
    }

    Some(set)
}

#[cfg(test)]
mod tests {
    #[test]
    fn naive_loop() {
        assert_eq!(super::naive_loop(), 932718654);
    }
}