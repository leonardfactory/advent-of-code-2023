use itertools::Itertools;

pub fn resolve_line(line: &str) -> i32 {
    let chars: Vec<char> = line.chars().collect();
    let first = chars.iter().find_map(|&c| c.to_digit(10));
    let last = chars.iter().rev().find_map(|&c| c.to_digit(10));
    format!("{}{}", first.unwrap(), last.unwrap())
        .parse::<i32>()
        .unwrap()
}

pub fn sum_all_lines(input: &str) -> i32 {
    let lines = input.lines().map(resolve_line).collect_vec();
    lines.iter().sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(sum_all_lines(input), 142);
    }
}
