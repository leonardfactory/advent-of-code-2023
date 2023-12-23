use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use toolkit::{debug, key::Key};

pub enum Kind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

pub struct Module {
    pub key: Key,
    pub kind: Kind,
    pub outputs: Vec<Key>,
    pub inputs: Vec<Key>,
}

pub fn parse_modules(input: &str) -> HashMap<Key, Module> {
    let mut hashmap: HashMap<Key, Module> = input
        .lines()
        .map(|line| {
            let (desc, outputs) = line.split_once(" -> ").unwrap();
            let outputs = outputs
                .split(", ")
                .map(|s| s.parse::<Key>().unwrap())
                .collect_vec();

            let module = match desc.chars().next().unwrap() {
                '&' => Module {
                    key: desc[1..].parse::<Key>().unwrap(),
                    kind: Kind::Conjunction,
                    outputs,
                    inputs: vec![], // We'll fill this in later
                },
                '%' => Module {
                    key: desc[1..].parse::<Key>().unwrap(),
                    kind: Kind::FlipFlop,
                    outputs,
                    inputs: vec![],
                },
                _ => Module {
                    key: desc.parse::<Key>().unwrap(),
                    kind: Kind::Broadcaster,
                    outputs,
                    inputs: vec![],
                },
            };

            (module.key, module)
        })
        .collect();

    let mut inputs: HashMap<Key, Vec<Key>> = HashMap::new();

    hashmap.iter().for_each(|(key, module)| {
        module.outputs.iter().for_each(|output| {
            inputs.entry(*output).or_default().push(*key);
        });
    });

    inputs.iter().for_each(|(key, inputs)| {
        println!("{} -> {:?}", key, inputs);
        if !hashmap.contains_key(key) {
            return;
        }

        let module = hashmap.get_mut(key).unwrap();
        if let Kind::Conjunction = module.kind {
            module.inputs = inputs.clone();
        }
    });

    hashmap
}

#[derive(Debug, Clone, Default, Eq)]
pub struct State {
    pub flipflop: HashMap<Key, bool>,
    pub conjunction: HashMap<Key, HashMap<Key, bool>>,
    pub rx_received_low: bool,
    pub counts_low: u64,
    pub counts_high: u64,
}

impl State {
    pub fn from_modules(modules: &HashMap<Key, Module>) -> Self {
        let mut state = Self::default();

        modules.iter().for_each(|(key, module)| match module.kind {
            Kind::Broadcaster => (),
            Kind::FlipFlop => {
                state.flipflop.insert(*key, false);
            }
            Kind::Conjunction => {
                state
                    .conjunction
                    .insert(*key, module.inputs.iter().map(|k| (*k, false)).collect());
            }
        });

        state
    }

    pub fn is_reset(&self) -> bool {
        self.flipflop.values().all(|v| !*v)
            && self.conjunction.values().all(|v| v.values().all(|v| !*v))
    }

    pub fn print(&self) {
        println!("\n\nFlipFlop:");
        self.flipflop.iter().for_each(|(key, value)| {
            println!("  {} = {}", key, value);
        });

        println!("\nConjunction:");
        self.conjunction.iter().for_each(|(key, value)| {
            println!(
                "  {} => {:?}",
                key,
                value
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect_vec()
            );
        });
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.flipflop == other.flipflop && self.conjunction == other.conjunction
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.flipflop.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state);
        });
        self.conjunction.iter().for_each(|(k, v)| {
            k.hash(state);
            v.iter().for_each(|(k, v)| {
                k.hash(state);
                v.hash(state);
            });
        });
    }
}

pub fn run_cycles(input: &str, count: usize) -> u64 {
    let modules = parse_modules(input);
    let mut state = State::from_modules(&modules);

    for _ in 0..count {
        run_cycle(&mut state, &modules);
    }

    state.counts_high * state.counts_low
}

pub fn run_cycle(state: &mut State, modules: &HashMap<Key, Module>) {
    let mut pulses: VecDeque<(Key, Key, bool)> = VecDeque::new();
    pulses.push_back((Key::new("button"), Key::new("broadcaster"), false));

    while let Some((source, key, pulse)) = pulses.pop_front() {
        match pulse {
            true => {
                state.counts_high += 1;
            }
            false => {
                if key == Key::new("rx") {
                    state.rx_received_low = true;
                }
                state.counts_low += 1;
            }
        }
        debug!("{} -{}-> {}", source, pulse, key);
        if !modules.contains_key(&key) {
            continue;
        }

        let module = modules.get(&key).unwrap();
        match module.kind {
            Kind::Broadcaster => {
                for output in &module.outputs {
                    pulses.push_back((key, *output, pulse));
                }
            }
            Kind::FlipFlop => {
                let state = state.flipflop.entry(key).or_insert(false);

                match pulse {
                    true => (),
                    false => {
                        *state = !*state;

                        for output in &module.outputs {
                            pulses.push_back((key, *output, *state));
                        }
                    }
                }
            }
            Kind::Conjunction => {
                let state = state
                    .conjunction
                    .entry(key)
                    .or_insert_with(|| module.inputs.iter().map(|k| (*k, false)).collect());

                state.insert(source, pulse);

                for output in &module.outputs {
                    pulses.push_back((key, *output, !state.iter().all(|(_, v)| *v)));
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(run_cycles(input, 1000), 32000000);
    }
}
