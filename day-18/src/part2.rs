use colored::Colorize;
use std::cmp;

use crate::part1::{Map, PlanItem, Tile};
use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer;
use regex::Regex;
use toolkit::{
    color::Rgb,
    debug,
    map::{Bounds, Pos},
};

pub fn parse_real_plan(input: &str) -> Vec<PlanItem> {
    lazy_static! {
        static ref ITEM_RE: Regex = Regex::new(r"(\w+) (\d+) \(#(\w+)\)").unwrap();
    }

    input
        .lines()
        .map(|line| {
            let captures = ITEM_RE.captures(line).unwrap();
            let dist = u32::from_str_radix(&captures[3][0..5], 16).unwrap();
            let dir = match u32::from_str_radix(&captures[3][5..6], 16).unwrap() {
                0 => Pos::RIGHT,
                1 => Pos::DOWN,
                2 => Pos::LEFT,
                3 => Pos::UP,
                _ => unreachable!(),
            };

            PlanItem {
                dir,
                dist,
                color: Rgb::WHITE,
            }
        })
        .collect_vec()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    pub from: Pos,
    pub to: Pos,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub is_vertical: bool,
}

pub fn dig_wide_lagoon(input: &str) -> u64 {
    let plan = parse_real_plan(input);
    dig_lagoon_with_plan(plan)
}

pub fn dig_lagoon_with_plan(plan: Vec<PlanItem>) -> u64 {
    let mut map = Map::new();
    map.set(Pos::new(0, 0), Tile::Dig { color: Rgb::WHITE });

    let mut edges: Vec<Edge> = Vec::new();
    let mut current_pos = Pos::ZERO;
    let mut bounds = Bounds::new(Pos::ZERO, Pos::ZERO);

    for item in plan.iter() {
        let next_pos = current_pos + item.dir * item.dist as i32;
        edges.push(Edge {
            from: current_pos,
            to: next_pos,
            min_x: cmp::min(current_pos.x, next_pos.x),
            max_x: cmp::max(current_pos.x, next_pos.x),
            min_y: cmp::min(current_pos.y, next_pos.y),
            max_y: cmp::max(current_pos.y, next_pos.y),
            is_vertical: item.dir == Pos::UP || item.dir == Pos::DOWN,
        });

        bounds.insert_pos(next_pos);
        current_pos = next_pos;
    }

    let mut y_count: u64 = 0;
    for y in bounds.y_range() {
        // println!("\n\nWe are: {}", y.to_string().on_bright_cyan());
        let y_edges = edges
            .iter()
            .filter(|edge| y <= edge.max_y && y >= edge.min_y)
            .sorted_by(|edge, other| {
                if edge.min_x == other.min_x {
                    edge.max_x.cmp(&other.max_x)
                } else {
                    edge.min_x.cmp(&other.min_x)
                }
            })
            .collect_vec();

        let mut fill_count = u64::from(!y_edges.is_empty());
        let mut is_inside = false;
        for (i, edge) in y_edges.iter().enumerate() {
            // println!("\n--> {}:\n{:?}\n{:?}", i, edge, next_edge);
            if edge.is_vertical {
                is_inside = !is_inside;

                let next_edge = y_edges.get(i + 1);
                if let Some(next_edge) = next_edge {
                    let fill = if is_inside || !next_edge.is_vertical {
                        next_edge.max_x - edge.max_x
                    } else {
                        1
                    };
                    assert!(fill >= 0);
                    fill_count += fill as u64;
                }
            } else {
                let prev_edge = y_edges[i - 1];
                let next_edge = y_edges[i + 1];
                assert!(prev_edge.is_vertical && next_edge.is_vertical);

                if (prev_edge.max_y == next_edge.max_y && prev_edge.max_y == y)
                    || (prev_edge.min_y == next_edge.min_y && prev_edge.min_y == y)
                {
                    // Same
                } else {
                    is_inside = !is_inside;
                }
            }
        }

        y_count += fill_count;
    }

    println!("bounds: {:?}, edges: {}", bounds, edges.len());
    y_count as u64
}

#[cfg(test)]
pub mod tests {
    use crate::{part1::parse_plan, part2::*};

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(dig_lagoon_with_plan(parse_plan(input)), 62);
        let input = include_str!("../input.txt");
        assert_eq!(dig_lagoon_with_plan(parse_plan(input)), 50603);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(dig_wide_lagoon(input), 952408144115);
    }
}
