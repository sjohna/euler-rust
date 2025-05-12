use std::cmp::Reverse;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use crate::util::pentagonal_number;

// slow, there's probably a faster way to do this
pub fn euler44_priority_queue() -> i64 {
    let mut pq = PriorityQueue::<(i64,i64), Reverse<i64>>::new();
    let mut next_pentagonal = HashMap::<i64,i64>::new();

    let mut pentagonal_iter = (1..).map(pentagonal_number);

    let first_pentagonal  = pentagonal_iter.next().unwrap();
    let mut last_pentagonal = pentagonal_iter.next().unwrap();

    next_pentagonal.insert(first_pentagonal, last_pentagonal);

    pq.push((1, 1), Reverse(0));

    while !pq.is_empty() {
        let (pent_pair, diff_rev) = pq.pop().unwrap();
        let diff = diff_rev.0;

        let sum = pent_pair.0 + pent_pair.1;

        while sum > last_pentagonal {
            let next_num = pentagonal_iter.next().unwrap();
            next_pentagonal.insert(last_pentagonal, next_num);
            last_pentagonal = next_num;
        }

        if next_pentagonal.contains_key(&diff) && next_pentagonal.contains_key(&sum) {  // why do I need &s here?
            return diff
        }

        next_pentagonal.entry(pent_pair.1).or_insert_with(|| pentagonal_iter.next().unwrap());

        pq.push((pent_pair.0, next_pentagonal[&pent_pair.1]), Reverse(next_pentagonal[&pent_pair.1] - pent_pair.0));

        if pent_pair.1 == next_pentagonal[&pent_pair.0] {
            pq.push((pent_pair.1, next_pentagonal[&pent_pair.1]), Reverse(next_pentagonal[&pent_pair.1] - pent_pair.1));
        }
    }

    // shouldn't happen
    -1
}