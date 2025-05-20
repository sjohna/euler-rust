use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub fn euler54() -> i32 {
    let path = Path::new("../../files/0054_poker.txt");
    let file = match File::open(path) {    // why is mut not needed here?
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut player_one_wins = 0;

    for line in std::io::BufReader::new(file).lines() {
        let hands = match dbg!(line) {
            Ok(string) => string,
            Err(why) => panic!("Couldn't read line from {}: {}", path.display(), why),
        };

        let cards = hands.split(' ').collect::<Vec<&str>>();
        if dbg!(hand_score(&cards[0..=4])) > dbg!(hand_score(&cards[5..=9])) {  // this is some heinous nonsense I need to figure out
            player_one_wins += 1;
        }
    }

    player_one_wins
}

fn hand_score(hand: &[&str]) -> u64 {
    // extract ranks and suits
    let mut ranks = Vec::<u64>::new();
    let mut rank_map = HashMap::<u64, u64>::new();
    let mut suits = HashSet::<char>::new();
    for card in hand {
        let card_chars = card.chars().collect::<Vec<char>>();

        let rank = match dbg!(card_chars[0]) {
            '2'..='9' => card_chars[0] as u64 - '0' as u64,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        };
        ranks.push(dbg!(rank));
        if rank_map.contains_key(&rank) {
            rank_map.insert(rank, rank_map[&rank]+1);
        } else {
            rank_map.insert(rank, 1);
        }
        suits.insert(card_chars[1]);
    }

    ranks.sort();

    let mut score = 0_u64;

    for (i, item) in ranks.iter().enumerate().take(5) {
        score |= item << (i*4);
    }

    let pair = 1 << 28;
    let two_pair = 1 << 29;
    let three_of_a_kind = 1 << 30;
    let straight = 1 << 31;
    let flush = 1 << 32;
    let full_house = 1 << 33;
    let four_of_a_kind = 1 << 34;
    let straight_flush = 1 << 35;

    // check for straights and flushes
    let contains_flush = suits.len() == 1;
    let mut contains_straight = false;

    // simple straights
    if ranks[1] - ranks[0] == 1 && ranks[2] - ranks[1] == 1 && ranks[3] - ranks[2] == 1 && ranks[4] - ranks[3] == 1 {
        contains_straight = true;
    }

    // check for waparound straights
    if ranks[0] == 2 && ranks[4] == 14 {
        contains_straight = (ranks[1] == 3 || ranks[1] == 11) && (ranks[2] == 4 || ranks[2] == 12) && (ranks[3] == 5 || ranks[3] == 13);
    }

    if contains_straight {
        score |= straight;
    }

    if contains_flush {
        score |= flush;
    }

    if contains_straight && contains_flush {
        score |= straight_flush;
    }

    // check other hands
    let mut pair_count = 0;
    let mut contains_three_of_a_kind = false;
    let mut contains_four_of_a_kind = false;
    let mut largest_group_rank = 0_u64;   // for distinguishing full houses, pairs, etc.
    let mut next_largest_group_rank = 0_u64;

    for (rank, count) in rank_map.iter() {
        if *count == 2 {    // why do I need a * here?
            pair_count += 1;
            if !contains_three_of_a_kind && !contains_four_of_a_kind {
                if largest_group_rank == 0 {
                    largest_group_rank = *rank;
                } else {
                    next_largest_group_rank = min(*rank, largest_group_rank);
                    largest_group_rank = max(*rank, largest_group_rank)
                }
            } else {
                next_largest_group_rank = *rank;
            }
        }

        if *count == 3 {
            contains_three_of_a_kind = true;
            next_largest_group_rank = largest_group_rank;
            largest_group_rank = *rank;
        }

        if *count == 4 {
            contains_four_of_a_kind = true;
            largest_group_rank = *rank;
        }
    }

    score |= largest_group_rank << 24;
    score |= next_largest_group_rank << 20;

    if pair_count > 0 {
        score |= pair;
    }

    if pair_count == 2 {
        score |= two_pair;
    }

    if contains_three_of_a_kind {
        score |= three_of_a_kind;
    }

    if contains_four_of_a_kind {
        score |= four_of_a_kind;
    }

    if pair_count > 0 && contains_three_of_a_kind {
        score |= full_house;
    }

    println!("{:#032b}", score);

    score
}

#[cfg(test)]
mod tests {
    use crate::problems::euler54::hand_score;

    #[test]
    fn hand_score_comparison() {
        assert!(hand_score(&["AS", "2C", "4D", "5S", "9C"]) > hand_score(&["TS", "2C", "4D", "5S", "9C"])); // high card comparison
        assert!(hand_score(&["AS", "2C", "4D", "5S", "9C"]) < hand_score(&["AS", "2C", "4D", "5S", "TC"])); // high card comparison, second-highest card
        assert!(hand_score(&["AS", "3C", "4D", "5S", "9C"]) > hand_score(&["AS", "2C", "4D", "5S", "9C"])); // high card comparison, lowest card is tiebreaker
        assert_eq!(hand_score(&["AS", "3C", "4D", "5S", "9C"]), hand_score(&["AH", "3H", "4H", "5D", "9S"])); // different suits, same value
        assert!(hand_score(&["TS", "TC", "4D", "5S", "9C"]) > hand_score(&["AS", "2C", "4D", "5S", "9C"])); // pair beats ace
        assert!(hand_score(&["TS", "TC", "4D", "5S", "9C"]) < hand_score(&["2S", "2C", "4D", "4S", "9C"])); // two pair beats pair
        assert!(hand_score(&["TS", "TC", "4D", "4S", "9C"]) > hand_score(&["2S", "2C", "4D", "4S", "9C"])); // higher two pair beats lower two pair
        assert!(hand_score(&["TS", "TC", "4D", "4S", "9C"]) > hand_score(&["2S", "2C", "10D", "10S", "9C"])); // higher two pair beats lower two pair, second pair tiebreaker
        assert!(hand_score(&["TS", "TC", "4D", "4S", "9C"]) < hand_score(&["2S", "10C", "10D", "10S", "9C"])); // three of a kind beats two pair
        assert!(hand_score(&["3S", "4C", "5D", "6S", "7C"]) > hand_score(&["2S", "10C", "10D", "10S", "9C"])); // straight beats three of a kind
        assert!(hand_score(&["3S", "4C", "5D", "6S", "7C"]) < hand_score(&["4S", "5C", "6D", "7S", "8C"])); // higher straight beats lower straight
        assert!(hand_score(&["AS", "2C", "KD", "QS", "JC"]) > hand_score(&["4S", "5C", "6D", "7S", "8C"])); // higher wrap-around straight beats lower straight
        assert!(hand_score(&["AS", "2C", "KD", "QS", "3C"]) < hand_score(&["AS", "2C", "KD", "QS", "JC"])); // higher wrap-around straight beats lower wrap-around straight
        assert!(hand_score(&["AH", "2H", "4H", "5H", "6H"]) > hand_score(&["AS", "2C", "KD", "QS", "JC"])); // flush beats straight
        assert!(hand_score(&["AH", "2H", "4H", "5H", "6H"]) < hand_score(&["2S", "2C", "2D", "3S", "3C"])); // full house beats flush
        assert!(hand_score(&["2H", "2H", "2H", "7D", "7D"]) > hand_score(&["2S", "2C", "2D", "3S", "3C"])); // full house with higher pair beats full house with lower pair
        assert!(hand_score(&["4H", "4H", "4H", "2D", "2C"]) > hand_score(&["3S", "3C", "3D", "2S", "2C"])); // full house with higher 3-of-a-kind beats full house with lower 3-of-a-kind
        assert!(hand_score(&["4H", "4H", "4H", "4D", "2C"]) > hand_score(&["3S", "3C", "3D", "2S", "2C"])); // four-of-a-kind beats full house
        assert!(hand_score(&["4H", "4H", "4H", "4D", "2C"]) < hand_score(&["6S", "6C", "6D", "6S", "2C"])); // higher four-of-a-kind beats lower four-of-a-kind
        assert!(hand_score(&["3H", "4H", "5H", "6H", "7H"]) > hand_score(&["6S", "6C", "6D", "6S", "2C"])); // straight flush beats four-of-a-kind
        assert!(hand_score(&["3H", "4H", "5H", "6H", "7H"]) < hand_score(&["AS", "KS", "QS", "JS", "TS"])); // royal flush beats straight flush
    }
}