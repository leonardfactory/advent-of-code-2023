use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use toolkit::{graph::bfs_cache, map::Pos};

use crate::part1::{expand_universe, parse_universe, Map, Tile};

pub fn universe_bfs_cache(map: &Map, pos: Pos, expansion: u64) -> HashMap<Pos, u64> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut cache = HashMap::new();
    queue.push_back((pos, 0));
    visited.insert(pos);

    while let Some((pos, len)) = queue.pop_front() {
        for (neighbour, tile) in map.neighbors(pos) {
            let direction = neighbour - pos;
            let distance = match tile {
                Tile::Empty => {
                    #[allow(clippy::if_same_then_else)]
                    if map.iter_column(neighbour.x).all(|(_, t)| t == &Tile::Empty)
                        && (direction == Pos::LEFT || direction == Pos::RIGHT)
                    {
                        expansion
                    } else if map.iter_row(neighbour.y).all(|(_, t)| t == &Tile::Empty)
                        && (direction == Pos::UP || direction == Pos::DOWN)
                    {
                        expansion
                    } else {
                        1
                    }
                }
                Tile::Galaxy(_) => 1,
            };
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                cache.insert(neighbour, len + distance);
                queue.push_back((neighbour, len + distance));
            }
        }
    }

    cache
}

pub fn count_old_distances(input: &str, expansion: u64) -> u64 {
    let map = parse_universe(input);
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

    let distance_cache: HashMap<Pos, HashMap<Pos, u64>> = galaxies
        .iter()
        .map(|&(pos, galaxy_index)| (pos, universe_bfs_cache(&map, pos, expansion)))
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
                .unwrap() as u64
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_old_distances(input, 10), 1030);
        assert_eq!(count_old_distances(input, 100), 8410);
    }
}
