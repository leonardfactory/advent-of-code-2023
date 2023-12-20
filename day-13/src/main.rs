use day_13::{part1::find_reflections, part2::find_smudged_reflections};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Summarizing input: {}", find_reflections(input));
    println!(
        "Summarizing smudged input: {}",
        find_smudged_reflections(input)
    );
    runner.end();
}
