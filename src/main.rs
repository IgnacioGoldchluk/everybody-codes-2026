use argh::FromArgs;
use solutions::solution::Solver;
use std::fs;
use std::time::Instant;
mod solutions;

#[derive(FromArgs)]
/// Executes the given Everybody Codes day
struct Args {
    /// the day to run
    #[argh(option)]
    day: u8,
}

fn main() {
    let args: Args = argh::from_env();

    let input = read_input(args.day);

    let solver: Box<dyn Solver> = match args.day {
        1 => Box::new(solutions::day1::Day1Solver {}),
        2 => Box::new(solutions::day2::Day2Solver {}),
        3 => Box::new(solutions::day3::Day3Solver {}),
        4 => Box::new(solutions::day4::Day4Solver {}),
        5 => Box::new(solutions::day5::Day5Solver {}),
        6 => Box::new(solutions::day6::Day6Solver {}),
        7 => Box::new(solutions::day7::Day7Solver {}),
        _ => todo!("Unreachable"),
    };
    let now = Instant::now();
    let solution = solver.solve(input);
    println!("{}", solution);
    println!("Elapsed: {:.2?}", now.elapsed());
}

fn read_input(day: u8) -> solutions::solution::Input {
    let file_path = format!("inputs/{}/", day);

    solutions::solution::Input {
        part1: fs::read_to_string(file_path.to_string() + "/1").unwrap(),
        part2: fs::read_to_string(file_path.to_string() + "/2").unwrap(),
        part3: fs::read_to_string(file_path.to_string() + "/3").unwrap(),
    }
}
