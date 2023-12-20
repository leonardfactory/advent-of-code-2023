use std::cmp;

use colored::Colorize;
use itertools::Itertools;
use toolkit::{
    debug,
    map::{Map as BaseMap, Pos, TileDisplay},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Ash,
    Rock,
}

impl TileDisplay for Tile {
    fn map_print(&self, _pos: Pos) -> Box<dyn std::fmt::Display> {
        match self {
            Tile::Ash => Box::new("."),
            Tile::Rock => Box::new("#".color("red")),
        }
    }
}

pub type Map = BaseMap<Tile>;

pub fn parse_patterns(input: &str) -> Vec<Map> {
    input
        .split("\n\n")
        .map(|pattern| {
            Map::parse(pattern, |c, _x, _y| match c {
                '#' => Some(Tile::Rock),
                '.' => Some(Tile::Ash),
                _ => panic!("Invalid tile"),
            })
        })
        .collect_vec()
}

pub fn find_reflection_columns(map: &Map) -> Vec<i32> {
    let mut columns = Vec::new();
    // map.print();
    for col_index in 1..map.width() {
        let range = cmp::min(map.width() - col_index, col_index);
        let mut is_reflection = true;

        for i in 0..range {
            let mirror_left = map.iter_column(col_index - i - 1).map(|(_, t)| t);
            let mirror_right = map.iter_column(col_index + i).map(|(_, t)| t);
            if !mirror_left.eq(mirror_right) {
                is_reflection = false;
                break;
            }
        }

        if is_reflection {
            // println!("Found reflection at column {}", col_index);
            columns.push(col_index);
        }
    }
    columns
}

pub fn find_reflection_rows(map: &Map) -> Vec<i32> {
    let mut rows = Vec::new();
    for row_index in 1..map.height() {
        let range = cmp::min(map.height() - row_index, row_index);
        let mut is_reflection = true;

        for i in 0..range {
            let mirror_left = map.iter_row(row_index - i - 1).map(|(_, t)| t);
            let mirror_right = map.iter_row(row_index + i).map(|(_, t)| t);
            if !mirror_left.eq(mirror_right) {
                is_reflection = false;
                break;
            }
        }

        if is_reflection {
            // println!("Found reflection at row {}", row_index);
            rows.push(row_index);
        }
    }
    rows
}

pub fn find_reflections(input: &str) -> i32 {
    let patterns = parse_patterns(input);

    patterns
        .iter()
        .map(|pattern| {
            let cols = find_reflection_columns(pattern);
            let rows = find_reflection_rows(pattern);
            // pattern.print();

            cols.iter().sum::<i32>() + rows.iter().map(|r| r * 100).sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_reflection_columns() {
        let patterns = parse_patterns(include_str!("../test.txt"));
        let pattern = patterns.first().unwrap();
        let cols = find_reflection_columns(pattern);
        assert_eq!(cols, vec![5]);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(find_reflections(input), 405);
    }
}
