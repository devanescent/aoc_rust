use crate::aoc_result::AoCResult;
use crate::shared::intcode::{InstructionResult, IntcodeProgram, RunMode};
use itertools::Itertools;

make_day!(Day07);

pub fn solve_part1(input: &String) -> AoCResult {
    let prgm = IntcodeProgram::new(input, None);
    let avail_phase_settings = vec![0, 1, 2, 3, 4];

    let mut max_thruster_output = 0;
    for phase_settings_perm in avail_phase_settings.iter().permutations(5) {
        let mut prev_output = 0;

        for phase in phase_settings_perm {
            let mut amp_prgm = prgm.clone();
            amp_prgm.input.push_back(*phase);
            amp_prgm.input.push_back(prev_output);
            amp_prgm.run(RunMode::Free);
            prev_output = amp_prgm.output[0];
        }

        max_thruster_output = std::cmp::max(max_thruster_output, prev_output);
    }

    AoCResult::Num(u64::try_from(max_thruster_output).unwrap())
}

pub fn solve_part2(input: &String) -> AoCResult {
    let prgm = IntcodeProgram::new(input, None);
    let avail_phase_settings = vec![5, 6, 7, 8, 9];

    let mut max_thruster_output = 0;
    for phase_settings_perm in avail_phase_settings.iter().permutations(5) {
        let mut amp_prgms = vec![
            prgm.clone(),
            prgm.clone(),
            prgm.clone(),
            prgm.clone(),
            prgm.clone(),
        ];

        let mut amp_states = vec![
            InstructionResult::RUNNING,
            InstructionResult::RUNNING,
            InstructionResult::RUNNING,
            InstructionResult::RUNNING,
            InstructionResult::RUNNING,
        ];

        // Set initial inputs and start running each amp;
        for i in 0..=4 {
            amp_prgms[i].input.push_back(*phase_settings_perm[i]);
            amp_states[i] = amp_prgms[i].run(RunMode::Free);
        }

        // Move data between amps and run until the last one halts:
        while amp_states[4] != InstructionResult::HALT {
            for i in 0..=4 {
                // Fetch output from prev amp:
                let prev_index = if i > 0 { i - 1 } else { 4 };
                let prev_output = *(amp_prgms[prev_index].output.get(0).unwrap_or(&0));

                // Assign as input to the current amp:
                amp_prgms[i].input.push_back(prev_output);
                amp_prgms[prev_index].output.clear();

                // Continue program:
                amp_states[i] = amp_prgms[i].run_continue();
            }
        }

        max_thruster_output = std::cmp::max(max_thruster_output, amp_prgms[4].output[0]);
    }

    AoCResult::Num(u64::try_from(max_thruster_output).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 43210);
    }

    #[test]
    fn part1_example2() {
		let input = String::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 54321);
    }

    #[test]
    fn part1_example3() {
		let input = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 65210);
    }

    #[test]
    fn part2_example1() {
		let input = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 139629729);
    }

    #[test]
    fn part2_example2() {
		let input = String::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 18216);
    }
}
