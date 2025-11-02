use crate::aoc_result::AoCResult;
use std::str::FromStr;

make_day!(Day01);

pub fn solve_part1(input: &String) -> AoCResult {
    let res: u64 = input
        .lines()
        .map(|l| u64::from_str(l).unwrap())
        .map(get_fuel)
        .sum();

    AoCResult::Num(res)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let res: u64 = input
        .lines()
        .map(|l| u64::from_str(l).unwrap())
        .map(get_fuel_recursive)
        .sum();

    AoCResult::Num(res)
}

fn get_fuel(mass: u64) -> u64 {
    u64::saturating_sub(mass / 3, 2)
}

fn get_fuel_recursive(mass: u64) -> u64 {
    let fuel = get_fuel(mass);
    if fuel > 0 {
        fuel + get_fuel_recursive(fuel)
    } else {
        0
    }
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

    #[test]
    fn part2_example1() {
        let input = String::from("14");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 2);
    }

    #[test]

    fn part2_example2() {
        let input = String::from("1969");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 966);
    }

    #[test]
    fn part2_example3() {
        let input = String::from("100756");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 50346);
    }
}
