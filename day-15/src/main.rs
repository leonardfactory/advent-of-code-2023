use day_15::{part1::hash_init_seq, part2::focusing_power};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Hash of init sequence: {}", hash_init_seq(input));
    println!("Fousing power: {}", focusing_power(input));
    runner.end();
}
