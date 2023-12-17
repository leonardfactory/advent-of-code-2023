use day_08::{part1::count_steps_exit, part2::run_ghosts_paths};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Steps to exit: {}", count_steps_exit(input));
    println!("Ghost steps: {}", run_ghosts_paths(input));
    runner.end();
}
