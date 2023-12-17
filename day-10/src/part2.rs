use std::collections::{HashMap, HashSet, VecDeque};

use colored::Colorize;
use itertools::Itertools;
use toolkit::map::{Pos, TileDisplay};

use crate::part1::{get_start_pos, get_starting_pipes, matching_pipe, parse_map, Map, Pipe, Tile};

fn resolve_starting_pipe(map: &mut Map, start_pos: Pos, pipe_start: Pos, pipe_end: Pos) {
    map.tiles.entry(start_pos).and_modify(|start_tile| {
        *start_tile = Tile::Pipe(Pipe {
            start: pipe_start,
            end: pipe_end,
            connected: true,
        })
    });
}

pub fn count_inside_loop(input: &str) -> u32 {
    let mut map = parse_map(input);
    let start_pos = get_start_pos(&map);
    let (start_cw, start_ccw) = get_starting_pipes(&map, start_pos);

    // Fix starting pipes
    resolve_starting_pipe(&mut map, start_pos, start_cw.0, start_ccw.0);

    let mut pipe_loop: HashSet<Pos> = HashSet::new();

    map.print();

    let mut current = start_pos;
    loop {
        let tile = map.get(current).unwrap();

        match tile {
            Tile::Pipe(pipe) => {
                let targets = vec![pipe.start, pipe.end];
                let target = targets.iter().find(|target| !pipe_loop.contains(target));

                match target {
                    Some(target) => {
                        pipe_loop.insert(*target);
                        current = *target;
                    }
                    None => break,
                }
            }
            _ => panic!("Invalid tile: {:?}", tile),
        }
    }

    map.print_with(|tile, pos| {
        if pipe_loop.contains(&pos) {
            Box::new(tile.map_print(pos).to_string().on_color("blue"))
        } else {
            tile.map_print(pos)
        }
    });

    let mut inside_list: HashSet<Pos> = HashSet::new();
    let mut outside_list: HashSet<Pos> = HashSet::new();

    for y in map.bounds.min.y..map.bounds.max.y {
        let mut is_in_loop = false;
        let mut latest_corner: char = '0';

        for x in map.bounds.min.x..map.bounds.max.x {
            let pos = Pos::new(x, y);
            let tile = map.get(pos).unwrap();

            match *tile {
                Tile::Pipe(pipe) if pipe_loop.contains(&pos) => match pipe.shape(pos) {
                    '-' => {}
                    '|' => is_in_loop = !is_in_loop,
                    'L' => {
                        latest_corner = 'L';
                    }
                    'F' => {
                        latest_corner = 'F';
                    }
                    'J' => {
                        if latest_corner == 'F' {
                            is_in_loop = !is_in_loop;
                        }
                        latest_corner = '0';
                    }
                    '7' => {
                        if latest_corner == 'L' {
                            is_in_loop = !is_in_loop;
                        }
                        latest_corner = '0';
                    }
                    _ => {}
                },
                _ if is_in_loop => {
                    if pipe_loop.contains(&pos) {
                        panic!("Already in loop: {:?}", pos);
                    }
                    // println!("Found at {:?}", pos);
                    // map.print_and_highlight(pos);
                    inside_list.insert(pos);
                }
                _ => {}
            }
        }

        // Remove the latest bits (from the right)
        for x in (map.bounds.min.x..map.bounds.max.x).rev() {
            let pos = Pos::new(x, y);
            let tile = map.get(pos).unwrap();
            match *tile {
                Tile::Pipe(_pipe) if pipe_loop.contains(&pos) => break,
                _ => {
                    inside_list.remove(&pos);
                }
            }
        }
    }

    // pipe_loop.iter().for_each(|loop_pos| {
    //     let mut current_pos = *loop_pos;
    //     let mut current_tile = map.get(current_pos).unwrap();

    //     let neighbors = map.neighbors(*loop_pos);
    //     let visitable_neighbors = neighbors
    //         .iter()
    //         .filter(|(pos, _tile)| !pipe_loop.contains(pos))
    //         .collect_vec();

    //     let blobs = visitable_neighbors
    //         .iter()
    //         .filter_map(|&&near_pos| {
    //             let mut queue = VecDeque::from(vec![near_pos]);
    //             let mut blob: HashSet<Pos> = HashSet::new();
    //             while let Some((pos, queue_tile)) = queue.pop_front() {
    //                 if map.is_near_bounds(pos) {
    //                     println!("Discard blob: near bounds");
    //                     return None;
    //                 }

    //                 match queue_tile {
    //                     Tile::Pipe(pipe) if pipe_loop.contains(&pos) => {}
    //                     _ => {
    //                         blob.insert(pos);
    //                         let neighbors = map.neighbors(pos);
    //                         neighbors
    //                             .iter()
    //                             .filter(|(pos, _tile)| {
    //                                 !pipe_loop.contains(pos) && !blob.contains(pos)
    //                             })
    //                             .for_each(|&(pos, tile)| queue.push_front((pos, tile)));
    //                     }
    //                 }
    //             }
    //             Some(blob)
    //         })
    //         .collect_vec();

    //     blobs.iter().for_each(|blob| {
    //         blob.iter().for_each(|pos| {
    //             inside_list.insert(*pos);
    //         });
    //     })
    // });

    map.print_with(|tile, pos| {
        if inside_list.contains(&pos) {
            Box::new(tile.map_print(pos).to_string().on_bright_green())
        } else if pipe_loop.contains(&pos) {
            Box::new(tile.map_print(pos).to_string().on_yellow())
        } else {
            tile.map_print(pos)
        }
    });

    // 1218 too high
    // 498 too high
    inside_list.len() as u32
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test2.txt");
        assert_eq!(count_inside_loop(input), 4);
    }

    #[test]
    fn test_example_10() {
        let input = include_str!("../test3.txt");
        assert_eq!(count_inside_loop(input), 10);
    }

    #[test]
    fn test_example_4() {
        let input = include_str!("../test4.txt");
        assert_eq!(count_inside_loop(input), 8);
    }
}
