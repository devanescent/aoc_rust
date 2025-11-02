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
	let mut prgm = IntcodeProgram::new(input, None);

	prgm.input.push_back(5);
	prgm.run();

	let diagnostic_code = prgm.output[0];
	AoCResult::Num(u64::try_from(diagnostic_code).unwrap())
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
		let mut prgm = IntcodeProgram::from(vec!(1002,4,3,4,33));
		prgm.run();
        assert_eq!(prgm.read(4), 99);
    }

	#[test]
	fn part1_example3() {
		let mut prgm = IntcodeProgram::from(vec!(1101,100,-1,4,0));
		prgm.run();
        assert_eq!(prgm.read(4), 99);
    }

	#[test]
    fn part2_example1() {
		// Program that outputs 1 if input is equal to 8, 0 otherwise
		let code = vec!(3,9,8,9,10,9,4,9,99,-1,8);

		let mut prgm1 = IntcodeProgram::from(code.clone());
		prgm1.input.push_back(8);
		prgm1.run();
        assert_eq!(prgm1.output[0], 1);

		let mut prgm2 = IntcodeProgram::from(code);
		prgm2.input.push_back(42);
		prgm2.run();
        assert_eq!(prgm2.output[0], 0);
    }

	#[test]
    fn part2_example2() {
		// Program that outputs 1 if input is less than 8, 0 otherwise
		let code = vec!(3,9,7,9,10,9,4,9,99,-1,8);

		let mut prgm1 = IntcodeProgram::from(code.clone());
		prgm1.input.push_back(4);
		prgm1.run();
        assert_eq!(prgm1.output[0], 1);

		let mut prgm2 = IntcodeProgram::from(code);
		prgm2.input.push_back(42);
		prgm2.run();
        assert_eq!(prgm2.output[0], 0);
    }

	#[test]
    fn part2_example3() {
		// Program that outputs 1 if input is equal to 8, 0 otherwise
		let code = vec!(3,3,1108,-1,8,3,4,3,99);

		let mut prgm1 = IntcodeProgram::from(code.clone());
		prgm1.input.push_back(8);
		prgm1.run();
        assert_eq!(prgm1.output[0], 1);

		let mut prgm2 = IntcodeProgram::from(code);
		prgm2.input.push_back(42);
		prgm2.run();
        assert_eq!(prgm2.output[0], 0);
    }

	#[test]
    fn part2_example4() {
		// Program that outputs 1 if input is less than 8, 0 otherwise
		let code = vec!(3,3,1107,-1,8,3,4,3,99);

		let mut prgm1 = IntcodeProgram::from(code.clone());
		prgm1.input.push_back(4);
		prgm1.run();
        assert_eq!(prgm1.output[0], 1);

		let mut prgm2 = IntcodeProgram::from(code);
		prgm2.input.push_back(42);
		prgm2.run();
        assert_eq!(prgm2.output[0], 0);
    }

	#[test]
    fn part2_example5() {
		// Program that outputs 0 if the input was zero, 1 otherwise
		let code = vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9);

		let mut prgm1 = IntcodeProgram::from(code.clone());
		prgm1.input.push_back(0);
		prgm1.run();
        assert_eq!(prgm1.output[0], 0);

		let mut prgm2 = IntcodeProgram::from(code);
		prgm2.input.push_back(42);
		prgm2.run();
        assert_eq!(prgm2.output[0], 1);
    }

	#[test]
    fn part2_example6() {
		// Program that outputs 0 if the input was zero, 1 otherwise
		let code = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);

		let mut prgm1 = IntcodeProgram::from(code.clone());
		prgm1.input.push_back(0);
		prgm1.run();
        assert_eq!(prgm1.output[0], 0);

		let mut prgm2 = IntcodeProgram::from(code);
		prgm2.input.push_back(42);
		prgm2.run();
        assert_eq!(prgm2.output[0], 1);
    }

	#[test]
    fn part2_example7() {
		// Program that outputs 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8
		let code = vec!(3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99);

		let mut prgm1 = IntcodeProgram::from(code.clone());
		prgm1.input.push_back(7);
		prgm1.run();
        assert_eq!(prgm1.output[0], 999);

		let mut prgm2 = IntcodeProgram::from(code.clone());
		prgm2.input.push_back(8);
		prgm2.run();
        assert_eq!(prgm2.output[0], 1000);

		let mut prgm3 = IntcodeProgram::from(code);
		prgm3.input.push_back(9);
		prgm3.run();
        assert_eq!(prgm3.output[0], 1001);
    }
}