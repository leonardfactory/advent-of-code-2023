use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!(
        "Sum of engine numbers: {}",
        day_03::part1::sum_valid_part_numbers(input)
    );
    println!("Sum of gear ratios: {}", day_03::part2::find_gears(input));
    runner.end();
}
