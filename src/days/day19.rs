use crate::aoc_result::AoCResult;
use crate::shared::intcode::{IntcodeProgram, RunMode};

make_day!(Day19);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut points_affected_by_tractor_beam = 0;
    let mut lines = Vec::<String>::new(); // for debug print

    for y in 0..50 {
        let mut line = String::new();
        for x in 0..50 {
            if is_point_in_tractor_beam(&input, x, y) {
                points_affected_by_tractor_beam += 1;
                line.push('#');
            } else {
                line.push('.');
            }
        }
        lines.push(line);
    }

    for line in lines {
        println!("{}", line);
    }

    AoCResult::Num(points_affected_by_tractor_beam)
}

pub fn solve_part2(input: &String) -> AoCResult {
    // Start somewhere to the left of the tractor beam:
    let mut x = 50;
    let mut y = 100;

    loop {
        // Check if bounding box is fully inside the tractor beam:
        let mut square_fits = true;
        'bounds_loop: for dx in 0..100 {
            let y_step = if dx == 0 || dx == 99 { 1 } else { 99 };
            for dy in (0..100).step_by(y_step) {
                if !is_point_in_tractor_beam(&input, x + dx, y + dy) {
                    square_fits = false;
                    if dy == 0 && dx == 0 {
                        // Move right until the top left corner is within the tractor beam:
                        x += 1;
                    } else if dx == 0 {
                        // Left edge is not fully within the beam:

                        // Is the top right corner within the beam?
                        if is_point_in_tractor_beam(&input, x + 99, y + dy) {
                            // Move further to the left
                            x += 1;
                        } else {
                            // Otherwise, the beam is not wide enough: move down
                            y += 1;
                        }
                    } else {
                        // Left border is within the beam, but the beam is not wide enough: move down
                        y += 1;
                    }

                    break 'bounds_loop;
                }
            }
        }

        if square_fits {
            break;
        }
    }

    AoCResult::Num((x * 10000 + y) as u64)
}

fn is_point_in_tractor_beam(input: &String, x: i64, y: i64) -> bool {
    let mut prgm: IntcodeProgram = IntcodeProgram::new(input, None);
    prgm.input.push_back(x);
    prgm.input.push_back(y);
    prgm.run(RunMode::Free);
    prgm.output[0] == 1
}
