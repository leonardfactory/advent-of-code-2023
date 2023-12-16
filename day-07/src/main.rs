use day_07::{part1::total_winnings, part2::jolly_total_winnings};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Total winning: {}", total_winnings(input));
    println!("Total Jolly winning: {}", jolly_total_winnings(input));
    runner.end();
}
