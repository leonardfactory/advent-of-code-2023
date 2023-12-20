use crate::part1::{calculate_north_beams_load, parse_rocks, slide_direction, Direction, Map};

pub fn run_slide_cycle(map: &mut Map) {
    slide_direction(map, Direction::North);
    slide_direction(map, Direction::West);
    slide_direction(map, Direction::South);
    slide_direction(map, Direction::East);
}

// too low 103856
// too low 103860
// too high 103876
pub fn run_slide_cycles(input: &str) -> u32 {
    let mut map = parse_rocks(input);
    // run_slide_cycle(&mut map);
    let mut latest = vec![];
    let mut skipped = false;
    let mut i = 0;
    while i < 1_000_000_000 {
        if i % 100_000 == 0 {
            println!("Cycle {}", i);
        }

        run_slide_cycle(&mut map);
        // map.print();

        if let Some(prev_index) = latest.iter().position(|m| *m == map) {
            println!("Cycle {} is the same as cycle {}", i, prev_index);
            let delta = i - prev_index;

            i += 1_000_000_000 - (1_000_000_000 % delta) - delta * ((i / delta) + 1);
            println!("Skipping to cycle {}", i);
            skipped = true;
            latest.clear();
        }

        i += 1;
        if !skipped {
            latest.push(map.clone());
        }
    }
    map.print();
    calculate_north_beams_load(&map)
}

#[cfg(test)]
pub mod tests {
    use crate::{part1::parse_rocks, part2::*};

    #[test]
    fn test_one_cycle() {
        let input = include_str!("../test.txt");
        let expected = include_str!("../test_1cycle.txt");
        let mut map = parse_rocks(input);
        let expected_map = parse_rocks(expected);
        run_slide_cycle(&mut map);
        map.print();
        expected_map.print();
        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(run_slide_cycles(input), 64);
    }
}
