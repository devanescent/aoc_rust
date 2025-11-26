use aoc_core::AoCResult;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    #[arg(value_parser = validate_year)]
    year: u16,

    #[arg(value_parser = validate_day)]
    day: u8,

    #[arg(default_value = "1", value_parser = validate_part)]
    part: u8,
}

fn validate_year(input: &str) -> Result<u16, String> {
    let err_msg = "Valid years: 2019, 2015".to_string();
    let year: u16 = input.parse().map_err(|_| &err_msg)?;
    if year == 2019 || year == 2025 {
         Ok(year)
    } else {
        Err(err_msg)
    }
}

fn validate_day(input: &str) -> Result<u8, String> {
    let err_msg = "Day must be between 01 and 25".to_string();
    let day: u8 = input.parse().map_err(|_| &err_msg)?;
    if day >= 1 && day <= 25 {
        Ok(day)
    } else {
        Err(err_msg)
    }
}

fn validate_part(input: &str) -> Result<u8, String> {
    let err_msg = "Part must be 1 or 2".to_string();
    let part: u8 = input.parse().map_err(|_| &err_msg)?;
    if part == 1 || part == 2 {
        Ok(part)
    } else {
        Err(err_msg)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("No file 'input.txt' found in project directory");
    let args = Args::parse();

    let res = match args.year {
        2019 => aoc_2019::solve(args.day, args.part, &input),
        2025 => aoc_2025::solve(args.day, args.part, &input),
           y => unreachable!("{} is not a valid year", y)
    };

    match res {
        AoCResult::Num(_) | AoCResult::Str(_) | AoCResult::PrintedToConsole => {
            println!("Day {:02} (part {}): {}", args.day, args.part, res)
        }
        _ => println!("Error: {}", res),
    };
}
