use toolkit::map::Pos;

use crate::part1::{count_energized, parse_contraption};

pub fn top_energized(input: &str) -> usize {
    let contraption = parse_contraption(input);
    let mut top_count = 0;

    let mut starts = vec![];
    for y in -1..=contraption.bounds.height() {
        starts.push((Pos::new(-1, y), Pos::RIGHT));
        starts.push((Pos::new(contraption.bounds.width(), y), Pos::LEFT));
    }

    for x in -1..=contraption.bounds.width() {
        starts.push((Pos::new(x, contraption.bounds.height()), Pos::UP));
        starts.push((Pos::new(x, -1), Pos::DOWN));
    }

    for start in starts {
        let count = count_energized(&contraption, start);
        if count > top_count {
            top_count = count;
        }
    }

    top_count
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(top_energized(input), 51);
    }
}
