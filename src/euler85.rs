use std::cmp::Reverse;
use priority_queue::PriorityQueue;
use crate::util::triangular_number;

pub fn euler85_priority_queue() -> i64 {
    let mut pq = PriorityQueue::<Rectangle, Reverse<i64>>::new();

    pq.push(Rectangle{width: 1, height: 1}, Reverse(1));

    let mut closest_count_below: i64 = 0;
    let mut closest_area_below: i64 = 0;

    while !pq.is_empty() {
        let (rect, rev_count) = pq.pop().unwrap();
        let count = rev_count.0;

        if count > 2_000_000 {
            let above_distance = count - 2_000_000;
            let below_distance = 2_000_000 - closest_count_below;

            return if above_distance < below_distance {
                rect.area()
            } else {
                closest_area_below
            }
        }

        if count > closest_count_below {
            closest_count_below = count;
            closest_area_below =  rect.area();
        }

        let mut next_wider = rect.clone();
        next_wider.width += 1;
        pq.push(next_wider.clone(), Reverse(next_wider.sub_rectangles()));

        if rect.width == rect.height {
            let next_bigger = Rectangle{width: rect.width + 1, height: rect.height + 1};
            pq.push(next_bigger.clone(), Reverse(next_bigger.sub_rectangles()));
        }
    }

    -1
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Rectangle {
    width: i64,
    height: i64,
}

impl Rectangle {
    fn sub_rectangles(&self) -> i64 {
        triangular_number(self.width) * triangular_number(self.height)
    }

    fn area(&self) -> i64 {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler85_priority_queue() {
        assert_eq!(super::euler85_priority_queue(), 2772)
    }
}