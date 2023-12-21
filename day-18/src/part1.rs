use std::{
    collections::{HashSet, VecDeque},
    io,
};

use colored::Colorize;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use toolkit::{
    color::Rgb,
    map::{Map as BaseMap, Pos, TileDisplay},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlanItem {
    pub dir: Pos,
    pub dist: u32,
    pub color: Rgb,
}

pub fn parse_plan(input: &str) -> Vec<PlanItem> {
    lazy_static! {
        static ref ITEM_RE: Regex = Regex::new(r"(\w+) (\d+) \(#(\w+)\)").unwrap();
    }

    input
        .lines()
        .map(|line| {
            let captures = ITEM_RE.captures(line).unwrap();
            let dir = match captures.get(1).unwrap().as_str() {
                "U" => Pos::UP,
                "D" => Pos::DOWN,
                "R" => Pos::RIGHT,
                "L" => Pos::LEFT,
                _ => unreachable!(),
            };

            let dist = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let color = captures.get(3).unwrap().as_str().parse::<Rgb>().unwrap();

            PlanItem { dir, dist, color }
        })
        .collect_vec()
}

pub enum Tile {
    Empty,
    Fill,
    Dig { color: Rgb },
}

impl TileDisplay for Tile {
    fn map_print(&self, _pos: Pos) -> Box<dyn std::fmt::Display> {
        match self {
            Tile::Empty => Box::new(" ".to_string()),
            Tile::Fill => Box::new(" ".on_bright_cyan()),
            Tile::Dig { color } => Box::new("#".truecolor(color.r, color.g, color.b)),
        }
    }
}

pub type Map = BaseMap<Tile>;

pub fn dig_lagoon(input: &str) -> u32 {
    let plan = parse_plan(input);
    let mut map = Map::new();
    map.set(Pos::new(0, 0), Tile::Dig { color: Rgb::WHITE });

    let mut current_pos = Pos::ZERO;
    for item in plan.iter() {
        for _ in 0..item.dist {
            current_pos += item.dir;
            map.set(current_pos, Tile::Dig { color: item.color });
        }
    }

    map.update_bounds();
    map.print();

    let mut fill_pos = Pos::new(map.bounds.min.x, 0);
    loop {
        let fill_tile = map.get(fill_pos).unwrap_or(&Tile::Empty);
        if let Tile::Empty = fill_tile {
            fill_pos += Pos::RIGHT;
            continue;
        }

        let left_fill = map.get(fill_pos + Pos::LEFT).unwrap_or(&Tile::Empty);
        let right_fill = map.get(fill_pos + Pos::RIGHT).unwrap_or(&Tile::Empty);
        if matches!(left_fill, Tile::Empty) && matches!(right_fill, Tile::Empty) {
            break;
        } else {
            fill_pos = Pos::new(map.bounds.min.x, fill_pos.y + 1);
        }
    }

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut queue: VecDeque<Pos> = vec![fill_pos + Pos::RIGHT].into_iter().collect();
    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        let tile = map.get(pos).unwrap_or(&Tile::Empty);
        if matches!(tile, Tile::Empty) {
            map.set(pos, Tile::Fill);
            queue.push_back(pos + Pos::UP);
            queue.push_back(pos + Pos::DOWN);
            queue.push_back(pos + Pos::LEFT);
            queue.push_back(pos + Pos::RIGHT);
        }

        // map.print_and_highlight(pos);
        // let mut input: String = String::new();
        // io::stdin().read_line(&mut input).unwrap();
    }

    // // Fill
    // for y in map.bounds.y_range() {
    //     let mut inside = false;
    //     for x in map.bounds.x_range() {
    //         let pos = Pos::new(x, y);
    //         match map.get(pos).unwrap_or(&Tile::Empty) {
    //             Tile::Empty => {
    //                 if inside {
    //                     map.set(pos, Tile::Fill);
    //                 }
    //             }
    //             Tile::Fill => {}
    //             Tile::Dig { .. } => {
    //                 let left_tile = map.get(pos + Pos::LEFT).unwrap_or(&Tile::Empty);
    //                 if !inside && matches!(left_tile, Tile::Dig { .. }) {
    //                     inside = true;
    //                 }

    //                 let right_tile = map.get(pos + Pos::RIGHT).unwrap_or(&Tile::Empty);
    //                 if inside && matches!(left_tile, Tile::Dig { .. }) {
    //                     inside = false;
    //                 }
    //             }
    //         }
    //     }
    // }

    map.update_bounds();
    map.print();

    map.iter()
        .filter(|(_, tile)| !matches!(tile, Tile::Empty))
        .count() as u32
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(dig_lagoon(input), 62);
    }
}
