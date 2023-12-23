use std::collections::HashMap;

use toolkit::key::Key;

use crate::part1::{parse_modules, run_cycle, Module, State};

pub fn find_min_rx_cycles(input: &str) -> u64 {
    let modules = parse_modules(input);
    let mut state = State::from_modules(&modules);
    let original_state = state.clone();
    original_state.print();

    let mut hashes: HashMap<State, u64> = HashMap::new();
    hashes.insert(state.clone(), 0);

    let mut i = 0;
    loop {
        if i % 100_000 == 0 {
            println!("i: {}", i);
            state.print();
        }
        if state.rx_received_low {
            println!("RX received low at {}", i);
            break;
        }

        run_cycle(&mut state, &modules);
        if let Some(prev) = hashes.get(&state) {
            println!("Cycle detected at {}, prev={}", i, prev);
            break;
        }

        i += 1;
        hashes.insert(state.clone(), i);
    }

    i
}

pub fn get_inputs(modules: &HashMap<Key, Module>) -> HashMap<Key, Vec<Key>> {
    let mut inputs: HashMap<Key, Vec<Key>> = HashMap::new();

    modules.iter().for_each(|(key, module)| {
        module.outputs.iter().for_each(|output| {
            inputs.entry(*output).or_default().push(*key);
        });
    });

    inputs
}

#[cfg(test)]
pub mod tests {
    use std::collections::{HashSet, VecDeque};

    use itertools::Itertools;
    use toolkit::key;

    use crate::part2::*;

    #[test]
    fn print_tree() {
        let input = include_str!("../input.txt");
        let modules = parse_modules(input);
        let mut state = State::from_modules(&modules);
        let inputs = get_inputs(&modules);
        let mut queue: VecDeque<(u32, Vec<Key>)> = VecDeque::new();
        queue.push_back((0, vec![Key::new("rx")]));

        let mut visited: HashSet<Key> = HashSet::new();

        while let Some((indent, mods)) = queue.pop_front() {
            for key in mods {
                if visited.contains(&key) {
                    println!("{}{}: {}", " ".repeat(indent as usize), key, "(visited)");
                    continue;
                }
                visited.insert(key);
                let button_default_input = vec![];
                let mod_inputs = inputs.get(&key).unwrap_or(&button_default_input);
                let mod_str = mod_inputs.iter().map(|m| m.to_string()).join(", ");
                println!("{}{}: {}", " ".repeat(indent as usize), key, mod_str);
                if mod_inputs.is_empty() {
                    continue;
                }

                let next = (indent + 1, mod_inputs.clone());
                queue.push_front(next);
            }
        }
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        // assert_eq!(1, 2);
    }
}
