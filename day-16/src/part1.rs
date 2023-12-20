use std::collections::{HashSet, VecDeque};

use colored::Colorize;
use itertools::Itertools;
use toolkit::{
    debug,
    map::{Map, Pos, TileDisplay},
};

pub fn part1() {}

pub enum Tile {
    MirrorTopLeft,
    MirrorTopRight,
    SplitterVertical,
    SplitterHorizontal,
    Empty,
}

impl TileDisplay for Tile {
    fn map_print(&self, _pos: Pos) -> Box<dyn std::fmt::Display> {
        match self {
            Tile::MirrorTopLeft => Box::new("\\"),
            Tile::MirrorTopRight => Box::new('/'),
            Tile::SplitterVertical => Box::new('|'),
            Tile::SplitterHorizontal => Box::new('-'),
            Tile::Empty => Box::new('.'),
        }
    }
}

pub type Contraption = Map<Tile>;
pub type Dir = Pos;

pub fn parse_contraption(input: &str) -> Contraption {
    Map::parse(input, |c, _x, _y| match c {
        '\\' => Some(Tile::MirrorTopLeft),
        '/' => Some(Tile::MirrorTopRight),
        '|' => Some(Tile::SplitterVertical),
        '-' => Some(Tile::SplitterHorizontal),
        '.' => Some(Tile::Empty),
        _ => None,
    })
}

pub fn count_energized_topleft(input: &str) -> usize {
    let contraption = parse_contraption(input);
    count_energized(&contraption, (Pos::new(-1, 0), Pos::RIGHT))
}

pub fn count_energized(contraption: &Contraption, start: (Pos, Dir)) -> usize {
    let mut energized: HashSet<(Pos, Dir)> = HashSet::new();

    let mut beams: VecDeque<(Pos, Dir)> = VecDeque::new();
    beams.push_back(start);

    while let Some((pos, dir)) = beams.pop_front() {
        // contraption.print_and_highlight(pos);
        let next_pos = pos + dir;

        if energized.contains(&(next_pos, dir)) || !contraption.bounds.contains(next_pos) {
            continue;
        }

        energized.insert((next_pos, dir));

        match contraption.get(next_pos) {
            Some(Tile::MirrorTopLeft) => {
                // "\"
                let new_dir = match dir {
                    Pos::UP => Pos::LEFT,
                    Pos::RIGHT => Pos::DOWN,
                    Pos::DOWN => Pos::RIGHT,
                    Pos::LEFT => Pos::UP,
                    _ => unreachable!(),
                };

                beams.push_back((next_pos, new_dir));
            }
            Some(Tile::MirrorTopRight) => {
                // "/"
                let new_dir = match dir {
                    Pos::UP => Pos::RIGHT,
                    Pos::RIGHT => Pos::UP,
                    Pos::DOWN => Pos::LEFT,
                    Pos::LEFT => Pos::DOWN,
                    _ => unreachable!(),
                };

                beams.push_back((next_pos, new_dir));
            }
            Some(Tile::SplitterVertical) => {
                // "|"
                match dir {
                    Pos::UP | Pos::DOWN => {
                        beams.push_back((next_pos, dir));
                    }
                    Pos::RIGHT | Pos::LEFT => {
                        beams.push_back((next_pos, Pos::UP));
                        beams.push_back((next_pos, Pos::DOWN));
                    }
                    _ => unreachable!(),
                }
            }
            Some(Tile::SplitterHorizontal) => {
                // "-"
                match dir {
                    Pos::UP | Pos::DOWN => {
                        beams.push_back((next_pos, Pos::LEFT));
                        beams.push_back((next_pos, Pos::RIGHT));
                    }
                    Pos::RIGHT | Pos::LEFT => {
                        beams.push_back((next_pos, dir));
                    }
                    _ => unreachable!(),
                }
            }
            Some(Tile::Empty) => {
                // "."
                beams.push_back((next_pos, dir));
            }
            None => (),
        }
    }

    // contraption.print_with(|tile, pos| {
    //     if energized.iter().any(|(p, _)| *p == pos) {
    //         Box::new("#".yellow())
    //     } else {
    //         tile.map_print(pos)
    //     }
    // });
    // debug!("Energized: {:#?}", energized);
    energized.iter().unique_by(|(p, _)| *p).count()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_energized_topleft(input), 46);
    }
}
