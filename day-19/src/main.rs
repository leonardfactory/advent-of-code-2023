use day_19::{part1::count_accepted_parts, part2::count_acceptable};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Accepted count: {}", count_accepted_parts(input));
    println!("Acceptable count: {}", count_acceptable(input));
    runner.end();
}
