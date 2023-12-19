use std::fmt::Display;

use itertools::Itertools;

use crate::part2::solve_backtracking;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spring {
    Broken,
    Operative,
    Unknown,
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Broken => write!(f, "#"),
            Spring::Operative => write!(f, "."),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Row {
    pub springs: Vec<Spring>,
    pub groups: Vec<usize>,
    pub fold_len: usize,
    pub fold_groups_len: usize,
}

pub fn parse_springs(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let mut row = Row {
                springs: Vec::new(),
                groups: Vec::new(),
                fold_len: 0,
                fold_groups_len: 0,
            };

            let (springs, groups) = line.split_once(' ').unwrap();

            for c in springs.chars() {
                match c {
                    '#' => row.springs.push(Spring::Broken),
                    '.' => row.springs.push(Spring::Operative),
                    '?' => row.springs.push(Spring::Unknown),
                    _ => (),
                }
            }

            for group in groups.split(',') {
                let group = group.parse::<usize>().unwrap();
                row.groups.push(group);
            }

            row.fold_len = row.springs.len();
            row.fold_groups_len = row.groups.len();

            row
        })
        .collect_vec()
}

fn springs_match_group(springs: &Vec<Spring>, groups: &Vec<usize>) -> bool {
    let mut computed_groups = Vec::new();
    let mut current_group = 0;
    let mut is_broken = false;
    for spring in springs {
        match spring {
            Spring::Broken => {
                is_broken = true;
                current_group += 1;
            }
            Spring::Operative => {
                if is_broken {
                    computed_groups.push(current_group);
                    current_group = 0;
                    is_broken = false;
                }
            }
            Spring::Unknown => panic!("Unknown spring"),
        }
    }

    if is_broken {
        computed_groups.push(current_group);
    }

    computed_groups == *groups
}

pub fn solve_row(row: &Row) -> u32 {
    let mut valid_count = 0;

    let unknows = row
        .springs
        .iter()
        .enumerate()
        .filter(|(_, s)| matches!(s, Spring::Unknown))
        .collect_vec();

    let valid_spring_types = vec!['.', '#'];
    let combinations = (1..=unknows.len())
        .map(|_| valid_spring_types.iter())
        .multi_cartesian_product();

    println!("Combs for row: {}", row.springs.iter().join(""));

    for combination in combinations {
        let mut springs = row.springs.clone();
        for (i, c) in combination.iter().enumerate() {
            let index = unknows[i].0;
            match c {
                '.' => springs[index] = Spring::Operative,
                '#' => springs[index] = Spring::Broken,
                _ => panic!("Unknown spring"),
            }
        }

        if springs_match_group(&springs, &row.groups) {
            // println!(" -> {}", springs.iter().join(""));
            // print!(" -> Valid");
            valid_count += 1;
        }
    }

    // springs
    valid_count
}

pub fn solve_valid_springs(input: &str) -> u64 {
    let rows = parse_springs(input);
    rows.iter().map(solve_backtracking).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(solve_valid_springs(input), 21);
    }
}
