use itertools::Itertools;

use crate::part1::{number_of_winning_ways, Race};

pub fn single_race_ways(input: &str) -> u64 {
    let race = parse_single_race(input);
    number_of_winning_ways(&race)
}

pub fn parse_single_race(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .trim()
        .split_ascii_whitespace()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .trim()
        .split_ascii_whitespace()
        .join("")
        .parse::<u64>()
        .unwrap();

    Race { time, distance }
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(single_race_ways(test), 71503);
    }
}
