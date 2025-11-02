use crate::aoc_result::AoCResult;
use crate::shared::intcode::IntcodeProgram;

make_day!(Day02);

pub fn solve_part1(input: &String) -> AoCResult {
   	let mut prgm = IntcodeProgram::new(input, None);

	// Do replacements and run program:
	prgm.write(1, 12);
	prgm.write(2, 2);
	prgm.run();

	AoCResult::Num(prgm.read(0))
}

pub fn solve_part2(input: &String) -> AoCResult {
	let prgm = IntcodeProgram::new(input, None);

	// Try replacements until output 19690720 is found
	for noun in 0..99 {
		for verb in 0..99 {
			// Use a fresh copy for each attempt:
			let mut prgm_cpy = prgm.clone();
			prgm_cpy.write(1, noun);
			prgm_cpy.write(2, verb);

			prgm_cpy.run();
			if prgm_cpy.read(0) == 19690720 {
				return AoCResult::Num(u64::try_from(100 * noun + verb).unwrap());
			}
		}
	}

	panic!("No solution found");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let mut prgm = IntcodeProgram::from(vec!(1,9,10,3,2,3,11,0,99,30,40,50));
        prgm.run();
        assert_eq!(prgm.read(0), 3500);
    }
}