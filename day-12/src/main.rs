use day_12::{part1::solve_valid_springs, part2::solve_folded_springs};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Solution for springs: {}", solve_valid_springs(input));
    println!(
        "Solution for folded springs: {}",
        solve_folded_springs(input)
    );
    runner.end();
}
