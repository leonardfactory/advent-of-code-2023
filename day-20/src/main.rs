use day_20::{part1::run_cycles, part2::find_min_rx_cycles};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("High * Low = {}", run_cycles(input, 1000));
    println!("Min RX = {}", find_min_rx_cycles(input));
    runner.end();
}
