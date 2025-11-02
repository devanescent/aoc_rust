use std::collections::{HashMap, HashSet};
use std::iter::successors;
use std::str::FromStr;
use crate::aoc_result::AoCResult;

make_day!(Day06);

pub fn solve_part1(input: &String) -> AoCResult {
	let planets = get_planets(input);
   	let orbits = get_orbits(input);

	// Count direct and indirect orbits for each planet:
	let total_orbits : usize = planets
		.iter()
		.map(|p| successors(Some(p), |p| orbits.get(*p)).count() - 1)
		.sum();
	
	AoCResult::Num(u64::try_from(total_orbits).unwrap())
}

pub fn solve_part2(input: &String) -> AoCResult {
	let orbits = get_orbits(input);
	
	// Find parents for YOU and SAN:
	let mut you_parents : Vec<_> = successors(Some("YOU".to_string()), |p| orbits.get(p).cloned()).collect();
	let mut san_parents : Vec<_> = successors(Some("SAN".to_string()), |p| orbits.get(p).cloned()).collect();
	
	// Find first common parent:
	while you_parents.last() == san_parents.last() {
		you_parents.pop();
		san_parents.pop();
	}

	let orbit_transfers = you_parents.len() - 1 + san_parents.len() - 1;
	AoCResult::Num(u64::try_from(orbit_transfers).unwrap())
}

fn get_planets(input: &String) -> HashSet<String> {
	input
		.lines()
		.map(|l| l.split(')'))
		.flatten()
		.map(|s| String::from_str(s).unwrap())
		.collect()
}

fn get_orbits(input: &String) -> HashMap<String, String> {
	input
        .lines()
        .map(|l| l.split_once(')').unwrap())
		// Invert order here: map each planet to the planet it orbits
		.map(|(orbit_center,in_orbit)| (String::from_str(in_orbit).unwrap(), String::from_str(orbit_center).unwrap()))
		.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
		let input = String::from("COM)B\r\nB)C\r\nC)D\r\nD)E\r\nE)F\r\nB)G\r\nG)H\r\nD)I\r\nE)J\r\nJ)K\r\nK)L");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 42);
    }

	#[test]
    fn part2_example1() {
		let input = String::from("COM)B\r\nB)C\r\nC)D\r\nD)E\r\nE)F\r\nB)G\r\nG)H\r\nD)I\r\nE)J\r\nJ)K\r\nK)L\r\nK)YOU\r\nI)SAN");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 4);
    }
}