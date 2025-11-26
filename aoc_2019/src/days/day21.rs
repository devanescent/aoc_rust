use aoc_core::AoCResult;
use crate::shared::intcode::{IntcodeProgram, RunMode};

aoc_core::make_day!(Day21);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
    
    // Jump, if one of the three tiles before the robot contain a hole and the tile at distance 4 is solid ground
    prgm.input_ascii("NOT A J\n");
    prgm.input_ascii("NOT B T\n");
    prgm.input_ascii("OR T J\n");
    prgm.input_ascii("NOT C T\n");
    prgm.input_ascii("OR T J\n");
    prgm.input_ascii("AND D J\n");
    prgm.input_ascii("WALK\n");
    prgm.run(RunMode::Free);

    let mut camera_view: Vec<String> = vec![];
    for out_line in prgm.output.chunk_by(|a, _| a != &10) {
        let camera_line = out_line
            .iter()
            .take_while(|c| c != &&10) // ignore line break at the end
            .map(|x| (*x as u8) as char)
            .collect::<String>();

        println!("{}", camera_line);
        camera_view.push(camera_line);
    }

    AoCResult::Num(*prgm.output.last().unwrap() as u64)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
    
    // Jump, if one of the three tiles before the robot contain a hole and the tiles at distance 4 and distance 8 are solid ground
    // Also, always jump if the tile directly before the robot is a hole
    prgm.input_ascii("NOT A J\n");
    prgm.input_ascii("NOT B T\n");
    prgm.input_ascii("OR T J\n");
    prgm.input_ascii("NOT C T\n");
    prgm.input_ascii("OR T J\n");
    prgm.input_ascii("AND D J\n");
    prgm.input_ascii("AND H J\n");
    prgm.input_ascii("NOT A T\n");
    prgm.input_ascii("OR T J\n");
    prgm.input_ascii("RUN\n");
    prgm.run(RunMode::Free);

    let mut camera_view: Vec<String> = vec![];
    for out_line in prgm.output.chunk_by(|a, _| a != &10) {
        let camera_line = out_line
            .iter()
            .take_while(|c| c != &&10) // ignore line break at the end
            .map(|x| (*x as u8) as char)
            .collect::<String>();

        println!("{}", camera_line);
        camera_view.push(camera_line);
    }

    AoCResult::Num(*prgm.output.last().unwrap() as u64)
}
