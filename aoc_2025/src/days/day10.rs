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
    AoCResult::NotImplemented
}


struct Machine {
    lights_target: u64,
    buttons: Vec<u64>,
    joltage_reqs: Vec<u64>
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
        assert_eq!(u64::from(result), 7);
    }
}
