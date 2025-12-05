use std::{cmp::max, collections::HashSet};

use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day05);

pub fn solve_part1(input: &String) -> AoCResult {
    let (fresh_ingredient_ranges, ingredients) = input.split_once("\r\n\r\n").unwrap();

    let id_ranges = fresh_ingredient_ranges
        .trim()
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .collect_vec();

    let fresh_ingredients = ingredients
        .trim()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|id| id_ranges.iter().any(|(start, end)| start <= id && id <= end ))
        .count();

    AoCResult::Num(fresh_ingredients as u64)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let id_ranges = input
        .split_once("\r\n\r\n").unwrap().0
        .trim()
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .sorted_by_key(|range| range.0)
        .collect_vec();
    
    // Merge ranges together:
    let mut merged_range = id_ranges.iter().next().unwrap().clone();
    let mut non_overlapping_ranges = vec![];
    for range in id_ranges.iter().skip(1) {
        if range.0 <= merged_range.1 {
            merged_range = (merged_range.0, max(range.1, merged_range.1));
        } else {
            non_overlapping_ranges.push(merged_range);
            merged_range = range.clone();
        }
    }
    non_overlapping_ranges.push(merged_range);

    let total_range = non_overlapping_ranges
        .into_iter()
        .map(|r| r.1 - r.0 + 1)
        .sum::<u64>();

    AoCResult::Num(total_range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("3-5\r\n10-14\r\n16-20\r\n12-18\r\n\r\n1\r\n5\r\n8\r\n11\r\n17\r\n32");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 3);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("3-5\r\n10-14\r\n16-20\r\n12-18\r\n\r\n1\r\n5\r\n8\r\n11\r\n17\r\n32");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 14);
    }
}
