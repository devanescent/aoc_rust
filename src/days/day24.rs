use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::aoc_result::AoCResult;

make_day!(Day24);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut area = input
        .trim_end()
        .lines()
        .map(str::to_owned)
        .collect_vec();

    let mut prev_layouts = HashSet::<String>::new();

    let result;
    loop {
        let layout = area.join("");
        if prev_layouts.contains(&layout) {
            // Calculate biodiversity:
            result = layout.chars()
                .enumerate()
                .map(|(i, c)| { if c == '#' { 1 << i } else { 0 } })
                .sum::<u64>();
            break;
        } else {
            prev_layouts.insert(layout);
        }

        area = process_minute(area);
    }

    AoCResult::Num(result)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let mut bugs = HashSet::<RecursiveCell>::new();
    for (y, line) in input.trim_end().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                bugs.insert(RecursiveCell { x: x as i32, y: y as i32, level: 0 });
            }
        }
    }

    for _ in 0..200 {
        bugs = process_minute_recursive(bugs);
    }

    AoCResult::Num(bugs.len() as u64)
}

fn process_minute(area: Vec<String>) -> Vec<String> {
    let mut next_area = Vec::<String>::new();

    let height = area.len() as i32;
    let width = area[0].len() as i32;

    for y in 0..height {
        let mut next_line = String::new();

        for x in 0..width {

            // Count adjacent bugs:
            let mut adjacent_bugs = 0;

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let y0 = y + dy;
                let x0 = x + dx;

                if y0 >= 0 && y0 < height && x0 >= 0 && x0 < width {
                    // Point is inside the area:
                    if area.get(y0 as usize).unwrap().chars().nth(x0 as usize).unwrap() == '#' {
                        adjacent_bugs += 1;
                    }
                }
            }

            if area.get(y as usize).unwrap().chars().nth(x as usize).unwrap() == '#' {
                // Current tile is a bug: dies unless there is exactly one bug adjacent to it
                next_line += if adjacent_bugs == 1 { "#" } else { "." }
            } else {
                // Current tile is empty: becomes infested if exactly one or two bugs are adjacent to it
                next_line += if adjacent_bugs == 1 || adjacent_bugs == 2 { "#" } else { "." }
            }
        }

        next_area.push(next_line);
    }

    return next_area;
}

fn process_minute_recursive(bugs: HashSet<RecursiveCell>) -> HashSet<RecursiveCell> {
   let mut new_bugs = HashMap::<RecursiveCell, u8>::new();

    for old_bug in bugs.iter() {
        let mut adjacent_bugs = 0;

        // Check neighbor cells:
        let mut check_neighbor = |x: i32, y: i32, level: i32| {
            let neighbor = RecursiveCell { x: x, y: y, level: level };
            if bugs.iter().contains(&neighbor) { 
                adjacent_bugs += 1;
            } else { 
                // If neighbor cell is empty, a bug could spawn there:
                new_bugs.entry(neighbor).and_modify(|e| *e += 1 ).or_insert(1);
            };
        };

        // Top: 
        if old_bug.y == 0 {
            // Top row in current 5x5 area: neighbor is from outer level
            check_neighbor(2, 1, old_bug.level - 1);
        } else if old_bug.y == 3 && old_bug.x == 2 {
            // directly below center tile: all bottom tiles of the recursive inner cell are adjacent to this bug:
            for inner_x in 0..5 {
                check_neighbor(inner_x, 4, old_bug.level + 1);
            }
        } else {
            // Cell above
            check_neighbor(old_bug.x, old_bug.y - 1, old_bug.level);
        }

        // Down
        if old_bug.y == 4 {
            // Bottom row in current 5x5 area: neighbor is from outer level
            check_neighbor(2, 3, old_bug.level - 1);
        } else if old_bug.y == 1 && old_bug.x == 2 {
            // directly above center tile: all upper tiles of the recursive inner cell are adjacent to this bug:
            for inner_x in 0..5 {
                check_neighbor(inner_x, 0, old_bug.level + 1);
            }
        } else {
            // Cell below
            check_neighbor(old_bug.x, old_bug.y + 1, old_bug.level);
        }
        
        // Left
        if old_bug.x == 0 {
            // Leftmost row in current 5x5 area: neighbor is from outer level
            check_neighbor(1, 2, old_bug.level - 1);
        } else if old_bug.y == 2 && old_bug.x == 3 {
            // directly right from center tile: all rightmost tiles of the recursive inner cell are adjacent to this bug:
            for inner_y in 0..5 {
                check_neighbor(4, inner_y, old_bug.level + 1);
            }
        } else {
            // Cell to the left
            check_neighbor(old_bug.x - 1, old_bug.y, old_bug.level);
        }

        // Right
        if old_bug.x == 4 {
            // Rightmost row in current 5x5 area: neighbor is from outer level
            check_neighbor(3, 2, old_bug.level - 1);
        } else if old_bug.y == 2 && old_bug.x == 1 {
            // directly left from center tile: all leftmost tiles of the recursive inner cell are adjacent to this bug:
            for inner_y in 0..5 {
                check_neighbor(0, inner_y, old_bug.level + 1);
            }
        } else {
            // Cell to the right
            check_neighbor(old_bug.x + 1, old_bug.y, old_bug.level);
        }

        if adjacent_bugs == 1 {
            // Bug survives:
            new_bugs.insert(old_bug.clone(), 1);
        }
    }

    // From the new bugs: only those with 1 or 2 neighbors survive:
    new_bugs.into_iter()
        .filter(|(_, count)| { count == &1 || count == &2 })
        .map(|(cell, _)| cell)
        .collect()
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct RecursiveCell {
    x: i32,
    y: i32,
    level: i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("....#\r\n#..#.\r\n#..##\r\n..#..\r\n#....");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 2129920);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("....#\r\n#..#.\r\n#..##\r\n..#..\r\n#....");
        
        let mut bugs = HashSet::<RecursiveCell>::new();
        for (y, line) in input.trim_end().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    bugs.insert(RecursiveCell { x: x as i32, y: y as i32, level: 0 });
                }
            }
        }

        for _ in 0..10 {
            bugs = process_minute_recursive(bugs);
        }
        
        assert_eq!(bugs.len(), 99);
    }
}
