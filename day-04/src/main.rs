use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!(
        "Scratchcards score: {}",
        day_04::part1::count_winning(input)
    );
    println!(
        "Scratchcards number: {}",
        day_04::part2::run_scratchcard_game(input)
    );
    runner.end();
}
