use std::collections::{HashMap, HashSet, VecDeque};

use crate::aoc_result::AoCResult;
use crate::shared::geometry::Point;
use crate::shared::intcode::IntcodeProgram;

make_day!(Day15);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut prgm = IntcodeProgram::new(input, None);

    // Movement of the robot, stack-based depth-first search:
    let mut movement_stack = vec![Point::new(0, 0)];

    // Locations the robot visited:
    let mut visited_locations = HashMap::<Point, StatusCode>::new();

    let mut distance_moved = 0u64;
    let mut distance_to_oxygen = 0u64;

    // Start program:
    prgm.run();

    while !movement_stack.is_empty() {
        // Current top of the stack:
        let current_pos = movement_stack.last().unwrap().clone();

        // Keep track of explored locations:
        visited_locations.entry(current_pos.clone()).or_insert(StatusCode::Empty);

        // Try moving in a new direction:
        let next_pos: Point;
        let mut moved_backwards = false;

        let n = current_pos.move_by(0, -1);
        if !visited_locations.contains_key(&n) {
            // Try north:
            next_pos = n;
        } else {
            let e = current_pos.move_by(1, 0);
            if !visited_locations.contains_key(&e) {
                // Try east:
                next_pos = e;
            } else {
                let s = current_pos.move_by(0, 1);
                if !visited_locations.contains_key(&s) {
                    // Try south:
                    next_pos = s;
                } else {
                    let w = current_pos.move_by(-1, 0);
                    if !visited_locations.contains_key(&w) {
                        // Try west:
                        next_pos = w;
                    } else {
                        // Move back to previous position:
                        movement_stack.pop();
                        if movement_stack.is_empty() { 
                            break;
                        } else {
                            next_pos = movement_stack.last().unwrap().clone();
                            moved_backwards = true;
                        }
                    }
                }
            }
        }

        // Convert next direction into input value:
        match current_pos.distance_to(&next_pos) {
            Point{x: 0, y: -1} => { prgm.input.push_back(1); }
            Point{x: 0, y: 1} => { prgm.input.push_back(2); }
            Point{x: 1, y: 0} => { prgm.input.push_back(4); }
            Point{x: -1, y: 0} => { prgm.input.push_back(3); }
            _ => panic!("Skipped a step!")
        }

        prgm.run_continue();

        // Check output:
        if !prgm.output.is_empty() {
            match prgm.output[0] {
                1 => {
                    // Moved
                    if !moved_backwards {
                        distance_moved += 1;
                        movement_stack.push(next_pos.clone());
                    } else {
                        distance_moved -= 1;
                    }
                }
                2 => {
                    // Oxygen system
                    if !moved_backwards {
                        distance_moved += 1;
                        movement_stack.push(next_pos.clone());
                    } else {
                        distance_moved -= 1;
                    }

                    distance_to_oxygen = distance_moved;
                    visited_locations.insert(next_pos.clone(), StatusCode::OxygenSystem);
                }
                _ => { 
                    // Wall / other
                    visited_locations.insert(next_pos.clone(), StatusCode::Wall);
                }
            }

            prgm.output.clear();
        }
    }

    print_to_console(&visited_locations);

    AoCResult::Num(distance_to_oxygen)
}

pub fn solve_part2(input: &String) -> AoCResult {
   let mut prgm = IntcodeProgram::new(input, None);

    // Movement of the robot, stack-based depth-first search:
    let mut movement_stack = vec![Point::new(0, 0)];

    // Locations the robot visited:
    let mut visited_locations = HashMap::<Point, StatusCode>::new();

    // Start program to build map:
    prgm.run();

    while !movement_stack.is_empty() {
        // Current top of the stack:
        let current_pos = movement_stack.last().unwrap().clone();

        // Keep track of explored locations:
        visited_locations.entry(current_pos.clone()).or_insert(StatusCode::Empty);

        // Try moving in a new direction:
        let next_pos: Point;
        let mut moved_backwards = false;

        let n = current_pos.move_by(0, -1);
        if !visited_locations.contains_key(&n) {
            // Try north:
            next_pos = n;
        } else {
            let e = current_pos.move_by(1, 0);
            if !visited_locations.contains_key(&e) {
                // Try east:
                next_pos = e;
            } else {
                let s = current_pos.move_by(0, 1);
                if !visited_locations.contains_key(&s) {
                    // Try south:
                    next_pos = s;
                } else {
                    let w = current_pos.move_by(-1, 0);
                    if !visited_locations.contains_key(&w) {
                        // Try west:
                        next_pos = w;
                    } else {
                        // Move back to previous position:
                        movement_stack.pop();
                        if movement_stack.is_empty() { 
                            break;
                        } else {
                            next_pos = movement_stack.last().unwrap().clone();
                            moved_backwards = true;
                        }
                    }
                }
            }
        }

        // Convert next direction into input value:
        match current_pos.distance_to(&next_pos) {
            Point{x: 0, y: -1} => { prgm.input.push_back(1); }
            Point{x: 0, y: 1} => { prgm.input.push_back(2); }
            Point{x: 1, y: 0} => { prgm.input.push_back(4); }
            Point{x: -1, y: 0} => { prgm.input.push_back(3); }
            _ => panic!("Skipped a step!")
        }

        prgm.run_continue();

        // Check output:
        if !prgm.output.is_empty() {
            match prgm.output[0] {
                1 => {
                    // Moved
                    if !moved_backwards {
                        movement_stack.push(next_pos.clone());
                    }
                }
                2 => {
                    // Oxygen system
                    if !moved_backwards {
                        movement_stack.push(next_pos.clone());
                    }

                    visited_locations.insert(next_pos.clone(), StatusCode::OxygenSystem);
                }
                _ => { 
                    // Wall / other
                    visited_locations.insert(next_pos.clone(), StatusCode::Wall);
                }
            }

            prgm.output.clear();
        }
    }

    // Oxygen spread:
    let mut oxygen = HashSet::<Point>::new();
    let oxygen_start = visited_locations.iter().find(|x| x.1 == &StatusCode::OxygenSystem).unwrap().0;
    oxygen.insert(oxygen_start.clone());

    // BFS for oxygen spread with minute the position was reached:
    let mut oxigen_queue = VecDeque::<(Point, u64)>::new();
    oxigen_queue.push_back((oxygen_start.clone(), 0));

    let mut max_time = 0;
    while !oxigen_queue.is_empty() {
        let (current_pos, time) = oxigen_queue.pop_front().unwrap();

        max_time = std::cmp::max(max_time, time);

        // Find all neighbors from visited set:
        let nesw = vec![
            current_pos.move_by(0, -1),
            current_pos.move_by(1, 0),
            current_pos.move_by(0, 1),
            current_pos.move_by(-1, 0)
        ];

        for neighbor in nesw {
            if !oxygen.contains(&neighbor) && visited_locations.iter().find(|x| x.1 == &StatusCode::Empty && x.0 == &neighbor).is_some() {
                oxygen.insert(neighbor.clone());
                oxigen_queue.push_back((neighbor, time + 1));
            }
        }
    }

    AoCResult::Num(max_time)

}

#[derive(PartialEq)]
enum StatusCode {
    Empty,
    Wall,
    OxygenSystem
}

fn print_to_console(points: &HashMap<Point, StatusCode>) {
        // Dimensions of the painting area:
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;

        let mut first_point = true;
        for (p, _) in points.iter() {
            if first_point || p.x < x_min { x_min = p.x; }
            if first_point || p.x > x_max { x_max = p.x; }
            if first_point || p.y < y_min { y_min = p.y; }
            if first_point || p.y > y_max { y_max = p.y; }
            first_point = false;
        }

        let width = x_max - x_min + 1;
        let height = y_max - y_min + 1;
        let mut drawing = vec![' '; (width * height) as usize];

        for (p, status) in points.iter() {
            drawing[((p.y - y_min) * width + (p.x - x_min)) as usize] = match status {
                StatusCode::Empty if p.x == 0 && p.y == 0 => 'S',
                StatusCode::Empty => '.',
                StatusCode::Wall => '#',
                StatusCode::OxygenSystem => 'O'
            }
        }

        for line in drawing.chunks(width as usize) {
            println!("{}", line.iter().collect::<String>());
        }
    }