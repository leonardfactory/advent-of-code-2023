use std::{collections::HashMap, fmt::Display};

use colored::Colorize;
use itertools::Itertools;
use toolkit::map::{Map as BaseMap, Pos, TileDisplay};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
}

impl TileDisplay for Tile {
    fn map_print(&self, pos: Pos) -> Box<dyn Display> {
        match self {
            Tile::Ground => Box::new('.'),
            Tile::Start => Box::new('S'),
            Tile::Pipe(pipe) => match (pipe.start - pos, pipe.end - pos) {
                (Pos { x: 0, y: -1 }, Pos { x: 0, y: 1 }) => Box::new("│".color("red")),
                (Pos { x: -1, y: 0 }, Pos { x: 1, y: 0 }) => Box::new("─".color("red")),
                (Pos { x: 0, y: -1 }, Pos { x: 1, y: 0 }) => Box::new("└".color("red")),
                (Pos { x: 0, y: -1 }, Pos { x: -1, y: 0 }) => Box::new("┘".color("red")),
                (Pos { x: 0, y: 1 }, Pos { x: 1, y: 0 }) => Box::new("┌".color("red")),
                (Pos { x: 0, y: 1 }, Pos { x: -1, y: 0 }) => Box::new("┐".color("red")),
                _ => Box::new("?".color("yellow")),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pipe {
    pub start: Pos,
    pub end: Pos,
    pub connected: bool,
}

impl Pipe {
    pub fn shape(&self, pos: Pos) -> char {
        match (self.start - pos, self.end - pos) {
            (Pos { x: 0, y: -1 }, Pos { x: 0, y: 1 }) => '|',
            (Pos { x: -1, y: 0 }, Pos { x: 1, y: 0 }) => '-',
            (Pos { x: 0, y: -1 }, Pos { x: 1, y: 0 }) => 'L',
            (Pos { x: 0, y: -1 }, Pos { x: -1, y: 0 }) => 'J',
            (Pos { x: 0, y: 1 }, Pos { x: 1, y: 0 }) => 'F',
            (Pos { x: 0, y: 1 }, Pos { x: -1, y: 0 }) => '7',
            _ => Pipe::shape(
                &Pipe {
                    start: self.end,
                    end: self.start,
                    connected: self.connected,
                },
                pos,
            ),
        }
    }
}

fn tile_pipe(current: Pos, start: Pos, end: Pos) -> Tile {
    Tile::Pipe(Pipe {
        start: current + start,
        end: current + end,
        connected: false,
    })
}

pub type Map = BaseMap<Tile>;

pub fn matching_pipe(pos: Pos) -> Pos {
    match pos {
        Pos::UP => Pos::DOWN,
        Pos::DOWN => Pos::UP,
        Pos::LEFT => Pos::RIGHT,
        Pos::RIGHT => Pos::LEFT,
        _ => panic!("Invalid direction: {:?}", pos),
    }
}

pub fn parse_map(input: &str) -> Map {
    Map::parse(input, |c, x, y| {
        let pos = Pos::new(x, y);
        match c {
            '.' => Some(Tile::Ground),
            'S' => Some(Tile::Start),
            '|' => Some(tile_pipe(pos, Pos::UP, Pos::DOWN)),
            '-' => Some(tile_pipe(pos, Pos::LEFT, Pos::RIGHT)),
            'L' => Some(tile_pipe(pos, Pos::UP, Pos::RIGHT)),
            'J' => Some(tile_pipe(pos, Pos::UP, Pos::LEFT)),
            'F' => Some(tile_pipe(pos, Pos::DOWN, Pos::RIGHT)),
            '7' => Some(tile_pipe(pos, Pos::DOWN, Pos::LEFT)),
            _ => panic!("Invalid character: {}", c),
        }
    })
}

pub fn get_start_pos(map: &Map) -> Pos {
    *map.tiles
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::Start))
        .unwrap()
        .0
}

pub fn get_starting_pipes(map: &Map, start_pos: Pos) -> ((Pos, Pipe), (Pos, Pipe)) {
    let (start_cw, start_ccw) = &map
        .neighbors(start_pos)
        .iter()
        .filter_map(|(pos, tile)| {
            let target_pipe = matching_pipe(*pos - start_pos);
            println!(
                "target_pipe: {:?}, pos={:?}, start={:?}",
                target_pipe, pos, start_pos
            );
            match tile {
                Tile::Pipe(pipe)
                    if pipe.start - *pos == target_pipe || pipe.end - *pos == target_pipe =>
                {
                    println!(" - found pipe: {:?}", pipe);
                    Some((*pos, *pipe))
                }
                _ => None,
            }
        })
        .collect_tuple()
        .unwrap();

    (*start_cw, *start_ccw)
}

pub fn farthest_pipe(input: &str) -> u32 {
    let map = parse_map(input);
    let start_pos = get_start_pos(&map);

    let distance;

    map.print();

    let (start_cw, start_ccw) = get_starting_pipes(&map, start_pos);

    // We iterate clockwise and counter-clockwise at the same time, tracking
    // the distance from the start position to each tile. When the two
    // iterators meet, we have the farthest tile from the start position.

    let mut current_cw = start_cw.0;
    let mut tracked_clockwise: HashMap<Pos, u32> = HashMap::new();
    tracked_clockwise.insert(start_pos, 0);
    tracked_clockwise.insert(current_cw, 1);

    let mut current_ccw = start_ccw.0;
    let mut tracked_counter_clockwise: HashMap<Pos, u32> = HashMap::new();
    tracked_counter_clockwise.insert(start_pos, 0);
    tracked_counter_clockwise.insert(current_ccw, 1);

    loop {
        let cw_tile = map.get(current_cw).unwrap();

        match cw_tile {
            Tile::Pipe(pipe) => {
                let cw_targets = vec![pipe.start, pipe.end];
                let cw_target = cw_targets
                    .iter()
                    .find(|target| !tracked_clockwise.contains_key(target))
                    .unwrap();

                let cw_distance = tracked_clockwise.get(&current_cw).unwrap_or(&0) + 1;
                tracked_clockwise.insert(*cw_target, cw_distance);
                current_cw = *cw_target;
                println!("cw distance: {}", cw_distance);
            }
            _ => panic!("Invalid tile: {:?}", cw_tile),
        }

        let ccw_tile = map.get(current_ccw).unwrap();
        match ccw_tile {
            Tile::Pipe(pipe) => {
                let ccw_targets = vec![pipe.start, pipe.end];
                let ccw_target = ccw_targets
                    .iter()
                    .find(|target| !tracked_counter_clockwise.contains_key(target))
                    .unwrap();

                let ccw_distance = tracked_counter_clockwise.get(&current_ccw).unwrap_or(&0) + 1;
                tracked_counter_clockwise.insert(*ccw_target, ccw_distance);
                current_ccw = *ccw_target;
                println!("ccw distance: {}", ccw_distance);
            }
            _ => panic!("Invalid tile: {:?}", ccw_tile),
        }

        if current_cw == current_ccw {
            distance = *tracked_clockwise.get(&current_cw).unwrap();
            break;
        }
    }
    distance
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(farthest_pipe(input), 8);
    }
}
