use crate::aoc_result::AoCResult;
use std::str::FromStr;

pub fn solve_part1(input: &String) -> AoCResult {
    let res: u64 = input
        .lines()
        .map(|l| u64::from_str(l).unwrap())
        .map(|n| n / 3 - 2)
        .sum();

    AoCResult::Num(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("12");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 2);
    }

    #[test]
    fn part1_example2() {
        let input = String::from("14");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 2);
    }

    #[test]

    fn part1_example3() {
        let input = String::from("1969");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 654);
    }

    #[test]
    fn part1_example4() {
        let input = String::from("100756");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 33583);
    }
}
