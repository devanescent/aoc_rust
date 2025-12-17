use std::cmp::min;

use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day10);

pub fn solve_part1(input: &String) -> AoCResult {
    let machines = input
        .trim_end()
        .lines()
        .map(|l| Machine::new(l))
        .collect_vec();

    let total_button_presses: u64 = machines.iter()
        .map(|m| m.get_min_button_presses())
        .sum();

    AoCResult::Num(total_button_presses)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let machines = input
        .trim_end()
        .lines()
        .map(|l| Machine::new(l))
        .collect_vec();

    let total_button_presses = machines.iter()
        .map(|m| m.configure_joltage_counters())
        .sum();
    
    AoCResult::Num(total_button_presses)
}


struct Machine {
    lights_target: u64,
    buttons: Vec<u64>,
    joltage_reqs: Vec<i64>
}

impl Machine {
    fn new(input: &str) -> Self {
        // Convert light diagram into numeric value
        let mut lights_target = 0u64;

        // Convert buttons into numeric values:
        let mut buttons_values = vec![];

        // Convert joltage requirements into list of values:
        let mut joltags_reqs = vec![];

        for s in input.split(" ") {
            if s.starts_with("[") {
                // Light diagram:
                for (i, c) in s[1..s.len() - 1].char_indices() {
                    if c == '#' {
                        lights_target |= 1 << i;
                    }
                }

            } else if s.starts_with("(") {
                // Button:
                let mut btn_value = 0u64;
                for b in s[1..s.len() - 1].split(",") {
                    btn_value |= 1 << b.parse::<u64>().unwrap();
                }
                buttons_values.push(btn_value);

            } else if s.starts_with("{") {
                // Joltages:
                joltags_reqs = s[1..s.len() - 1].split(",").map(|n| n.parse().unwrap()).collect();
            }
        }

        Machine {
            lights_target: lights_target,
            buttons: buttons_values,
            joltage_reqs: joltags_reqs
        }
    }

    fn get_min_button_presses(&self) -> u64 {
        for i in 1..self.buttons.len() {
            for combo in self.buttons.iter().combinations(i) {
                let result = combo.into_iter().fold(0u64, |acc, x| acc ^ *x);
                if result == self.lights_target {
                    return i as u64;
                }
            }
        }

        return 0;
    }

    // Get minimum button presses for joltage counters
    // (solution implemented from: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/)
    fn configure_joltage_counters(&self) -> u64 {
        self.configure_joltage_counters_recursive(&self.joltage_reqs, 0).unwrap()
    }

    fn configure_joltage_counters_recursive(&self, joltages: &Vec<i64>, debug_level: usize) -> Option<u64> {
        let mut min_button_presses = None;

        // Convert joltages into light pattern:
        let lights_target = joltages.iter()
            .enumerate()
            .map(|(i,j)| if j % 2 == 0 { 0 } else { 1 << i } )
            .fold(0, |acc,x| acc + x);

        // Reduce required joltages by combining button presses that produce the required light pattern:
        // (same code as get_min_button_presses())
        let mut valid_combos = vec![];
        for i in 1..=self.buttons.len() {
            for combo in self.buttons.iter().combinations(i) {
                let result = combo.iter().fold(0u64, |acc, x| acc ^ *x);
                if result == lights_target {
                    // Valid combo:
                    valid_combos.push(combo.iter().cloned().cloned().collect_vec());
                }
            }
        }

        // If pattern is 0 (i.e, all joltages are even), add pressing zero buttons as valid combo
        if lights_target == 0 {
            valid_combos.push(vec![]);
        }

        for btns in valid_combos {
            // Reduce required joltage by the button presses:
            let mut new_joltages = joltages.clone();
            for (i, j) in new_joltages.iter_mut().enumerate() {
                for &b in btns.iter() {
                    if b & (1 << i) != 0 {
                        *j -= 1;
                    }
                }
            }

            if new_joltages.iter().any(|j| j < &0) {
                // Negative counters => invalid solution, try another combination
                continue;
            }

            let total_presses = if new_joltages.iter().all(|j| j == &0) {
                // Remaining joltage is 0: end condition
                Some(btns.len() as u64)
            } else {
                // if all values are even, divide by 2 and recurse
                for j in new_joltages.iter_mut() { *j /= 2; }
                let recursive_result = self.configure_joltage_counters_recursive(&new_joltages, debug_level + 1);
                if let Some(result_val) = recursive_result {
                    Some(btns.len() as u64 + (2 * result_val))
                } else { 
                    // No solution found:
                    None
                }
            };

            if min_button_presses.is_none() {
                min_button_presses = total_presses;
            } else if let Some(res) = total_presses {
                min_button_presses = Some(min(res, min_button_presses.unwrap()));
            }
        }

        min_button_presses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\r\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\r\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 7);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\r\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\r\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 33);
    }
}
