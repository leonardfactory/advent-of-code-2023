use crate::part1::{parse_input, Game};

pub fn min_cubes_power(game: &Game) -> i32 {
    let min_r = game.rounds.iter().map(|r| r.r).max().unwrap();
    let min_g = game.rounds.iter().map(|r| r.g).max().unwrap();
    let min_b = game.rounds.iter().map(|r| r.b).max().unwrap();
    min_r * min_g * min_b
}

pub fn count_fewest_cubes(input: &str) -> i32 {
    let games = parse_input(input);
    let min_cubes_power = games.iter().map(min_cubes_power).sum();
    min_cubes_power
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_fewest_cubes(input), 2286);
    }
}
