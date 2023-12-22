use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
};
use toolkit::{debug, key::Key};

pub fn part1() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Part {
    pub props: [u32; 4],
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Part(x={})", self.props[0])
    }
}

impl Index<Prop> for Part {
    type Output = u32;

    fn index(&self, index: Prop) -> &Self::Output {
        &self.props[index as usize]
    }
}

impl IndexMut<Prop> for Part {
    fn index_mut(&mut self, index: Prop) -> &mut Self::Output {
        &mut self.props[index as usize]
    }
}

impl Part {
    pub fn parse(s: &str) -> Self {
        let mut part = Self { props: [0; 4] };

        let assignments = s[1..s.len() - 1].split(',');
        assignments.for_each(|a| {
            let (prop, value) = a.split_once('=').unwrap();

            let prop = Prop::from_char(prop.chars().next().unwrap());
            part[prop] = value.parse::<u32>().unwrap();
        });

        part
    }

    pub fn sum(&self) -> u32 {
        self.props.iter().sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prop {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl Prop {
    pub fn from_char(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("Invalid prop: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Compare {
    Gt,
    Lt,
}

impl Compare {
    pub fn from_char(c: char) -> Self {
        match c {
            '>' => Self::Gt,
            '<' => Self::Lt,
            _ => panic!("Invalid compare: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Branch {
    Reject,
    Accept,
    Jump(Key),
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Reject => write!(f, "Reject"),
            Branch::Accept => write!(f, "Accept"),
            Branch::Jump(key) => write!(f, "Jump({})", key),
        }
    }
}

impl Branch {
    pub fn from_key(key: Key) -> Self {
        match key.to_string().as_str() {
            "R" => Self::Reject,
            "A" => Self::Accept,
            _ => Self::Jump(key),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rule {
    If(Condition, Branch),
    Else(Branch),
}

impl Rule {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(r"(\w+)([<>]{1})(\d+):(\w+)").unwrap();
        }

        match input.split_once(':') {
            Some((_, _)) => {
                let captures = RULE_RE.captures(input).unwrap();
                let prop = Prop::from_char(captures[1].chars().next().unwrap());
                let compare = Compare::from_char(captures[2].chars().next().unwrap());
                let value = captures[3].parse::<u32>().unwrap();
                let branch = captures[4].parse::<Key>().unwrap();

                Self::If(
                    Condition {
                        prop,
                        compare,
                        value,
                    },
                    Branch::from_key(branch),
                )
            }
            None => Self::Else(Branch::from_key(input.parse::<Key>().unwrap())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Condition {
    pub prop: Prop,
    pub compare: Compare,
    pub value: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Workflow {
    pub name: Key,
    pub rules: Vec<Rule>,
}

impl Workflow {
    pub fn parse(input: &str) -> Self {
        let (key, rules) = input[0..input.len() - 1].split_once('{').unwrap();
        let key = key.parse::<Key>().unwrap();
        let rules = rules.split(',').map(Rule::parse).collect();

        Self { name: key, rules }
    }

    pub fn solve(&self, part: &Part) -> Branch {
        let found_rule = self
            .rules
            .iter()
            .find(|rule| match rule {
                Rule::If(condition, _) => match condition.compare {
                    Compare::Gt => part[condition.prop] > condition.value,
                    Compare::Lt => part[condition.prop] < condition.value,
                },
                Rule::Else(_) => true,
            })
            .unwrap();

        match found_rule {
            Rule::If(_, branch) => *branch,
            Rule::Else(branch) => *branch,
        }
    }
}

pub struct WorkflowMap {
    map: HashMap<Key, Workflow>,
}

impl WorkflowMap {
    pub fn get(&self, key: &Key) -> Option<&Workflow> {
        self.map.get(key)
    }

    pub fn is_accepted(&self, part: &Part) -> bool {
        debug!("\nChecking part: {}", part);
        let mut workflow = &self.map[&"in".parse().unwrap()];
        loop {
            debug!(" [{}] ", workflow.name.to_string());
            match workflow.solve(part) {
                Branch::Reject => {
                    debug!("Rejected part: {:?}", part);
                    return false;
                }
                Branch::Accept => return true,
                Branch::Jump(key) => workflow = &self.map[&key],
            }
        }
    }
}

pub fn parse_workflows(input: &str) -> (WorkflowMap, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows.lines().map(Workflow::parse).collect_vec();

    let workflow_map = WorkflowMap {
        map: workflows.into_iter().map(|w| (w.name, w)).collect(),
    };

    let parts = parts.lines().map(Part::parse).collect();

    (workflow_map, parts)
}

pub fn count_accepted_parts(input: &str) -> u32 {
    let (workflows, parts) = parse_workflows(input);

    parts
        .iter()
        .filter(|p| workflows.is_accepted(p))
        .map(|p| p.sum())
        .sum::<u32>()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parse_part() {
        let part = Part::parse("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!(part[Prop::X], 787);
        assert_eq!(part[Prop::M], 2655);
        assert_eq!(part[Prop::A], 1222);
        assert_eq!(part[Prop::S], 2876);

        assert_eq!(part.sum(), 7540);
    }

    #[test]
    fn test_parse_workflow() {
        let workflow = Workflow::parse("px{a<2006:qkq,m>2090:A,rfg}");
        assert_eq!(workflow.name, "px".parse::<Key>().unwrap());
        assert_eq!(workflow.rules.len(), 3);
        assert_eq!(
            workflow.rules[0],
            Rule::If(
                Condition {
                    prop: Prop::A,
                    compare: Compare::Lt,
                    value: 2006,
                },
                Branch::Jump("qkq".parse().unwrap()),
            )
        );
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_accepted_parts(input), 19114);
    }
}
