use itertools::Itertools;

use crate::part1::{parse_almanac, Almanac};

struct SeedsRange {
    seed: u64,
    range: u64,
}

pub fn find_ranges_nearest_location(input: &str) -> u64 {
    let almanac = parse_almanac(input);

    let mut seed_ranges: Vec<SeedsRange> = vec![];
    let mut seeds_iter = almanac.seeds.iter();
    while let Some((seed, range)) = seeds_iter.next_tuple() {
        seed_ranges.push(SeedsRange {
            seed: *seed,
            range: *range,
        });
    }

    // Ok, we could theoretically do a binary search here, dividing the range for
    // each seed in multiple ranges based on the N+1 almanac mapping rules;
    // this partial bruteforce however takes just 30sec to run, so no need to bother! 🎄
    for i in 0..4_000_000_000 {
        // if i % 100_000_000 == 0 {
        //     println!("Checking location: {}", i);
        // }
        let seed = inverse_lookup_seed(&almanac, i);
        let is_in_range = seed_ranges.iter().any(|seed_range| {
            seed_range.seed <= seed && seed_range.seed + seed_range.range >= seed
        });

        if is_in_range {
            println!("Found at location: {}, rule:", i);
            return i;
        }
    }
    panic!("Not found");
}

impl Almanac {
    pub fn inverse_lookup(&self, to: &str, value: u64) -> (u64, String) {
        let mapping = self.to_mapping(to);
        let rule = mapping
            .rules
            .iter()
            .find(|rule| rule.destination <= value && value <= rule.destination + rule.length);

        match rule {
            Some(rule) => {
                let from_value = rule.source + (value - rule.destination);
                // println!(
                //     "[to={}] Found rule: {:?} (to={},from={})",
                //     to, rule, value, from_value
                // );
                (from_value, mapping.from.clone())
            }
            None => (value, mapping.from.clone()),
        }
    }
}

pub fn inverse_lookup_seed(almanac: &Almanac, location: u64) -> u64 {
    let mut current = location;
    let mut mapping_to = "location".to_string();
    while mapping_to != "seed" {
        let (value, mapping_from) = almanac.inverse_lookup(&mapping_to, current);
        current = value;
        mapping_to = mapping_from.to_string();
    }
    current
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        println!("Test example");
        let test = include_str!("../test.txt");
        assert_eq!(find_ranges_nearest_location(test), 46);
    }
}
