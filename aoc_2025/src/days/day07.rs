use std::collections::{HashSet, HashMap, VecDeque};

use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day07);

pub fn solve_part1(input: &String) -> AoCResult {
    let manifold = TachyonManifold::new(input);

    let mut visited = HashSet::<(u64,u64)>::new();
    let mut queue = VecDeque::new();
    queue.push_back(manifold.get_start());

    let mut splits = 0;
    while !queue.is_empty() {
        let cur_pos = queue.pop_front().unwrap();

        if !manifold.is_pos_split(&cur_pos) {
            // Move downwards:
            let next_pos = (cur_pos.0, cur_pos.1 + 1);
            if manifold.is_valid(&next_pos) && visited.insert(next_pos) {
                queue.push_back(next_pos);
            }
        } else {
            // Split left and right:
            splits += 1;

            let left_pos = (cur_pos.0 - 1, cur_pos.1 + 1);
            if visited.insert(left_pos) {
                queue.push_back(left_pos);
            }

            let right_pos = (cur_pos.0 + 1, cur_pos.1 + 1);
            if visited.insert(right_pos) {
                queue.push_back(right_pos);
            }
        }
    }

    AoCResult::Num(splits)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let manifold = TachyonManifold::new(input);

    // Count visits at each point (with number of timelines):
    let mut visited = HashMap::<(u64,u64), u64>::new();
    let mut queue = VecDeque::new();
    queue.push_back(manifold.get_start());

    let mut timelines = 0;
    while !queue.is_empty() {
        let cur_pos= queue.pop_front().unwrap();
        let cur_timelines = *visited.get(&cur_pos).unwrap_or(&1);

        if !manifold.is_pos_split(&cur_pos) {
            // Move downwards:
            let next_pos = (cur_pos.0, cur_pos.1 + 1);
            if manifold.is_valid(&next_pos) {
                visited.entry(next_pos)
                    .and_modify(|e| *e += cur_timelines)
                    .or_insert_with(|| {
                        queue.push_back(next_pos);
                        cur_timelines
                    });
            } else {
                timelines += cur_timelines;
            }
        } else {
            // Split left and right:
            let left_pos = (cur_pos.0 - 1, cur_pos.1 + 1);
            visited.entry(left_pos)
                .and_modify(|e| *e += cur_timelines)
                .or_insert_with(|| {
                    queue.push_back(left_pos);
                    cur_timelines
                });

            let right_pos = (cur_pos.0 + 1, cur_pos.1 + 1);
            visited.entry(right_pos)
                .and_modify(|e| *e += cur_timelines)
                .or_insert_with(|| {
                    queue.push_back(right_pos);
                    cur_timelines
                });
        }
    }

    AoCResult::Num(timelines)
}

struct TachyonManifold {
    width: u64,
    manifold: Vec<char>,
}

impl TachyonManifold {
    fn new(input: &String) -> Self {
        TachyonManifold { 
            width: input.find('\r').unwrap() as u64,
            manifold: input
                .trim_end()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect_vec()
         }
    }

    fn get_start(&self) -> (u64,u64) {
        let start_index = self.manifold.iter().find_position(|&c| c == &'S').unwrap().0 as u64;
        (start_index % self.width, start_index / self.width)
    }

    fn is_valid(&self, pos: &(u64, u64)) -> bool {
        ((pos.1 * self.width + pos.0) as usize) < self.manifold.len()
    }

    fn is_pos_split(&self, pos: &(u64, u64)) -> bool {
        self.manifold[(pos.1 * self.width + pos.0) as usize] == '^'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from(".......S.......\r\n...............\r\n.......^.......\r\n...............\r\n......^.^......\r\n...............\r\n.....^.^.^.....\r\n...............\r\n....^.^...^....\r\n...............\r\n...^.^...^.^...\r\n...............\r\n..^...^.....^..\r\n...............\r\n.^.^.^.^.^...^.\r\n...............");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 21);
    }

    #[test]
    fn part2_example1() {
        let input = String::from(".......S.......\r\n...............\r\n.......^.......\r\n...............\r\n......^.^......\r\n...............\r\n.....^.^.^.....\r\n...............\r\n....^.^...^....\r\n...............\r\n...^.^...^.^...\r\n...............\r\n..^...^.....^..\r\n...............\r\n.^.^.^.^.^...^.\r\n...............");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 40);
    }
}
