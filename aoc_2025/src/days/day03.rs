use std::collections::HashSet;

use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day03);

pub fn solve_part1(input: &String) -> AoCResult {
    let res: u64 = input
        .trim_end()
        .lines()
        .map(|l| find_max_joltage(l, 2))
        .sum();

    AoCResult::Num(res)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let res: u64 = input
        .trim_end()
        .lines()
        .map(|l| find_max_joltage(l, 12))
        .sum();

    AoCResult::Num(res)
}

fn find_max_joltage(bank: &str, batt_num: usize) -> u64 {
    let mut selected_batts = HashSet::<(usize, char)>::new();

    find_rightmost_max_recursive(bank, 0, bank.len(), &mut selected_batts, batt_num);

    selected_batts.iter()
        .sorted_by(|(i1, _), (i2, _)| i2.cmp(i1)) // sort from least-significant to most-significant digit
        .enumerate()
        .map(|(i, (_,c))| 10u64.pow(i as u32) * (c.to_digit(10).unwrap() as u64))
        .sum()
}

// 'end' is one behind the last valid value:
fn find_rightmost_max_recursive(bank: &str, start: usize, end: usize, selected_batts: &mut HashSet<(usize, char)>, batt_num_target: usize) -> bool {  
    let max_opt = bank.char_indices()
        .skip(start)
        .take(end - start)
        .max_by(|(_, c1), (_, c2)| c1.cmp(c2));

    if let Some(max_val) = max_opt {
        // Found first value in given range that matches the maximum value:
        let next_max = bank.char_indices()
            .skip(start)
            .take(end - start)
            .find(|(_, c)| c == &max_val.1).unwrap();
        selected_batts.insert(next_max);
        
        // Target size reached?
        if selected_batts.len() == batt_num_target {
            return true;
        }

        // First, try to find a value to the right of it:
        if find_rightmost_max_recursive(bank, next_max.0 + 1, end, selected_batts, batt_num_target) {
            return true;
        // Else try the left:
        } else if find_rightmost_max_recursive(bank, start, next_max.0, selected_batts, batt_num_target) {
           return true;
        }
    }

    return false;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("987654321111111\r\n811111111111119\r\n234234234234278\r\n818181911112111");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 357);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("987654321111111\r\n811111111111119\r\n234234234234278\r\n818181911112111");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 3121910778619);
    }
}
