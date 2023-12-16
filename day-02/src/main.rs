use day_02::{part1::sum_possible_ids, part2::count_fewest_cubes};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Sum possible ids: {}", sum_possible_ids(input));
    println!("Min cubes power: {}", count_fewest_cubes(input));
    runner.end();
}
