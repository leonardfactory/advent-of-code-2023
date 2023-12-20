use day_16::{
    part1::{count_energized, count_energized_topleft},
    part2::top_energized,
};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Energized: {}", count_energized_topleft(input));
    println!("Top energized: {}", top_energized(input));
    runner.end();
}
