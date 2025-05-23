use std::cmp::Reverse;
use priority_queue::PriorityQueue;
use crate::util::{integer, next_list};

// slow, there's probably a faster way to do this
pub fn priority_queue() -> i64 {
    let mut pq = PriorityQueue::<(i64,i64), Reverse<i64>>::new();

    let pentagonal_iter = (1..).map(integer::pentagonal_number);
    let mut next_pentagonal = next_list::NextList::from_iter(Box::new(pentagonal_iter));

    pq.push((1, 1), Reverse(0));

    while !pq.is_empty() {
        let (pent_pair, diff_rev) = pq.pop().unwrap();
        let diff = diff_rev.0;

        let sum = pent_pair.0 + pent_pair.1;

        next_pentagonal.advance_to(sum);

        if next_pentagonal.contains_key(diff) && next_pentagonal.contains_key(sum) {
            return diff
        }

        let next = next_pentagonal.next(pent_pair.1).unwrap();

        pq.push((pent_pair.0, next), Reverse(next - pent_pair.0));

        if pent_pair.1 == next_pentagonal.next(pent_pair.0).unwrap() {
            pq.push((pent_pair.1, next), Reverse(next - pent_pair.1));
        }
    }

    // shouldn't happen
    -1
}