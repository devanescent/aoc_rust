use std::collections::HashSet;

use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day02);

pub fn solve_part1(input: &String) -> AoCResult {
    let ranges = get_ranges(input);

    let mut invalid_id_sum = 0;
    for (start, end) in ranges {
        let start_num: u64 = start.parse().unwrap();
        let end_num: u64 = end.parse().unwrap();

        // Iterate over first half of the number, second half has to match the first half:
        let mut half_num: u64 = if start.len() > 1 {
            start.split_at(start.len() / 2).0.parse().unwrap()
        } else {
            // For single-digit starts, start iterating from 1
            1
        };

        loop {
            let next_num: u64 = format!("{}{}", half_num, half_num).parse().unwrap();
            if next_num > end_num {
                break;
            } else if next_num >= start_num {
                invalid_id_sum += next_num;
            }

            half_num += 1;
        }
    }

    AoCResult::Num(invalid_id_sum)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let ranges = get_ranges(input);

    // Because of the way patterns are checked, the same pattern can occur multiple times
    // Collect in hashset, then sum up at the end (assumes the ranges do not overlap,
    // so the same number is valid only once)
    let mut invalid_ids = HashSet::new();
    for (start, end) in ranges {
        let start_num: u64 = start.parse().unwrap();
        let end_num: u64 = end.parse().unwrap();

        // Last possible pattern start depends on end_num length:
        let last: u64 = if end.len() % 2 == 0 {
            // Even length: take the first half as number + 1:
            end.split_at(end.len() / 2).0.parse::<u64>().unwrap() + 1
        } else {
            // Take (len / 2) number of 9s (99...):
            10u64.pow((end.len() / 2) as u32) - 1
        };

        // Build patterns:
        for i in 1..=last {
            let mut pattern = format!("{}{}", i, i); // pattern must repeat at least twice!
            
            // Increase pattern length while resulting number is less than the start number:
            let mut next_num = pattern.parse::<u64>().unwrap();
            while next_num < start_num {
                pattern += &i.to_string();
                next_num = pattern.parse::<u64>().unwrap();
            }

            if next_num <= end_num {
                invalid_ids.insert(next_num);
            }
        }
    }

    let invalid_id_sum = invalid_ids.iter().sum();
    AoCResult::Num(invalid_id_sum)
}

fn get_ranges(input: &String) -> Vec<(&str, &str)> {
    input
        .trim_end()
        .split(",")
        .map(|range| range.split_once("-").unwrap() )
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 1227775554);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 4174379265);
    }
}
