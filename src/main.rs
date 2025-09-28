mod aoc_result;
mod days;

use crate::days::day02;
use std::fs;

fn main() {
    let data =
        fs::read_to_string("input.txt").expect("Expected file 'input.txt' in project directory");
    println!("Day 02 (part 1): {}", day02::solve_part1(&data));
    println!("Day 02 (part 2): {}", day02::solve_part2(&data));
}
