use itertools::Itertools;

pub struct Game {
    pub id: i32,
    pub rounds: Vec<Round>,
}

#[derive(Copy, Clone)]
pub struct Round {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

pub fn parse_game(line: &str) -> Game {
    let (id_str, rounds) = line.split_once(':').unwrap();
    let id = id_str.replace("Game ", "").parse::<i32>().unwrap();
    let game = Game {
        id,
        rounds: rounds.split(';').map(parse_round).collect_vec(),
    };
    game
}

pub fn parse_round(line: &str) -> Round {
    let tokens = line.split(',').map(|t| t.trim());
    let mut round = Round { r: 0, g: 0, b: 0 };

    tokens.for_each(|t| {
        let (val, color) = t.split_once(' ').unwrap();
        let val = val.parse::<i32>().unwrap();
        match color {
            "red" => round.r = val,
            "green" => round.g = val,
            "blue" => round.b = val,
            _ => panic!("Invalid color"),
        }
    });

    round
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect_vec()
}

pub fn is_game_possibile(condition: Round, game: &Game) -> bool {
    !game
        .rounds
        .iter()
        .any(|round| (round.r > condition.r) || (round.g > condition.g) || (round.b > condition.b))
}

pub fn sum_possible_ids(input: &str) -> i32 {
    let games = parse_input(input);
    let condition = Round {
        r: 12,
        g: 13,
        b: 14,
    };
    let possible_ids_sum = games
        .iter()
        .filter(|game| is_game_possibile(condition, game))
        .map(|g| g.id)
        .sum();

    possible_ids_sum
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(sum_possible_ids(test), 8);
    }
}
