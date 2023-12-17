use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

use toolkit::key::Key;

pub fn part1() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    pub key: Key,
    pub left: Key,
    pub right: Key,
}

pub struct Graph {
    pub nodes: HashMap<Key, Node>,
}

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

impl Instruction {
    pub fn parse(c: char) -> Instruction {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        }
    }
}

pub fn parse_map(input: &str) -> (Vec<Instruction>, Graph) {
    lazy_static! {
        static ref NODE_RE: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    }

    let (instructions_data, graph_nodes) = input.split_once("\n\n").unwrap();

    let instructions = instructions_data
        .chars()
        .map(Instruction::parse)
        .collect_vec();

    let mut nodes = HashMap::new();
    for line in graph_nodes.lines() {
        let captures = NODE_RE.captures(line).unwrap();
        // println!("Captures: {:?}", captures);
        let key = captures[1].parse().unwrap();
        let left = captures[2].parse().unwrap();
        let right = captures[3].parse().unwrap();

        let node = Node { key, left, right };
        nodes.insert(key, node);
    }

    (instructions, Graph { nodes })
}

pub fn count_steps_exit(input: &str) -> u32 {
    let (instructions, graph) = parse_map(input);

    let mut current_node = graph.nodes[&Key::from_str("AAA").unwrap()];
    let mut steps = 0;

    loop {
        for instruction in instructions.iter() {
            println!("Instruction: {:?}", instruction);
            match instruction {
                Instruction::Left => current_node = graph.nodes[&current_node.left],
                Instruction::Right => current_node = graph.nodes[&current_node.right],
            }
            steps += 1;

            if current_node.key == "ZZZ" {
                return steps;
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test1.txt");
        assert_eq!(count_steps_exit(input), 2);
    }

    #[test]
    fn test_example2() {
        let input = include_str!("../test2.txt");
        assert_eq!(count_steps_exit(input), 6);
    }
}
