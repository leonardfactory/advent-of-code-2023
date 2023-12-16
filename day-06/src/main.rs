use day_06::{part1::multiply_winning_ways, part2::single_race_ways};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Product of winning ways: {}", multiply_winning_ways(input));
    println!("Single winning ways: {}", single_race_ways(input));
    runner.end();
}
