pub mod days;
use aoc_core::AoCResult;
use aoc_core::AoCDay;

use crate::days::*;

pub fn solve(day: u8, part: u8, input: &String) -> AoCResult {
    let solver: Option<&dyn AoCDay> = match day {
         1 => Some(&day01::Day01 {}),
         2 => Some(&day02::Day02 {}),
         3 => Some(&day03::Day03 {}),
         _ => None,
    };

    if let Some(solver) = solver {
        if part == 1 {
            solver.run_part1(input)
        } else if part == 2 {
            solver.run_part2(input)
        } else {
            AoCResult::InvalidPartErr(day, part)
        }
    } else {
        AoCResult::InvalidDayErr(day)
    }
}
