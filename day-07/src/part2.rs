use std::collections::HashMap;

use itertools::Itertools;

use crate::part1::Hand;

pub fn parse_card(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        n => n.to_digit(10).unwrap(),
    }
}

pub fn format_card(c: &u32) -> char {
    match c {
        14 => 'A',
        13 => 'K',
        12 => 'Q',
        1 => 'J',
        10 => 'T',
        n => std::char::from_digit(*n, 10).unwrap(),
    }
}

impl Hand {
    fn jolly_score(&self) -> u32 {
        println!("\n[Jolly] Set {}", self.to_string());
        let (jollys, cards): (Vec<_>, Vec<_>) =
            self.cards.clone().into_iter().partition(|c| *c == 1);

        if jollys.is_empty() {
            println!(" - No jollys, score={}", self.score());
            return self.score();
        }

        if jollys.len() == 5 {
            println!(" - All jollys, score=6");
            return 6;
        }

        let available_cards = cards
            .clone()
            .into_iter()
            .unique()
            .combinations_with_replacement(jollys.len());

        available_cards
            .into_iter()
            .map(|mut jolly_cards| {
                let mut jolly_hand = Hand {
                    bid: self.bid,
                    cards: cards.clone(),
                };
                jolly_hand.cards.append(&mut jolly_cards);
                println!(
                    " - Try with {:?}: {}, score={}",
                    jolly_cards,
                    jolly_hand.to_string(),
                    jolly_hand.score()
                );
                jolly_hand.score()
            })
            .max()
            .unwrap()
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

fn jolly_cmp(hand: &Hand, other: &Hand) -> std::cmp::Ordering {
    let self_score = hand.jolly_score();
    // println!("Self {} score: {}", hand.to_string(), self_score);
    let other_score = other.jolly_score();
    // println!("Other {} score: {}", other.to_string(), other_score);

    match self_score.cmp(&other_score) {
        std::cmp::Ordering::Equal => {
            for (self_card, other_card) in hand.cards.iter().zip(other.cards.iter()) {
                if self_card != other_card {
                    return self_card.partial_cmp(other_card).unwrap();
                }
            }
            panic!("Hands are equal")
        }
        result => result,
    }
}

pub fn jolly_total_winnings(input: &str) -> u32 {
    let hands = parse_hands(input);
    // println!("Hands: {:?}", hands);
    let sorted_hands = hands.iter().sorted_by(|a, b| jolly_cmp(a, b)).collect_vec();
    sorted_hands
        .iter()
        .enumerate()
        .for_each(|(i, h)| println!("{}: {}", i + 1, h.to_string()));
    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_score() {
        let hands = parse_hands("T55J5 1\nKTJJT 1\nQQQJA 1");
        assert_eq!(hands[0].jolly_score(), 5);
        assert_eq!(hands[1].jolly_score(), 5);
        assert_eq!(hands[2].jolly_score(), 5);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(jolly_total_winnings(input), 5905);
    }
}
