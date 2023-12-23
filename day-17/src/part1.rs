use std::{
    cmp,
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use lazy_static::lazy_static;
use toolkit::{
    debug,
    map::{Map, Pos, TileDisplay},
};

pub struct Heat(u32);
pub type HeatMap = Map<Heat>;

impl TileDisplay for Heat {
    fn map_print(&self, _pos: Pos) -> Box<dyn std::fmt::Display> {
        Box::new(format!("{}", self.0))
    }
}

pub fn parse_heatmap(input: &str) -> HeatMap {
    HeatMap::parse(input, |c, _x, _y| Some(Heat(c.to_digit(10).unwrap())))
}

pub type Dir = Pos;

pub fn tangent_dirs(dir: Dir) -> Vec<Dir> {
    vec![dir.rotate_left(), dir.rotate_right()]
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Route {
    pos: Pos,
    dir: Dir,
    /**
     * Number of steps left to take (max 3)
     */
    count: u32,
}

impl Route {
    /**
     * Same direction, one step forward
     */
    fn forward(&self) -> Self {
        Self {
            pos: self.pos + self.dir,
            dir: self.dir,
            count: self.count - 1,
        }
    }

    fn rotated(&self, dir: Dir) -> Self {
        Self {
            pos: self.pos + dir,
            dir,
            count: 2,
        }
    }

    fn print(&self) -> String {
        format!("{:?} {} c={}", self.pos, self.dir.as_dir_str(), self.count)
    }
}

pub fn find_coldest_route(heatmap: &HeatMap) -> u32 {
    let mut cloud: HashMap<Pos, u32> = HashMap::new();
    cloud.insert(Pos::ZERO, 0);

    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(Pos::ZERO + Dir::RIGHT);
    queue.push_back(Pos::ZERO + Dir::DOWN);

    while let Some(pos) = queue.pop_front() {}

    0
}

pub fn find_coldest_route_2(heatmap: &HeatMap, route: Route, max_visited: Pos) -> u32 {
    lazy_static! {
        static ref CACHE: Mutex<HashMap<Route, u32>> = Mutex::new(HashMap::new());
        static ref GLOBAL_MAX: Mutex<u32> = Mutex::new(u32::MAX);
    }

    {
        let cache = CACHE.lock().unwrap();
        if let Some(&cached) = cache.get(&route) {
            return cached;
        }
    }

    debug!("{}", route.print());
    if !heatmap.bounds.contains(route.pos) {
        return u32::MAX;
    }

    // Heuristic
    if max_visited.x > route.pos.x + 2 || max_visited.y > route.pos.y + 2 {
        return u32::MAX;
    }

    let next_max_visited = Pos {
        x: cmp::max(max_visited.x, route.pos.x),
        y: cmp::max(max_visited.y, route.pos.y),
    };
    let cell_temp = heatmap.get(route.pos).unwrap().0;
    if route.pos == heatmap.bounds.max {
        debug!("Finished, found route");
        return cell_temp;
    }

    let mut min = u32::MAX;
    if route.count > 0 {
        min = cmp::min(
            min,
            find_coldest_route(heatmap, route.forward(), next_max_visited),
        );
    }

    for tangent_dir in tangent_dirs(route.dir) {
        min = cmp::min(
            min,
            find_coldest_route(heatmap, route.rotated(tangent_dir), next_max_visited),
        );
    }

    let mut cache = CACHE.lock().unwrap();
    cache.insert(route, min + cell_temp);
    min + cell_temp
}

pub fn find_coldest(input: &str) -> u32 {
    let heatmap = parse_heatmap(input);

    find_coldest_route(
        &heatmap,
        Route {
            pos: Pos::ZERO,
            dir: Dir::RIGHT, // Other direction are taken by tangent_dirs
            count: 2,
        },
        Pos::ZERO,
    )
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(find_coldest(input), 102);
    }
}
