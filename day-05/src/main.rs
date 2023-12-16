use day_05::{part1::find_nearest_location, part2::find_ranges_nearest_location};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Nearest location: {}", find_nearest_location(input));
    println!(
        "Nearest range location (wow): {}",
        find_ranges_nearest_location(input)
    );
    runner.end();
}
