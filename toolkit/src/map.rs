pub mod bounds;
pub mod pos;

pub use bounds::*;
use colored::Colorize;
pub use pos::*;

use std::{collections::HashMap, fmt::Display, ops::Index};

pub type Neighbor<'a, T> = (Pos, &'a T);

/// A trait for types that can be displayed as a tile on a map.
pub trait TileDisplay {
    fn map_print(&self, pos: Pos) -> Box<dyn Display>;
}

/// A generic map, usually parsed from an input file, composed of tiles.
/// Each tile is stored in a HashMap, indexed by its position, in order
/// to allow for fast lookup and virtually unlimited size.
#[derive(Clone, Debug)]
pub struct Map<T> {
    pub tiles: HashMap<Pos, T>,
    pub bounds: Bounds,
}

impl<T> Map<T> {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            bounds: Bounds::empty(),
        }
    }

    pub fn update_bounds(&mut self) {
        self.bounds.max.x = self.tiles.keys().map(|p| p.x).max().unwrap_or(0);
        self.bounds.max.y = self.tiles.keys().map(|p| p.y).max().unwrap_or(0);
        self.bounds.min.x = self.tiles.keys().map(|p| p.x).min().unwrap_or(0);
        self.bounds.min.y = self.tiles.keys().map(|p| p.y).min().unwrap_or(0);
    }

    pub fn from_tiles(tiles: Vec<(Pos, T)>) -> Self {
        let mut map = Self::new();
        map.tiles = tiles.into_iter().collect();
        map.update_bounds();
        map
    }

    pub fn parse(input: &str, mut parse_tile: impl FnMut(char, usize, usize) -> Option<T>) -> Self {
        let mut map = Self::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(tile) = parse_tile(c, x, y) {
                    map.tiles.insert(Pos::new(x, y), tile);
                }
            }
        }

        map.update_bounds();

        map
    }

    pub fn width(&self) -> i32 {
        self.bounds.width()
    }

    pub fn height(&self) -> i32 {
        self.bounds.height()
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        self.tiles.get(&pos)
    }

    pub fn all_neighbors(&self, pos: Pos) -> Vec<Neighbor<T>> {
        pos.all_neighbors()
            .into_iter()
            .filter_map(|p| self.get(p).map(|t| (p, t)))
            .collect()
    }

    pub fn is_near_bounds(&self, pos: Pos) -> bool {
        pos.x >= self.bounds.max.x
            || pos.x <= self.bounds.min.x
            || pos.y >= self.bounds.max.y
            || pos.y <= self.bounds.min.y
    }

    pub fn neighbors(&self, pos: Pos) -> Vec<Neighbor<T>> {
        pos.neighbors()
            .into_iter()
            .filter_map(|p| self.get(p).map(|t| (p, t)))
            .collect()
    }

    pub fn print_with(&self, display_tile: impl Fn(&T, Pos) -> Box<dyn Display>) {
        println!("\nMap:");
        for y in self.bounds.min.y..=self.bounds.max.y {
            for x in self.bounds.min.x..=self.bounds.max.x {
                let pos = Pos::new(x, y);
                if let Some(tile) = self.get(pos) {
                    print!("{}", display_tile(tile, pos));
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn iter_column(&self, x: i32) -> impl Iterator<Item = (Pos, &T)> {
        (self.bounds.min.y..=self.bounds.max.y).map(move |y| {
            let pos = Pos::new(x, y);
            (pos, self.get(pos).unwrap())
        })
    }

    pub fn iter_row(&self, y: i32) -> impl Iterator<Item = (Pos, &T)> {
        (self.bounds.min.x..=self.bounds.max.x).map(move |x| {
            let pos = Pos::new(x, y);
            (pos, self.get(pos).unwrap())
        })
    }
}

impl<T: TileDisplay> Map<T> {
    pub fn print_and_highlight(&self, highlight: Pos) {
        self.print_with(|tile, pos| {
            if pos == highlight {
                Box::new("X".color("yellow"))
            } else {
                tile.map_print(pos)
            }
        })
    }

    pub fn print(&self) {
        self.print_with(|tile, pos| tile.map_print(pos))
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Pos> for Map<T> {
    type Output = T;

    fn index(&self, pos: Pos) -> &T {
        &self.tiles[&pos]
    }
}
