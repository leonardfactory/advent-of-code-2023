const VALID_TOKENS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn parse_chars_line(line: &str) -> i32 {
    line.chars()
        .enumerate()
        .into_iter()
        .find_map(|(i, c)| {
            if c.is_ascii_digit() {
                return Some(c.to_digit(10).unwrap() as i32);
            }
            for (j, token) in VALID_TOKENS.iter().enumerate() {
                if i + token.len() < line.len() && line[i..(i + token.len())].eq(*token) {
                    return Some((j + 1) as i32);
                }
            }
            None
        })
        .unwrap()
}

pub fn parse_chars_line_rev(line: &str) -> i32 {
    let REVERSED_TOKENS: Vec<String> = VALID_TOKENS
        .iter()
        .map(|t| t.chars().rev().collect::<String>())
        .collect::<Vec<_>>();

    let rev_line = line.chars().rev().collect::<String>();

    rev_line
        .chars()
        .enumerate()
        .into_iter()
        .find_map(|(i, c)| {
            if c.is_ascii_digit() {
                return Some(c.to_digit(10).unwrap() as i32);
            }
            for (j, token) in REVERSED_TOKENS.iter().enumerate() {
                // println!(
                //     "Checking token {} on slice {}",
                //     token,
                //     &rev_line[i..(i + token.len())]
                // );
                if i + token.len() < rev_line.len() && rev_line[i..(i + token.len())].eq(token) {
                    return Some((j + 1) as i32);
                }
            }
            None
        })
        .unwrap()
}

pub fn sum_string_tokens(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .iter()
        .map(|line| {
            format!("{}{}", parse_chars_line(line), parse_chars_line_rev(line))
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let test = include_str!("../test2.txt");
        assert_eq!(sum_string_tokens(test), 281);
    }
}
