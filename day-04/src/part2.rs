use crate::part1::{parse_cards, Card};

pub fn count_winning_numbers(card: &Card) -> u32 {
    card.numbers
        .iter()
        .filter(|n| card.winning.contains(n))
        .count() as u32
}

pub fn run_scratchcard_game(input: &str) -> u32 {
    let cards = parse_cards(input);
    let mut copies_count: Vec<u32> = vec![1; cards.len()];
    for i in 0..cards.len() {
        let multiplier = copies_count[i];
        let score = count_winning_numbers(&cards[i]);
        println!(
            "Card: {}, Multiplier: {}, Score: {}",
            i + 1,
            multiplier,
            score
        );
        for j in 0..score {
            copies_count[i + j as usize + 1] += multiplier;
        }
        // println!(" - Count: {:?}", copies_count);
    }

    copies_count.iter().sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(run_scratchcard_game(test), 30);
    }
}
