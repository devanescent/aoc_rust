use crate::aoc_result::AoCResult;
use std::str::FromStr;

pub fn solve_part1(input: &String) -> AoCResult {
   	let mut prgm : Vec<u64> = input
	   	.trim_end()
        .split(',')
		.map(|s| u64::from_str(s).expect(format!("Expected u64 in program, found '{}'", s).as_str()))
		.collect();

	// Do replacements and run program:
	prgm[1] = 12;
	prgm[2] = 2;
	run_program(&mut prgm);

	AoCResult::Num(prgm[0])
}

fn run_program(prgm: &mut Vec<u64>) {
	let mut pos = 0;
	let mut op = prgm[pos];

	while op != 99 {
		let arg1 = usize::try_from(prgm[pos + 1]).unwrap();
		let arg2 = usize::try_from(prgm[pos + 2]).unwrap();
		let arg3= usize::try_from(prgm[pos + 3]).unwrap();
		prgm[arg3] = match op {
			1 => prgm[arg1] + prgm[arg2],
			2 => prgm[arg1] * prgm[arg2],
			_ => panic!("Invalid program found")
		};

		pos += 4;
		op = prgm[pos];
	}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let mut prgm: Vec<u64> = vec!(1,9,10,3,2,3,11,0,99,30,40,50);
        run_program(&mut prgm);
        assert_eq!(prgm[0], 3500);
    }
}