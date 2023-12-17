use day_09::{part1::sum_predictions, part2::sum_postdictions};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Sum of predictions: {}", sum_predictions(input));
    println!("Sum of postdictions: {}", sum_postdictions(input));
    runner.end();
}
