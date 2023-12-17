use std::{collections::HashMap, fmt::Display};

use colored::Colorize;
use itertools::Itertools;
use toolkit::{
    graph::{bfs_cache, bfs_count},
    map::{Map as BaseMap, Pos, TileDisplay},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tile {
    Empty,
    Galaxy(u32),
}

impl TileDisplay for Tile {
    fn map_print(&self, _pos: Pos) -> Box<dyn Display> {
        match self {
            Tile::Empty => Box::new('.'),
            Tile::Galaxy(_galaxy_index) => Box::new("#".yellow()),
        }
    }
}

pub type Map = BaseMap<Tile>;

pub fn parse_universe(input: &str) -> Map {
    let mut galaxy_index: u32 = 0;
    Map::parse(input, |c, x, y| {
        let pos = Pos::new(x, y);
        match c {
            '.' => Some(Tile::Empty),
            '#' => {
                galaxy_index += 1;
                Some(Tile::Galaxy(galaxy_index))
            }
            _ => panic!("Invalid character: {}", c),
        }
    })
}

pub fn expand_universe(input: &str) -> Map {
    let map = parse_universe(input);
    let mut expanded_map = Map::new();

    let mut row_shift = 0;

    for y in map.bounds.min.y..=map.bounds.max.y {
        let row = map.tiles.iter().filter(|(pos, _)| pos.y == y).collect_vec();

        row.iter().for_each(|&(pos, tile)| {
            expanded_map.tiles.insert(pos.add_y(row_shift), *tile);
        });

        if row.iter().all(|&(_, &tile)| tile == Tile::Empty) {
            row_shift += 1;
            row.iter().for_each(|&(pos, _)| {
                expanded_map.tiles.insert(pos.add_y(row_shift), Tile::Empty);
            });
        }
    }

    let mut column_shift = 0;
    for x in map.bounds.min.x..=map.bounds.max.x {
        let is_col_empty = map
            .tiles
            .iter()
            .filter(|(pos, _)| pos.x == x)
            .all(|(_, &tile)| tile == Tile::Empty);

        if is_col_empty {
            let updated_tiles: Vec<(Pos, Tile)> = expanded_map
                .tiles
                .iter()
                .map(|(pos, tile)| {
                    (
                        if pos.x > x + column_shift {
                            pos.add_x(1)
                        } else {
                            *pos
                        },
                        *tile,
                    )
                })
                .collect_vec();

            expanded_map = Map::from_tiles(updated_tiles);
            column_shift += 1;
            for y in expanded_map.bounds.min.y..=expanded_map.bounds.max.y {
                expanded_map
                    .tiles
                    .insert(Pos::new(column_shift + x, y), Tile::Empty);
            }
        }
    }

    expanded_map
}

pub fn count_distances(input: &str) -> u32 {
    let map = expand_universe(input);
    println!("Expanded map:");
    map.print();
    let galaxies = map
        .tiles
        .iter()
        .filter_map(|(pos, tile)| match tile {
            Tile::Galaxy(galaxy_index) => Some((*pos, *galaxy_index)),
            _ => None,
        })
        .collect_vec();

    let distance_cache: HashMap<Pos, HashMap<Pos, usize>> = galaxies
        .iter()
        .map(|&(pos, galaxy_index)| {
            (
                pos,
                bfs_cache(pos, |pos| {
                    map.neighbors(pos).iter().map(|&(p, _)| p).collect_vec()
                }),
            )
        })
        .collect();

    let pairs = galaxies.iter().combinations(2).collect_vec();
    println!("Pairs: {}", pairs.len());
    pairs
        .iter()
        .map(|pair| {
            *distance_cache
                .get(&pair[0].0)
                .unwrap()
                .get(&pair[1].0)
                .unwrap() as u32
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_expanded() {
        let input = include_str!("../test.txt");
        let map = expand_universe(input);
        let map_ok = parse_universe(include_str!("../test_exp.txt"));
        assert_eq!(map.tiles, map_ok.tiles);
        assert_eq!(map.bounds, map_ok.bounds);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_distances(input), 374);
        // assert_eq!(map.bounds, map_ok.bounds);
    }
}
