use std::collections::HashMap;
use std::hash::Hash;

struct NextList<T: Eq + Hash + Ord> {
    first: Option<T>,
    max: Option<T>,
    next: HashMap<T,T>,
    iter: Box<dyn Iterator<Item = T>>,
    done: bool,
}

impl<T: Copy + Eq + Hash + Ord> NextList<T> {
    pub fn from_iter(mut iter: Box<dyn Iterator<Item = T>>) -> NextList<T> {
        let f = iter.next();
        let done = f.is_none();
        NextList{
            first: f,
            max: f,
            next: HashMap::new(),
            iter,
            done,
        }
    }

    pub fn first(&self) -> Option<T> {
        self.first
    }

    pub fn max(&self) -> Option<T> {
        self.max
    }

    pub fn next_of(&mut self, index: T) -> Option<T> {
        if self.max == None {
            return None
        }

        while index >= self.max.unwrap() && !self.done {
            let next_item = self.iter.next();
            match next_item {
                None => {
                    self.done = true;
                    return None
                }
                Some(next_item) => {
                    self.next.insert(self.max.unwrap(), next_item);
                    self.max = Some(next_item);
                }
            }
        }

        self.next.get(&index).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::prime;
    use crate::util::prime::next_list::NextList;

    #[test]
    fn basic() {
        let mut list = NextList::from_iter(Box::new(1..=5));
        assert_eq!(list.first(), Some(1));
        assert_eq!(list.max(), Some(1));
        assert_eq!(list.next_of(1), Some(2));
        assert_eq!(list.max(), Some(2));
        assert_eq!(list.next_of(2), Some(3));
        assert_eq!(list.max(), Some(3));
        assert_eq!(list.next_of(3), Some(4));
        assert_eq!(list.max(), Some(4));
        assert_eq!(list.next_of(5), None);
        assert_eq!(list.max(), Some(5));
    }

    #[test]
    fn gaps() {
        let mut list = NextList::from_iter(Box::new((1..=10).step_by(2)));
        assert_eq!(list.first(), Some(1));
        assert_eq!(list.max(), Some(1));
        assert_eq!(list.next_of(7), Some(9));
        assert_eq!(list.max(), Some(9));
        assert_eq!(list.next_of(8), None);
        assert_eq!(list.max(), Some(9));
        assert_eq!(list.next_of(100), None);
    }

    #[test]
    fn primes() {
        let mut list = NextList::from_iter(Box::new(prime::seq::naive_trial_division()));
        assert_eq!(list.max(), Some(2));
        assert_eq!(list.next_of(2), Some(3));
        assert_eq!(list.next_of(3), Some(5));
        assert_eq!(list.next_of(5), Some(7));
        assert_eq!(list.next_of(7), Some(11));
        assert_eq!(list.next_of(9109), Some(9127));
        assert_eq!(list.next_of(104658), None);
        assert_eq!(list.next_of(104659), Some(104677));
    }
}