use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

pub fn bfs_count<P: Hash + Eq + Copy>(
    initial: P,
    get_adjacents: impl Fn(P) -> Vec<P>,
    end_found: impl Fn(P) -> bool,
) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((initial, 0));
    visited.insert(initial);

    while let Some((pos, len)) = queue.pop_front() {
        // println!("Visiting {:?} (len: {}) end={:?}", pos, len, graph.end);
        if end_found(pos) {
            return Some(len);
        }

        for neighbour in get_adjacents(pos) {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                queue.push_back((neighbour, len + 1));
            }
        }
    }

    None
}

pub fn bfs_cache<P: Hash + Eq + Copy>(
    initial: P,
    get_adjacents: impl Fn(P) -> Vec<P>,
) -> HashMap<P, usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut cache = HashMap::new();
    queue.push_back((initial, 0));
    visited.insert(initial);

    while let Some((pos, len)) = queue.pop_front() {
        for neighbour in get_adjacents(pos) {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                cache.insert(neighbour, len + 1);
                queue.push_back((neighbour, len + 1));
            }
        }
    }

    cache
}
