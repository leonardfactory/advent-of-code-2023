use colored::Colorize;
use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use toolkit::debug;

use crate::part1::{parse_springs, Row, Spring};

impl Row {
    fn unfold(&self) -> Self {
        let mut springs = Vec::new();
        let mut groups = Vec::new();

        for i in 0..5 {
            if i > 0 {
                springs.push(Spring::Unknown);
            }

            springs.extend(self.springs.clone());
            groups.extend(self.groups.clone());
        }

        Self {
            springs,
            groups,
            fold_len: self.fold_len,
            fold_groups_len: self.fold_groups_len,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Solution<'a> {
    pub springs: Vec<Spring>,
    pub unknow_index: usize,
    pub row: &'a Row,
}

impl<'a> Solution<'a> {
    fn from_row(row: &'a Row) -> Self {
        Self {
            springs: row.springs.clone(),
            unknow_index: 0,
            row,
        }
    }

    pub fn is_solved(&self) -> bool {
        self.springs
            .iter()
            .filter(|s| s == &&Spring::Unknown)
            .count()
            == 0
    }

    pub fn to_str(&self) -> String {
        self.springs
            .iter()
            .map(|s| match s {
                Spring::Broken => '#',
                Spring::Operative => '.',
                Spring::Unknown => '?',
            })
            .collect()
    }

    pub fn is_impossible(&self) -> bool {
        let mut computed_groups = Vec::new();
        let mut current_group = 0;
        let mut is_broken = false;
        let latest_group_len = self.row.groups.last().unwrap();
        //  ....?....?....
        for (i, spring) in self.springs.iter().enumerate() {
            // let group_index = i / (self.row.fold_len + 1);
            // let group_index = (i - usize::from(i > 0) * group_index) / self.row.fold_len;

            // if computed_groups.len() > self.row.fold_groups_len * (group_index + 1) {
            //     println!(" -> Solution is impossible: {}", self.to_str());
            //     return true;
            // }

            match spring {
                Spring::Broken => {
                    is_broken = true;
                    current_group += 1;
                }
                Spring::Operative => {
                    if is_broken {
                        computed_groups.push(current_group);
                        current_group = 0;
                        is_broken = false;
                    }

                    // If we have a computed group that doesn't match the
                    // expected group even partially, then we know this solution
                    // is _impossible_.
                    if computed_groups.len() > self.row.groups.len() {
                        // println!(" -> Solution is impossible: {}", self.to_str());
                        return true;
                    }
                    if computed_groups != self.row.groups[..computed_groups.len()] {
                        // println!(" -> Solution is impossible: {}", self.to_str());
                        return true;
                    }
                }
                // If we have an unknown spring, we can't know if it's broken
                // or not so assume the solution is still _possible_.
                Spring::Unknown => return false,
            }
        }

        if is_broken {
            computed_groups.push(current_group);
        }

        computed_groups != self.row.groups
    }
}

pub fn solve_backtracking(row: &Row) -> u64 {
    let mut solutions = VecDeque::new();
    solutions.push_back(Solution::from_row(row));
    let mut count = 0;

    while let Some(solution) = solutions.pop_front() {
        if solution.is_impossible() {
            continue;
        }

        if solution.is_solved() {
            // println!(" -> Solution found: {}", solution.to_str());
            count += 1;
            continue;
        }

        let next_unknown = solution.springs.iter().position(|s| s == &Spring::Unknown);
        for char in ['#', '.'] {
            let mut next_solution = solution.clone();

            match next_unknown {
                Some(index) => {
                    next_solution.unknow_index = index;
                    next_solution.springs[index] = match char {
                        '#' => Spring::Broken,
                        '.' => Spring::Operative,
                        _ => panic!("Unknown spring"),
                    };

                    solutions.push_back(next_solution);
                }
                None => continue,
            }
        }
    }

    count
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PartialRow {
    springs: Vec<Spring>,
    groups: Vec<usize>,
    in_broken: bool,
    broken_count: usize,
}

pub fn solve_partial(row: &PartialRow) -> u64 {
    lazy_static! {
        static ref CACHE: Mutex<HashMap<PartialRow, u64>> = Mutex::new(HashMap::new());
    }

    {
        let cache = CACHE.lock().unwrap();
        if cache.contains_key(row) {
            return *cache.get(row).unwrap();
        }
    }
    debug!(
        " -> Solving: {} g={:?} bc={}",
        row.springs.iter().join(""),
        row.groups,
        row.broken_count
    );
    let mut next_row = row.clone();
    if !next_row.springs.is_empty() {
        next_row.springs.remove(0);
    }

    let result = match row.springs.first() {
        // .. # -> Ok, non siamo in `in_broken` e abbiamo trovato un broken. Se groups è vuoto, allora è impossibile.
        // #. # -> Non possibile
        Some(Spring::Broken) if row.groups.is_empty() && !row.in_broken => {
            debug!("  -> impossible, broken without group");
            0
        }
        // #### 3 -> Siamo qui, ma c'è un broken in più. impossibile
        Some(Spring::Broken) if row.broken_count == row.groups[0] => {
            debug!(
                "  -> impossible, too many broken: already {}, expected {}",
                row.broken_count, row.groups[0]
            );
            0
        }
        Some(Spring::Broken) => {
            next_row.in_broken = true;
            next_row.broken_count += 1;
            solve_partial(&next_row)
        }
        Some(Spring::Operative) if row.in_broken && row.broken_count != row.groups[0] => 0,
        Some(Spring::Operative) => {
            if row.in_broken {
                next_row.groups.remove(0);
            }

            next_row.broken_count = 0;
            next_row.in_broken = false;
            solve_partial(&next_row)
        }
        Some(Spring::Unknown) => {
            let mut result = 0;

            let mut broken_row = next_row.clone();
            broken_row.springs.insert(0, Spring::Broken);
            result += solve_partial(&broken_row);

            let mut operative_row = next_row.clone();
            operative_row.springs.insert(0, Spring::Operative);
            result += solve_partial(&operative_row);

            result
        }
        None => u64::from(
            (row.groups.len() == 1 && row.broken_count == row.groups[0]) || row.groups.is_empty(),
        ),
    };

    let mut cache = CACHE.lock().unwrap();
    cache.insert(row.clone(), result);
    debug!(" -> Solving: {} = {}", row.springs.iter().join(""), result);
    result
}

fn solve_cached(row: &Row) -> u64 {
    debug!(
        "\n\n{} Solving: {}, groups: {:?}",
        "[BEGIN]".cyan(),
        row.springs.iter().join(""),
        row.groups
    );
    solve_partial(&PartialRow {
        springs: row.springs.clone(),
        groups: row.groups.clone(),
        broken_count: 0,
        in_broken: false,
    }) as u64
}

pub fn solve_cached_str(row: &str) -> u64 {
    let row = &parse_springs(row)[0];
    solve_cached(row)
}

pub fn solve_backtracking_str(row: &str) -> u64 {
    let row = &parse_springs(row)[0];
    solve_backtracking(row) as u64
}

pub fn solve_folded_springs(input: &str) -> u64 {
    let rows = parse_springs(input)
        .iter()
        .map(|r| r.unfold())
        .collect_vec();

    // too low 78849875086
    rows.iter().map(solve_cached).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    // #[test]
    // fn test_solve_backtrack() {
    //     let rows = parse_springs("???.### 1,1,3\n.??..??...?##. 1,1,3");
    //     assert_eq!(solve_backtracking(&rows[0]), 1);
    //     assert_eq!(solve_backtracking(&rows[1]), 4);

    //     let rows2 = parse_springs("?###???????? 3,2,1");
    //     assert_eq!(solve_backtracking(&rows2[0]), 10);
    // }

    #[test]
    fn test_solve_simple() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(solve_folded_springs(input), 2500);
        let input = "???.### 1,1,3";
        assert_eq!(solve_folded_springs(input), 1);
    }

    #[test]
    fn test_solve_sample1() {
        let input = "?###???????? 3,2,1";
        assert_eq!(solve_folded_springs(input), 506250);
    }

    #[test]
    fn test_final_only_one() {
        let row = &parse_springs("?## 3")[0];
        assert_eq!(solve_cached(row), 1);
        let row = &parse_springs("?##. 3")[0];
        assert_eq!(solve_cached(row), 1);
    }

    #[test]
    fn test_solve_folded_springs() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(solve_folded_springs(input), 16384);
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(solve_folded_springs(input), 1);
        let input = "????.#...#... 4,1,1";
        assert_eq!(solve_folded_springs(input), 16);
    }

    #[test]
    fn test_input_backtracking() {
        let input = ".?.??#?##??#????.. 8,1";
        assert_eq!(solve_folded_springs(input), 55301); // 12005
        let input = "#?#???????#?.? 3,1,2,2";
        assert_eq!(solve_folded_springs(input), 58564);
        let input = "..?????#.. 1,1";
        assert_eq!(solve_folded_springs(input), 23184);
        let input = "..?????#.. 1,1";
        assert_eq!(solve_cached_str(input), solve_backtracking_str(input));
        let row = parse_springs("..?????#.. 1,1")[0].unfold();
        assert_eq!(solve_cached(&row), solve_backtracking(&row));
    }

    #[test]
    fn test_test_part1() {
        let input = include_str!("../test.txt");
        let rows = parse_springs(input);
        assert_eq!(solve_cached(&rows[0]), 1);
        assert_eq!(solve_cached(&rows[1]), 4);
        assert_eq!(solve_cached(&rows[2]), 1);
        assert_eq!(solve_cached(&rows[3]), 1);
        assert_eq!(solve_cached(&rows[4]), 4);
        assert_eq!(solve_cached(&rows[5]), 10);
    }

    #[test]
    fn test_input_part1() {
        let input = include_str!("../input.txt");
        let rows = parse_springs(input);
        let sum = rows.iter().map(solve_cached).sum::<u64>();
        assert_eq!(sum, 7007);
    }

    #[test]
    fn test_edge_cases() {
        let input = "? 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = "#? 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = ".? 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = "#. 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = ".# 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = "?. 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = "?# 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = "?#? 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = "?.? 1";
        assert_eq!(solve_cached_str(input), 2);
        let input = "?#? 1";
        assert_eq!(solve_cached_str(input), 1);
        let input = "??? 3";
        assert_eq!(solve_cached_str(input), 1);
        let input = "??? 1";
        assert_eq!(solve_cached_str(input), 3);

        let input = ".....?.? 1";
        assert_eq!(solve_cached_str(input), 2);
        let input = "#....?.? 1,1";
        assert_eq!(solve_cached_str(input), 2);
        let input = "?#. 1";
        assert_eq!(solve_cached_str(input), 1);
    }

    #[test]
    fn test_edge_cases2() {
        let input = "###? 3";
        assert_eq!(solve_cached_str(input), 1);
        let input = "##### 5";
        assert_eq!(solve_cached_str(input), 1);
        let input = "#####. 5";
        assert_eq!(solve_cached_str(input), 1);
        let input = "?#####. 6";
        assert_eq!(solve_cached_str(input), 1);
        let input = "#####? 6";
        assert_eq!(solve_cached_str(input), 1);
        let input = "#####?. 6";
        assert_eq!(solve_cached_str(input), 1);
        let input = "#####?? 6";
        assert_eq!(solve_cached_str(input), 1);
        let input = "?.#####?? 1,6";
        assert_eq!(solve_cached_str(input), 1);
        let input = "?.?#####? 1,6";
        assert_eq!(solve_cached_str(input), 2);
        let input = "?.?##### 1,6";
        assert_eq!(solve_cached_str(input), 1);
    }

    // #[test]
    // fn test_compute_more() {
    //     let row = parse_springs("..?????#.. 1,1")[0].unfold();
    //     assert_eq!(solve_backtracking(&row), 0);
    // }
    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(solve_folded_springs(input), 525152);
    }
}
