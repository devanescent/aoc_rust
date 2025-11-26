use aoc_core::AoCResult;
use crate::shared::intcode::{IntcodeProgram, RunMode};

aoc_core::make_day!(Day17);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
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

    let mut alignment_sum = 0;
    for y in 0..camera_view.len() {
        if y == 0 { continue; }
        for x in 0..camera_view.first().unwrap().len() {
            if x == 0 { continue; }

            // If current position is scaffold:
            if let Some(pos) = camera_view[y].chars().nth(x) && pos == '#' {
                // Check all neighbors to find an intersection:
                if let Some(up) = camera_view.get(y - 1) && up.chars().nth(x).unwrap() == '#' {
                    if let Some(left) = camera_view[y].chars().nth(x - 1) && left == '#' {
                        if let Some(right) = camera_view[y].chars().nth(x + 1) && right == '#' {
                            if let Some(down) = camera_view.get(y + 1) && down.chars().nth(x).unwrap() == '#' {
                                alignment_sum += y * x;
                            }
                        }
                    }
                }
            }
        }
    }

    AoCResult::Num(alignment_sum as u64)
}

pub fn solve_part2(input: &String) -> AoCResult {
    // Manually solved for commands to traverse the scaffold and split into subroutines:
    // L,6,R,12,L,4,L,6,R,6,L,6,R,12,R,6,L,6,R,12,L,6,L,10,L,10,R,6,L,6,R,12,L,4,L,6,R,6,L,6,R,12,L,6,L,10,L,10,R,6,L,6,R,12,L,4,L,6,R,6,L,6,R,12,L,6,L,10,L,10,R,6
    // ------A-------                                               ------A-------                                  ------A-------
    // L,6,R,12,L,4,L                                               L,6,R,12,L,4,L                                  L,6,R,12,L,4,L
    //                ------B------             ---------C---------                ------B------                                   ------B------
    //                6,R,6,L,6,R,6             6,L,6,L,10,L,10,R,6                6,R,6,L,6,R,6                                   6,R,6,L,6,R,6
    //                             ------B------                                                ---------C---------                             ---------C---------
    //                             6,R,6,L,6,R,6                                                6,L,6,L,10,L,10,R,6                             6,L,6,L,10,L,10,R,6

    let func_a = "L,6,R,12,L,4,L";
    let func_b = "6,R,6,L,6,R,6";
    let func_c = "6,L,6,L,10,L,10,R,6";

    let func_main = "A,B,B,C,A,B,C,A,B,C";

    let mut prgm = IntcodeProgram::new(input, None);
    prgm.write(0, 2);

    // Provide routines as ascii code:
    let functions = [func_main, func_a, func_b, func_c];
    for func in functions {
        for c in func.chars() {
            let ascii_value = c as u8;
            prgm.input.push_back(ascii_value as i64);
        }
        prgm.input.push_back(10); // Seperate by newline
    }

    // Disable video feed:
    prgm.input.push_back(('n' as u8) as i64);
    prgm.input.push_back(10);

    prgm.run(RunMode::Free);
    let res = *prgm.output.last().unwrap();
    AoCResult::Num(res as u64)
}
