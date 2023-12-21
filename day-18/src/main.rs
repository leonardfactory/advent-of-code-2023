use day_18::{part1::dig_lagoon, part2::dig_wide_lagoon};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Lagoon metric cubes: {}", dig_lagoon(input));
    println!("Wide Lagoon: {}", dig_wide_lagoon(input));
    runner.end();
}
