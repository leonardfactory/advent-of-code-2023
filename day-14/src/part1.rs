use colored::Colorize;
use toolkit::{
    debug,
    map::{self, Map as BaseMap, Pos, TileDisplay},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Round,
    Cube,
    Empty,
}

impl TileDisplay for Tile {
    fn map_print(&self, _pos: Pos) -> Box<dyn std::fmt::Display> {
        match self {
            Tile::Round => Box::new("O".cyan()),
            Tile::Cube => Box::new("#".magenta()),
            Tile::Empty => Box::new('.'),
        }
    }
}

pub type Map = BaseMap<Tile>;

pub fn parse_rocks(input: &str) -> Map {
    Map::parse(input, |c, _, _| match c {
        'O' => Some(Tile::Round),
        '#' => Some(Tile::Cube),
        '.' => Some(Tile::Empty),
        _ => None,
    })
}

pub fn swap_tiles(map: &mut Map, from: Pos, to: Pos) {
    let swapped = map.set(to, *map.get(from).unwrap());
    map.set(from, swapped.unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

fn project(map: &Map, pos: Pos, dir: Direction) -> Pos {
    match dir {
        Direction::North => Pos::new(pos.x, pos.y),
        Direction::South => Pos::new(pos.x, map.height() - pos.y - 1),
        Direction::West => Pos::new(pos.y, pos.x),
        Direction::East => Pos::new(map.width() - pos.y - 1, pos.x),
    }
}

pub fn slide_direction(map: &mut Map, dir: Direction) {
    for x in 0..map.width() {
        let mut blocked_y = 0;
        for y in 0..map.height() {
            let pos = Pos::new(x, y);
            let tile = map.get(project(map, pos, dir)).unwrap();
            match tile {
                Tile::Empty => {}
                Tile::Cube => {
                    blocked_y = y + 1; // Next
                }
                Tile::Round => {
                    let from = project(map, Pos::new(x, y), dir);
                    let to = project(map, Pos::new(x, blocked_y), dir);
                    swap_tiles(map, from, to);
                    blocked_y += 1;
                }
            }
        }
    }
    // debug!("Sliding {:?}:", dir);
    // map.print();
}

pub fn calculate_north_beams_load(map: &Map) -> u32 {
    map.iter()
        .map(|(&pos, &tile)| match tile {
            Tile::Round => map.height() as u32 - pos.y as u32,
            _ => 0,
        })
        .sum::<u32>()
}

pub fn north_beams_load(input: &str) -> u32 {
    let mut map = parse_rocks(input);
    slide_direction(&mut map, Direction::North);
    map.print();
    calculate_north_beams_load(&map)
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(north_beams_load(input), 136);
    }
}
