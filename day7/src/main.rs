use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const JOKER_VAL: u8 = 1;

const FIVE_OF_A_KIND: u8 = 6;
const FOUR_OF_A_KIND: u8 = 5;
const FULL_HOUSE: u8 = 4;
const THREE_OF_A_KIND: u8 = 3;
const TWO_PAIR: u8 = 2;
const PAIR: u8 = 1;
const HIGH_CARD: u8 = 0;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    str: String,
    cards: Vec<u8>,
}

impl Hand {
    fn get_rank(&self) -> u8 {
        let mut card_counts: HashMap<u8, u8> = HashMap::new();
        let mut joker_count = 0;
        for &card in &self.cards {
            if card == JOKER_VAL {
                joker_count += 1;
            } else {
                *card_counts.entry(card).or_insert(0) += 1;
            }
        }

        let mut tmp: Vec<_> = card_counts.iter().collect();
        tmp.sort_by(|(a_val, a_count), (b_val, b_count)| {
            if a_count == b_count {
                a_val.cmp(&b_val)
            } else {
                b_count.cmp(&a_count)
            }
        });

        if joker_count == 5 {
            return FIVE_OF_A_KIND;
        }

        let highest_count = tmp[0].1;
        let next_highest_count = if tmp.len() > 1 { tmp[1].1 } else { &0 };
        match (highest_count, next_highest_count, joker_count) {
            (5, _, _) => FIVE_OF_A_KIND,
            (4, _, 1) => FIVE_OF_A_KIND,
            (4, _, 0) => FOUR_OF_A_KIND,
            (3, _, 2) => FIVE_OF_A_KIND,
            (3, _, 1) => FOUR_OF_A_KIND,
            (3, 2, _) => FULL_HOUSE,
            (3, _, _) => THREE_OF_A_KIND,
            (2, _, 3) => FIVE_OF_A_KIND,
            (2, _, 2) => FOUR_OF_A_KIND,
            (2, 2, 1) => FULL_HOUSE,
            (2, _, 1) => THREE_OF_A_KIND,
            (2, 2, 0) => TWO_PAIR,
            (2, _, _) => PAIR,
            (1, _, 4) => FIVE_OF_A_KIND,
            (1, _, 3) => FOUR_OF_A_KIND,
            (1, _, 2) => THREE_OF_A_KIND,
            (1, _, 1) => PAIR,
            _ => HIGH_CARD,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank = self.get_rank();
        let other_rank = other.get_rank();
        let rank_cmp = self_rank.cmp(&other_rank);
        if rank_cmp != Ordering::Equal {
            return rank_cmp;
        } else {
            for i in 0..self.cards.len() {
                let c = self.cards[i].cmp(&other.cards[i]);
                if c != Ordering::Equal {
                    return c;
                }
            }
        }

        return Ordering::Equal;
    }
}

fn parse_hands(lines: &Vec<String>) -> Vec<(Hand, u32)> {
    return lines
        .iter()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            let cards = parts[0]
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap() as u8,
                })
                .collect();

            (
                Hand {
                    str: parts[0].to_string(),
                    cards: cards,
                },
                parts[1].parse().unwrap(),
            )
        })
        .collect();
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut hands = parse_hands(lines);
    hands.sort_by(|(a, _), (b, _)| a.cmp(&b));
    println!("{:?}", hands);
    for (hand, _) in &hands {
        println!("{:?} {}", hand, hand.get_rank());
    }

    let mut result = 0;
    for (rank, (_, bet)) in hands.iter().enumerate() {
        result += (rank as u32 + 1) * bet;
    }

    return result;
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let pt2_result = part2(&lines);

    println!("Part 2: {}", pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("32T3K 765"),
            String::from("T55J5 684"),
            String::from("KK677 28"),
            String::from("KTJJT 220"),
            String::from("QQQJA 483"),
        ];

        let result = part2(&lines);
        assert_eq!(result, 5905);
    }
}
