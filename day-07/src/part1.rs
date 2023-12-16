use std::collections::HashMap;

use itertools::Itertools;

pub fn parse_card(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        n => n.to_digit(10).unwrap(),
    }
}

pub fn format_card(c: &u32) -> char {
    match c {
        14 => 'A',
        13 => 'K',
        12 => 'Q',
        11 => 'J',
        10 => 'T',
        n => std::char::from_digit(*n, 10).unwrap(),
    }
}

#[derive(Eq, Debug)]
pub struct Hand {
    pub cards: Vec<u32>,
    pub bid: u32,
}

impl Hand {
    pub fn score(&self) -> u32 {
        let mut groups: HashMap<u32, usize> = HashMap::new();
        self.cards.iter().for_each(|c| {
            groups
                .entry(*c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let unique_count = self.cards.iter().unique().count();
        // println!("Groups: {:?}", groups);

        match unique_count {
            1 => 6,                                                // Five of a kind
            2 if groups.iter().any(|(k, &group)| group == 4) => 5, // Four of a kind
            2 if groups.iter().any(|(k, &group)| group == 3) => 4, // Full house
            3 if groups.iter().any(|(k, &group)| group == 3) => 3, // Three of a kind
            3 => 2,                                                // Two pair
            4 => 1,                                                // One pair
            _ => 0,
        }
    }
}

impl ToString for Hand {
    fn to_string(&self) -> String {
        let cards = self.cards.iter().map(format_card).join("");
        format!("{} {}", cards, self.bid)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_score = self.score();
        println!("Self {} score: {}", self.to_string(), self_score);
        let other_score = other.score();
        println!("Other {} score: {}", other.to_string(), other_score);

        match self_score.cmp(&other_score) {
            std::cmp::Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    if self_card != other_card {
                        return self_card.partial_cmp(other_card);
                    }
                }
                panic!("Hands are equal")
            }
            result => Some(result),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards_str, bid_str) = line.split_once(' ').unwrap();
            let cards = cards_str.chars().map(parse_card).collect_vec();
            let bid = bid_str.parse::<u32>().unwrap();
            Hand { cards, bid }
        })
        .collect_vec()
}

pub fn total_winnings(input: &str) -> u32 {
    let hands = parse_hands(input);
    println!("Hands: {:?}", hands);
    let sorted_hands = hands.iter().sorted().collect_vec();
    sorted_hands
        .iter()
        .for_each(|h| println!("{}", h.to_string()));
    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(total_winnings(input), 6440);
    }
}
