use std::io;
use crate::aoc_result::AoCResult;
use crate::shared::intcode::{InstructionResult, IntcodeProgram, RunMode};

make_day!(Day25);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
    let mut state = prgm.run(RunMode::Step);


    // Map build from playing:
    //
    //                                                     +------------+
    // +-------------+     +----------+     +---------+    |  Security  |     +-------------+      +---------+
    // | Warp Drive  |     | Sick Bay |     | Stables |    | Checkpoint |-----| Engineering |      | Storage |
    // | Maintenance |-----|          |-----|         |    +------------+     +-------------+      +---------+
    // +-------------+     +----------+     +---------+           |                 |                   |
    //                                           |            +---------+     +----------+        +-----------+      +-------------+     +----------+
    //                      +---------+          |            | Cockpit |     |   Gift   |        |    Crew   |------| Observatory |-----| Holodeck |
    //                      | Hallway |          |            +---------+     | Wrapping |--------|  Quarters |      +-------------+     +----------+
    //                      +---------+          |        +-----------+       |  Center  |        +-----------+           |
    //                           |               |        |    Hot    |       +----------+                                |
    //                           |               |        | Chocolate |--+        |                                       |
    //                           |               |        | Fountain  |  |   +---------+                                  |
    //                    +------------+    +---------+   +-----------+  +---| Kitchen |                             +---------+
    //                    | Navigation |    |   Hull  |                      +---------+                             | Science |        +----------+
    //                    |            |----|  Breach |--------------------------------------------------------------|   Lab   |--------| Passages |
    //                    +------------+    +---------+                                                              +---------+        +----------+
    //                           |
    //                           |
    //                      +--------+     +----------+
    //                      | Arcade |-----| Corridor |
    //                      +--------+     +----------+

    loop {
        match state {
            InstructionResult::HALT => { break; }
            InstructionResult::WAIT_FOR_INPUT => {
                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_ok() {
                    prgm.input_ascii(input.replace("\r", "").as_str());
                }
            }
            InstructionResult::RUNNING => {
                // Check for output characters:
                if !prgm.output.is_empty() {
                    print!("{}", (prgm.output[0] as u8) as char);
                    prgm.output.clear();
                }
            }
            _ => { }
        };

        state = prgm.run_step();
    }

    AoCResult::PrintedToConsole
}

pub fn solve_part2(_input: &String) -> AoCResult { AoCResult::NotImplemented }