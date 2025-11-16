use std::collections::HashMap;

use crate::aoc_result::AoCResult;
use crate::shared::geometry::Point;
use crate::shared::intcode::{InstructionResult, IntcodeProgram};

make_day!(Day11);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
    let mut painting_robot = PaintingRobot::new();

    painting_robot.run_paint_prgm(&mut prgm);
    AoCResult::Num(painting_robot.get_painted_panels())
}

pub fn solve_part2(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);
    let mut painting_robot = PaintingRobot::new();

    // Start from white panel:
    painting_robot.paint(Color::White);

    painting_robot.run_paint_prgm(&mut prgm);
    painting_robot.print_to_console();
    AoCResult::PrintedToConsole
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    Black = 0,
    White = 1,
}

struct PaintingRobot {
    position: Point,
    direction: Direction,

    painting_area: HashMap<Point, Color>,
}

impl PaintingRobot {
    fn new() -> Self {
        PaintingRobot {
            position: Point { x: 0, y: 0 },
            direction: Direction::Up,
            painting_area: HashMap::<Point, Color>::new(),
        }
    }

    fn turn_and_move(&mut self, turn_dir: Direction) {
        // Turn:
        self.direction = match self.direction {
            Direction::Up if turn_dir == Direction::Right => Direction::Right,
            Direction::Up => Direction::Left,
            Direction::Down if turn_dir == Direction::Right => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left if turn_dir == Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
            Direction::Right if turn_dir == Direction::Right => Direction::Down,
            Direction::Right => Direction::Up,
        };

        // Move:
        self.position = match self.direction {
            Direction::Up => Point { x: self.position.x, y: self.position.y - 1 },
            Direction::Down => Point { x: self.position.x, y: self.position.y + 1 },
            Direction::Left => Point { x: self.position.x - 1, y: self.position.y },
            Direction::Right => Point { x: self.position.x + 1, y: self.position.y },
        };
    }

    fn get_camera_input(&self) -> Color {
        *self.painting_area.get(&self.position).unwrap_or(&Color::Black)
    }

    fn paint(&mut self, color: Color) {
        self.painting_area.entry(self.position).insert_entry(color);
    }

    fn get_painted_panels(&self) -> u64 {
        self.painting_area.len() as u64
    }

    fn run_paint_prgm(&mut self, prgm: &mut IntcodeProgram) -> bool {
        let mut prgm_state = prgm.run();
        while prgm_state == InstructionResult::WAIT_FOR_INPUT {
            // Provide camera input to the program:
            let camera_input = self.get_camera_input();
            prgm.input.push_back(camera_input as i64);
            prgm_state = prgm.run_continue();

            if prgm_state == InstructionResult::WAIT_FOR_INPUT {
                // Process output:
                let color = match prgm.output[0] {
                    0 => Color::Black,
                    1 => Color::White,
                    _ => unreachable!(),
                };
                self.paint(color);

                let turn_dir = match prgm.output[1] {
                    0 => Direction::Left,
                    1 => Direction::Right,
                    _ => unreachable!(),
                };
                self.turn_and_move(turn_dir);

                prgm.output.clear();
            } else if prgm_state != InstructionResult::HALT {
                // Error during processing
                return false;
            }
        }

        return true;
    }

    fn print_to_console(&self) {
        // Dimensions of the painting area:
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;

        let mut first_point = true;
        for (p,_) in self.painting_area.iter() {
            if first_point || p.x < x_min { x_min = p.x; }
            if first_point || p.x > x_max { x_max = p.x; }
            if first_point || p.y < y_min { y_min = p.y; }
            if first_point || p.y > y_max { y_max = p.y; }
            first_point = false;
        }

        let width = x_max - x_min + 1;
        let height = y_max - y_min + 1;
        let mut drawing = vec!['.'; (width * height) as usize];

        for (p, color) in self.painting_area.iter() {
            drawing[(p.y * width + p.x) as usize] = match color {
                Color::Black => '.',
                Color::White => '#',
            }
        }

        for line in drawing.chunks(width as usize) {
            println!("{}", line.iter().collect::<String>());
        }
    }
}
