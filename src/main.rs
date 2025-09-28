mod aoc_result;
mod days;

use crate::days::day01;
use std::fs;

fn main() {
    let data =
        fs::read_to_string("input.txt").expect("Expected file 'input.txt' in project directory");
    println!("Day 01 (part 1): {}", day01::solve_part1(&data));
    println!("Day 01 (part 2): {}", day01::solve_part2(&data));
}
