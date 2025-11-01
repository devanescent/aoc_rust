use itertools::{Itertools, fold};
use crate::aoc_result::AoCResult;
use std::str::FromStr;

make_day!(Day04);

pub fn solve_part1(input: &String) -> AoCResult {
   	let input_limits = input.split_once('-').unwrap();
	let range_limits = (u32::from_str(input_limits.0).unwrap(), u32::from_str(input_limits.1).unwrap());

	let valid_password_count = (range_limits.0..=range_limits.1)
		.map(|n| n.to_string())
		.filter(|pwd| check_adjacent_equal_digits(&pwd) && check_no_decreasing_digits(&pwd))
		.count();

	AoCResult::Num(u64::try_from(valid_password_count).unwrap())
}

pub fn solve_part2(input: &String) -> AoCResult {
	let input_limits = input.split_once('-').unwrap();
	let range_limits = (u32::from_str(input_limits.0).unwrap(), u32::from_str(input_limits.1).unwrap());

	let valid_password_count = (range_limits.0..=range_limits.1)
		.map(|n| n.to_string())
		.filter(|pwd| check_only_two_adjacent_equal_digits(&pwd) && check_no_decreasing_digits(&pwd))
		.count();

	AoCResult::Num(u64::try_from(valid_password_count).unwrap())
}

fn check_adjacent_equal_digits(password: &String) -> bool {
	password.chars().tuple_windows().any(|(c1, c2)| c1 == c2)
}

fn check_no_decreasing_digits(password: &String) -> bool {
	password.chars().tuple_windows().all(|(c1, c2)| c1 <= c2)
}

fn check_only_two_adjacent_equal_digits(password: &String) -> bool {
	// Count adjacent equal digits:
	let mut current_digit = ' ';
	let mut current_digit_count = 0;
	for c in password.chars() {
		if c != current_digit {
			if current_digit_count == 2 {
				return true;
			}

			current_digit = c;
			current_digit_count = 1;
		} else {
			current_digit_count += 1;
		}
	}

	return current_digit_count == 2;
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
		let input = String::from("111111-111111");
        let result = solve_part1(&input); 
        assert_eq!(u64::from(result), 1);
    }

	#[test]
	fn part1_example2() {
		let input = String::from("223450-223450");
        let result = solve_part1(&input); 
        assert_eq!(u64::from(result), 0);
    }

	#[test]
	fn part1_example3() {
		let input = String::from("123789-123789");
        let result = solve_part1(&input); 
        assert_eq!(u64::from(result), 0);
    }

	#[test]
	fn part2_example1() {
		let input = String::from("112233-112233");
        let result = solve_part2(&input); 
        assert_eq!(u64::from(result), 1);
    }

	#[test]
	fn part2_example2() {
		let input = String::from("123444-123444");
        let result = solve_part2(&input); 
        assert_eq!(u64::from(result), 0);
    }

	#[test]
	fn part2_example3() {
		let input = String::from("111122-111122");
        let result = solve_part2(&input); 
        assert_eq!(u64::from(result), 1);
    }
}