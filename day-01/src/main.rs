use day_01::{part1::sum_all_lines, part2::sum_string_tokens};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("First part sum: {}", sum_all_lines(input));
    println!("Second part sum: {}", sum_string_tokens(input));
    runner.end();
}
