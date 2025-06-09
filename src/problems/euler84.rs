use std::collections::HashSet;
use rand::Rng;
use crate::util::integer::ModInt;

pub fn simulate(die_size: i64) {
    let cc_cards = [0, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    let ch_cards = [0, 10, 11, 24, 39, 5, 100, 100, 200, -3, -1, -1, -1, -1, -1, -1]; // 100: next railroad, 200: next utility, -3: back 3

    let mut rng = rand::rng();

    let mut square_counts = vec![0_i64; 40];
    let mut total = 0;

    let mut curr_square = ModInt::new(0, 40);

    let mut double_count = 0;

    loop {
        square_counts[curr_square.usize()] += 1_i64;
        total += 1;

        if total % 1_000_000 == 0 {
            println!();
            for sc in square_counts.iter().enumerate() {
                println!("{:>2}: {}", sc.0, sc.1);
            }
            println!("------------------------------");
            println!("{}", total);

            let mut sortable = square_counts.iter().enumerate().collect::<Vec<(usize,&i64)>>();
            sortable.sort_by_key(|p| p.1);

            println!("{:0>2}{:0>2}{:0>2}", sortable[39].0, sortable[38].0, sortable[37].0);
        }

        let d1 = rng.random_range(1..=die_size);
        let d2 = rng.random_range(1..=die_size);
        if d1 == d2 {
            double_count += 1;
            if double_count == 3 {
                double_count = 0;
                curr_square.assign(10);
                continue;
            }
        } else {
            double_count = 0;
        }

        let advance = d1 + d2;

        curr_square += advance;

        // special squares
        if curr_square == 2 || curr_square == 17 || curr_square == 33 {
            // community chest
            let cc_card = cc_cards[rng.random_range(0..16)];

            if cc_card != -1 {
                curr_square.assign(cc_card);
            }
        } else if curr_square == 7 || curr_square == 22 || curr_square == 36 {
            // chance
            let ch_card = ch_cards[rng.random_range(0..16)];

            if ch_card == -3 {
                curr_square -= 3;
            } else if ch_card == 100 {
                while (curr_square % 10).value() != 5 {
                    curr_square += 1;
                }
            } else if ch_card == 200 {
                while curr_square != 12 && curr_square != 28 {
                    curr_square += 1;
                }
            } else if ch_card != -1 {
                curr_square.assign(ch_card);
            }
        } else if curr_square == 30 {
            curr_square.assign(10);
        }
    }
}