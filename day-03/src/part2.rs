use itertools::Itertools;
use toolkit::map::{Map as BaseMap, Pos};

type Map = BaseMap<Tile>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Digit { digit: u32, head: Pos },
    Gear,
    Symbol(char),
}

pub fn parse_map(input: &str) -> Map {
    Map::parse(input, |c, _, _| match c {
        '.' => None,
        c if c.is_ascii_digit() => Some(Tile::Digit {
            digit: c.to_digit(10).unwrap(),
            head: Pos::ZERO,
        }),
        '*' => Some(Tile::Gear),
        c => Some(Tile::Symbol(c)),
    })
}

pub fn parse_number_heads(engine: &mut Map) {
    let engine_check = engine.clone();
    engine.tiles.iter_mut().for_each(|(pos, tile)| {
        if let Tile::Digit { digit: _, head } = tile {
            let mut cursor = *pos;
            // Inefficient, but who cares!
            // Next time we could use a HashSet to keep track of visited positions.
            while let Some(Tile::Digit {
                digit,
                head: prev_head,
            }) = engine_check.get(cursor + Pos::W)
            {
                cursor = cursor + Pos::W;
            }

            *head = cursor;
        }
    });
}

pub fn find_gears(input: &str) -> u32 {
    let mut engine = parse_map(input);

    parse_number_heads(&mut engine);

    let mut gears: Vec<_> = engine
        .tiles
        .iter()
        .filter(|(_, tile)| matches!(tile, Tile::Gear))
        .collect();

    let mut gears_numbers = gears
        .iter()
        .map(|(&pos, _)| {
            engine
                .all_neighbors(pos)
                .iter()
                .filter_map(|(_p, &tile)| match tile {
                    Tile::Digit { digit: _, head } => Some(head),
                    _ => None,
                })
                .unique()
                .collect_vec()
        })
        .filter(|numbers| numbers.len() == 2)
        .collect_vec();

    gears_numbers
        .iter()
        .map(|number_heads| {
            if let Some((first, second)) = number_heads
                .iter()
                .filter_map(|&pos| match engine.get(pos) {
                    Some(Tile::Digit { digit, head: _ }) => {
                        let mut value = *digit;
                        let mut current_pos = pos;
                        while let Some(Tile::Digit {
                            digit: next_digit,
                            head: _,
                        }) = engine.get(current_pos + Pos::E)
                        {
                            value *= 10;
                            value += next_digit;
                            current_pos = current_pos + Pos::E;
                        }
                        Some(value)
                    }
                    _ => None,
                })
                .collect_tuple()
            {
                println!("{} * {} = {}", first, second, first * second);
                first * second
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(find_gears(test), 467835);
    }
}
