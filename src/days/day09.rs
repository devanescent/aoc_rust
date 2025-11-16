use crate::aoc_result::AoCResult;
use crate::shared::intcode::IntcodeProgram;

make_day!(Day09);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
    prgm.input.push_back(1);
    prgm.run();

    let boost_keycode = *prgm.output.last().unwrap();
    AoCResult::Num(u64::try_from(boost_keycode).unwrap())
}

pub fn solve_part2(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
    prgm.input.push_back(2);
    prgm.run();

    let coordinates = *prgm.output.last().unwrap();
    AoCResult::Num(u64::try_from(coordinates).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
        let mut prgm = IntcodeProgram::from(input.clone());
        prgm.run();

        assert_eq!(input, prgm.output);
    }

    #[test]
    fn part1_example2() {
        let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut prgm = IntcodeProgram::from(input.clone());
        prgm.run();
        assert_eq!(16, prgm.output[0].to_string().len());
    }

    #[test]
    fn part1_example3() {
        let input = vec![104, 1125899906842624, 99];
        let mut prgm = IntcodeProgram::from(input.clone());
        prgm.run();
        assert_eq!(1125899906842624, prgm.output[0]);
    }
}
