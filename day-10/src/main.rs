use day_10::{part1::farthest_pipe, part2::count_inside_loop};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Farthest pipe: {}", farthest_pipe(input));
    println!("Inside pipe count: {}", count_inside_loop(input));
    runner.end();
}
