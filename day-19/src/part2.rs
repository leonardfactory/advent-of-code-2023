use colored::Colorize;
use std::{cmp, fmt::Display};

use toolkit::{debug, key::Key};

use crate::part1::{parse_workflows, Branch, Compare, Rule, WorkflowMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

impl Default for Range {
    fn default() -> Self {
        Self { min: 1, max: 4000 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Solution {
    pub ranges: [Range; 4],
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x={}..{}), (m={}..{}), (a={}..{}), (s={}..{})",
            self.ranges[0].min,
            self.ranges[0].max,
            self.ranges[1].min,
            self.ranges[1].max,
            self.ranges[2].min,
            self.ranges[2].max,
            self.ranges[3].min,
            self.ranges[3].max,
        )
    }
}

pub fn count_acceptable(input: &str) -> u64 {
    let (workflow_map, _) = parse_workflows(input);

    solve_range(
        &workflow_map,
        Solution {
            ranges: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
        },
        Branch::Jump("in".parse::<Key>().unwrap()),
    )
}

pub fn solve_range(workflow_map: &WorkflowMap, solution: Solution, branch: Branch) -> u64 {
    debug!("\nSolving: {} => {}", solution, branch.to_string().cyan());
    match branch {
        Branch::Accept => {
            let count = solution.ranges.iter().fold(1, |acc, range| {
                acc * (cmp::max(range.max as i32 - range.min as i32 + 1, 0) as u64)
            });
            debug!("Found solution: {} => {}", solution, count);
            count
        }
        Branch::Reject => 0,
        Branch::Jump(branch) => {
            let workflow = workflow_map.get(&branch).unwrap();

            let mut count = 0;
            let mut remaining = solution;
            for rule in workflow.rules.iter() {
                let mut narrowed = remaining;
                match rule {
                    Rule::If(condition, branch) => {
                        match condition.compare {
                            Compare::Gt => {
                                narrowed.ranges[condition.prop as usize].min = condition.value + 1;

                                remaining.ranges[condition.prop as usize].max = condition.value;
                            }
                            Compare::Lt => {
                                narrowed.ranges[condition.prop as usize].max = condition.value - 1;

                                remaining.ranges[condition.prop as usize].min = condition.value;
                            }
                        }
                        count += solve_range(workflow_map, narrowed, *branch)
                    }
                    Rule::Else(branch) => {
                        count += solve_range(workflow_map, remaining, *branch);
                    }
                }
            }
            count
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_acceptable(input), 167409079868000);
    }
}
