use aoc_2019::aoc_result::AoCResult;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    day: u8,

    #[arg(default_value = "1")]
    part: u8,
}

fn main() {
    let args = Args::parse();

    let input =
        fs::read_to_string("input.txt").expect("Expected file 'input.txt' in project directory");

    let res = aoc_2019::solve(args.day, args.part, &input);
    match res {
        AoCResult::Num(_) | AoCResult::Str(_) | AoCResult::PrintedToConsole => {
            println!("Day {:02} (part {}): {}", args.day, args.part, res)
        }
        _ => println!("Error: {}", res),
    };
}
