#[derive(Debug, Clone)]
pub struct Card {
    pub winning: Vec<u32>,
    pub numbers: Vec<u32>,
}

pub fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_, numbers_text) = line.split_once(": ").unwrap();
            let (winning_text, numbers_text) = numbers_text.split_once(" | ").unwrap();
            let winning = winning_text
                .split(' ')
                .filter_map(|n| n.trim().parse::<u32>().ok())
                .collect();

            let numbers = numbers_text
                .split(' ')
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();

            Card { winning, numbers }
        })
        .collect()
}

pub fn card_score(card: &Card) -> u32 {
    let found_winning = card
        .numbers
        .iter()
        .filter(|n| card.winning.contains(n))
        .count();

    match found_winning {
        0 => 0,
        n => u32::pow(2, (n - 1) as u32),
    }
}

pub fn count_winning(input: &str) -> u32 {
    let cards = parse_cards(input);
    cards.iter().map(card_score).sum::<u32>()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parse() {
        let test = "Card 1: 12 13 | 3  2 12";
        let card = parse_cards(test);
        assert_eq!(card[0].winning, vec![12, 13]);
        assert_eq!(card[0].numbers, vec![3, 2, 12]);
    }

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(count_winning(test), 13);
    }
}
