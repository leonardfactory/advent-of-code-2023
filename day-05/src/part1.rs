use std::collections::HashMap;

use itertools::Itertools;

pub fn part1() {}

pub struct Mapping {
    pub from: String,
    pub to: String,
    pub rules: Vec<MappingRule>,
}

#[derive(Debug, Clone, Copy)]
pub struct MappingRule {
    pub destination: u64,
    pub source: u64,
    pub length: u64,
}

pub struct Almanac {
    pub seeds: Vec<u64>,
    pub mappings: Vec<Mapping>,
}

impl Almanac {
    pub fn from_mapping(&self, from: &str) -> &Mapping {
        self.mappings
            .iter()
            .find(|mapping| mapping.from == from)
            .unwrap()
    }

    pub fn to_mapping(&self, to: &str) -> &Mapping {
        self.mappings
            .iter()
            .find(|mapping| mapping.to == to)
            .unwrap()
    }

    pub fn lookup(&self, from: &str, value: u64) -> (u64, String) {
        let mapping = self.from_mapping(from);
        let rule = mapping
            .rules
            .iter()
            .find(|rule| rule.source <= value && value <= rule.source + rule.length);

        match rule {
            Some(rule) => {
                let to_value = rule.destination + (value - rule.source);
                (to_value, mapping.to.clone())
            }
            None => (value, mapping.to.clone()),
        }
    }
}

pub fn parse_almanac(input: &str) -> Almanac {
    let chunks: Vec<_> = input.split("\n\n").collect();
    println!("Chunks: {:?}", chunks[0]);
    let seeds = chunks[0]
        .replace("seeds: ", "")
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mappings: Vec<Mapping> = chunks
        .iter()
        .skip(1)
        .map(|chunk| {
            let mut lines = chunk.lines();

            // es. "seed-to-soil map:""
            let key_tokens: Vec<_> = lines
                .next()
                .unwrap()
                .replace(" map:", "")
                .split('-')
                .map(|s| s.trim().to_string())
                .collect();

            let rules = lines
                .map(|line| {
                    let tokens: Vec<_> = line.split(' ').collect();
                    let from = tokens[0].parse::<u64>().unwrap();
                    let to = tokens[1].parse::<u64>().unwrap();
                    let length = tokens[2].parse::<u64>().unwrap();

                    MappingRule {
                        destination: from,
                        source: to,
                        length,
                    }
                })
                .collect_vec();

            Mapping {
                from: key_tokens[0].clone(),
                to: key_tokens[2].clone(),
                rules,
            }
        })
        .collect();

    Almanac { seeds, mappings }
}

pub fn lookup_location(almanac: &Almanac, seed: u64) -> u64 {
    let mut current = seed;
    let mut mapping_from = "seed".to_string();
    while mapping_from != "location" {
        let (value, mapping_to) = almanac.lookup(&mapping_from, current);
        current = value;
        mapping_from = mapping_to.to_string();
    }
    current
}

pub fn find_nearest_location(input: &str) -> u64 {
    let almanac = parse_almanac(input);
    let nearest_location = almanac
        .seeds
        .iter()
        .map(|seed| lookup_location(&almanac, *seed))
        .min()
        .unwrap();
    nearest_location
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parse() {
        let test = include_str!("../test.txt");
        let almanac = parse_almanac(test);
        assert_eq!(almanac.seeds.len(), 4);
        assert_eq!(almanac.mappings.len(), 7);
        assert_eq!(almanac.from_mapping("humidity").to, "location");

        assert_eq!(almanac.lookup("humidity", 0), (0, "location".to_string()));
        assert_eq!(almanac.lookup("seed", 79), (81, "soil".to_string()));
        assert_eq!(almanac.lookup("seed", 13), (13, "soil".to_string()));
        assert_eq!(almanac.lookup("seed", 14), (14, "soil".to_string()));
        assert_eq!(almanac.lookup("seed", 55), (57, "soil".to_string()));
    }

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(find_nearest_location(test), 35);
    }
}
