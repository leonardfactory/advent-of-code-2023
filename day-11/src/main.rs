use day_11::{part1::count_distances, part2::count_old_distances};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Sum pairs distance: {}", count_distances(input));
    println!(
        "Sum old universe distances: {}",
        count_old_distances(input, 1_000_000)
    );
    runner.end();
}
