use day_14::{part1::north_beams_load, part2::run_slide_cycles};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("North beams load: {}", north_beams_load(input));
    println!("Cycled north beams load: {}", run_slide_cycles(input));
    runner.end();
}
