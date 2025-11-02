use crate::aoc_result::AoCResult;
use crate::shared::intcode::IntcodeProgram;

make_day!(Day05);

pub fn solve_part1(input: &String) -> AoCResult {
   	let mut prgm = IntcodeProgram::new(input, None);

	prgm.input.push_back(1);
	prgm.run();

	let diagnostic_code = *prgm.output.last().unwrap();
	AoCResult::Num(u64::try_from(diagnostic_code).unwrap())
}

pub fn solve_part2(input: &String) -> AoCResult {
	AoCResult::NotImplemented
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
		// Program that outputs whatever it gets as input
		let mut prgm = IntcodeProgram::from(vec!(3,0,4,0,99));
		prgm.input.push_back(123);
		prgm.run();
        assert_eq!(prgm.output[0], 123);
    }

	#[test]
	fn part1_example2() {
		// Program that outputs whatever it gets as input
		let mut prgm = IntcodeProgram::from(vec!(1002,4,3,4,33));
		prgm.run();
        assert_eq!(prgm.read(4), 99);
    }

	#[test]
	fn part1_example3() {
		// Program that outputs whatever it gets as input
		let mut prgm = IntcodeProgram::from(vec!(1101,100,-1,4,0));
		prgm.run();
        assert_eq!(prgm.read(4), 99);
    }
}