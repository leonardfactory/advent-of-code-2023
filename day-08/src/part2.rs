use std::collections::HashSet;

use itertools::Itertools;
use num::integer;

use crate::part1::{parse_map, Instruction};

pub fn run_ghosts_paths(input: &str) -> u64 {
    let (instructions, graph) = parse_map(input);

    let current_nodes: Vec<_> = graph
        .nodes
        .keys()
        .filter(|k| k.to_string().ends_with('A'))
        .collect();

    let ending_keys: HashSet<_> = graph
        .nodes
        .keys()
        .filter(|k| k.to_string().ends_with('Z'))
        .collect();

    let patterns = current_nodes
        .iter()
        .map(|k| {
            let mut pattern_steps = 0;
            let mut current_node = &graph.nodes[k];
            loop {
                for instruction in instructions.iter() {
                    match instruction {
                        Instruction::Left => current_node = &graph.nodes[&current_node.left],
                        Instruction::Right => current_node = &graph.nodes[&current_node.right],
                    }
                    pattern_steps += 1;

                    if ending_keys.contains(&current_node.key) {
                        return pattern_steps;
                    }
                }
            }
        })
        .collect_vec();

    let lcm: u64 = patterns.iter().fold(1, |acc, x| integer::lcm(acc, *x));
    lcm
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test3.txt");
        assert_eq!(run_ghosts_paths(input), 6);
    }
}
