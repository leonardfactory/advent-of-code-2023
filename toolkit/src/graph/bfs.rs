use std::{
    collections::{HashSet, VecDeque},
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
