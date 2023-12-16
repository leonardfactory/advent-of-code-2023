use itertools::Itertools;
use toolkit::map::{Map as BaseMap, Pos, TileDisplay};

type Map = BaseMap<Tile>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Digit { digit: u32, include: bool },
    Symbol(char),
}

pub fn parse_map(input: &str) -> Map {
    Map::parse(input, |c, _, _| match c {
        '.' => None,
        c if c.is_ascii_digit() => Some(Tile::Digit {
            digit: c.to_digit(10).unwrap(),
            include: false,
        }),
        c => Some(Tile::Symbol(c)),
    })
}

pub fn sum_valid_part_numbers(input: &str) -> u32 {
    let mut engine = parse_map(input);
    let engine_check = engine.clone();

    engine.tiles.iter_mut().for_each(|(pos, tile)| {
        if let Tile::Digit { digit: _, include } = tile {
            *include = engine_check
                .all_neighbors(*pos)
                .iter()
                .any(|(_p, &neighbor)| matches!(neighbor, Tile::Symbol(_)));
        }
    });

    let heads = engine
        .tiles
        .iter()
        .filter(|(&pos, tile)| {
            matches!(tile, Tile::Digit { digit: _, include })
                && !matches!(
                    engine.get(pos + Pos::W),
                    Some(Tile::Digit { digit, include })
                )
        })
        .collect_vec();

    let valid_numbers = heads
        .iter()
        .filter_map(|(&pos, tile)| {
            if let Tile::Digit { digit, include } = tile {
                let mut should_count: bool = *include;
                let mut value = *digit;
                let mut current_pos = pos;
                while let Some(Tile::Digit {
                    digit: next_digit,
                    include: next_include,
                }) = engine.get(current_pos + Pos::E)
                {
                    should_count |= next_include;
                    value *= 10;
                    value += next_digit;
                    current_pos = current_pos + Pos::E;
                }

                match should_count {
                    true => Some(value),
                    false => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    valid_numbers.iter().sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(sum_valid_part_numbers(test), 4361);
    }
}
