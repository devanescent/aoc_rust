#[macro_use]
pub mod aoc_day;
use crate::aoc_day::AoCDay;

pub mod aoc_result;
use crate::aoc_result::AoCResult;

pub mod days;
use crate::days::*;

pub mod shared;

pub fn solve(day: u8, part: u8, input: &String) -> AoCResult {
    let solver: Option<&dyn AoCDay> = match day {
        1 => Some(&day01::Day01 {}),
        2 => Some(&day02::Day02 {}),
        3 => Some(&day03::Day03 {}),
        4 => Some(&day04::Day04 {}),
        5 => Some(&day05::Day05 {}),
        6 => Some(&day06::Day06 {}),
        7 => Some(&day07::Day07 {}),
        8 => Some(&day08::Day08 {}),
        9 => Some(&day09::Day09 {}),
		10 => Some(&day10::Day10 {}),
		11 => Some(&day11::Day11 {}),
		12 => Some(&day12::Day12 {}),
		13 => Some(&day13::Day13 {}),
		14 => Some(&day14::Day14 {}),
		15 => Some(&day15::Day15 {}),
		16 => Some(&day16::Day16 {}),
		17 => Some(&day17::Day17 {}),
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
