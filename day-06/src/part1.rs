use itertools::Itertools;

pub fn part1() {}

pub struct Race {
    pub time: u64,
    pub distance: u64,
}

pub fn number_of_winning_ways(race: &Race) -> u64 {
    let mut ways: u64 = 0;
    for time in 0..race.time as u64 {
        let speed = time;
        if speed * (race.time - time) > race.distance {
            ways += 1;
        }
    }
    ways
}

pub fn parse_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let distances = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| Race { time, distance })
        .collect_vec()
}

pub fn multiply_winning_ways(input: &str) -> u64 {
    let races = parse_races(input);
    races.iter().map(number_of_winning_ways).product()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_speed() {
        assert_eq!(
            number_of_winning_ways(&Race {
                time: 7,
                distance: 9
            }),
            4
        );
        assert_eq!(
            number_of_winning_ways(&Race {
                time: 15,
                distance: 40
            }),
            8
        );
        assert_eq!(
            number_of_winning_ways(&Race {
                time: 30,
                distance: 200
            }),
            9
        );
    }

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(multiply_winning_ways(test), 288);
    }
}
