use aoc_core::AoCResult;
use std::collections::HashSet;
use itertools::Itertools;

aoc_core::make_day!(Day04);

pub fn solve_part1(input: &String) -> AoCResult {
    let grid_width = input.find('\r').unwrap();

    let mut paper_rolls = HashSet::<(i32, i32)>::new();
    for (i, c) in input.chars().filter(|c| c != &'\r' && c != &'\n').enumerate() {
        if c == '@' {
            paper_rolls.insert(((i % grid_width) as i32, (i / grid_width) as i32));
        }
    }

    // Find paper rolls with fewer than 4 neighbors:
    let result = paper_rolls.iter()
        .filter(|(px, py)| has_fewer_than_4_neighbors(&paper_rolls, px, py))
        .count();

    AoCResult::Num(result as u64)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let mut paper_rolls = get_paper_rolls(input);
    let orig_paper_rolls_count = paper_rolls.len();

    loop {
        let rolls_to_remove = paper_rolls.iter()
            .filter(|(px, py)| has_fewer_than_4_neighbors(&paper_rolls, px, py))
            .cloned()
            .collect_vec();

        if rolls_to_remove.len() > 0 {
            for x in rolls_to_remove {
                paper_rolls.remove(&x);
            }
        } else {
            // No more paper rolls can be removed:
            break;
        }
    }

    AoCResult::Num((orig_paper_rolls_count - paper_rolls.len()) as u64)
}

fn get_paper_rolls(input: &String) -> HashSet<(i32, i32)> {
    let grid_width = input.find('\r').unwrap();

    input.chars()
        .filter(|c| c != &'\r' && c != &'\n')
        .enumerate()
        .filter(|(_, c)| c == &'@')
        .map(|(i, _)| ((i % grid_width) as i32, (i / grid_width) as i32) )
        .collect()
}

fn has_fewer_than_4_neighbors(grid: &HashSet<(i32, i32)>, px: &i32, py: &i32) -> bool {
    let mut neighbors = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0  {
                let nx = px + dx;
                let ny = py + dy;
                if grid.contains(&(nx, ny)) {
                    neighbors += 1;
                }
            }
        }
    }
    neighbors < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("..@@.@@@@.\r\n@@@.@.@.@@\r\n@@@@@.@.@@\r\n@.@@@@..@.\r\n@@.@@@@.@@\r\n.@@@@@@@.@\r\n.@.@.@.@@@\r\n@.@@@.@@@@\r\n.@@@@@@@@.\r\n@.@.@@@.@.");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 13);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("..@@.@@@@.\r\n@@@.@.@.@@\r\n@@@@@.@.@@\r\n@.@@@@..@.\r\n@@.@@@@.@@\r\n.@@@@@@@.@\r\n.@.@.@.@@@\r\n@.@@@.@@@@\r\n.@@@@@@@@.\r\n@.@.@@@.@.");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 43);
    }
}
