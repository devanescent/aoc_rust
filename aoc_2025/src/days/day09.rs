use std::{cmp::{Reverse, max, min}};

use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day09);

pub fn solve_part1(input: &String) -> AoCResult {
    let max_area = input
        .trim_end()
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(a,b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .tuple_combinations()
        .map(|(t1, t2)| ((t1.0 - t2.0).abs() + 1) * ((t1.1 - t2.1).abs() + 1))
        .max()
        .unwrap();

    AoCResult::Num(max_area as u64)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let red_tiles = input
        .trim_end()
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(a,b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect_vec();

    let mut outer_edges: Vec<((i64, i64), (i64, i64))> = red_tiles.iter()
        .cloned()
        .tuple_windows()
        .collect_vec();

    // Add edge between last and first point:
    outer_edges.push((red_tiles[0], red_tiles.last().unwrap().clone()));

    // Order edges so coordinates of the first point are smaller than coordinates of the second point
    // (which will make later comparisons easier!)
    let outer_edges = outer_edges.into_iter()
        .map(|((ax, ay), (bx,by))| {
            if ax == bx {
                // Vertical edge:
                ((ax, min(ay, by)), (bx, max(ay, by)))
            } else {
                // Horizontal edge:
                ((min(ax, bx), ay), (max(ax, bx), by))
            }
        })
        .collect_vec();

    // Checks possible rectangles ordered by size so the first valid rectangle will be the largest.
    // A rectangle is assumed to be valid, if no other edges cross it (this is not sufficient in 
    // the general case, but works for the input!)
    let mut largest_area = 0;
    for (p1, p2) in input
        .trim_end()
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(a,b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .tuple_combinations()
        .sorted_by_cached_key(|(t1, t2)| {
            let area = ((t1.0 - t2.0).abs() + 1) * ((t1.1 - t2.1).abs() + 1);
            Reverse(area)
        }) {
            if (p1 == (9,5) && p2 == (2,3)) || (p2 == (9,5) && p1 == (2,3)) {
                println!("Here!");
            }
            // Top left corner of the current rectangle:
            let x0 = min(p1.0, p2.0);
            let y0 = min(p1.1, p2.1);

            // Bottom right corner of the current rectangle:
            let x1 = max(p1.0, p2.0);
            let y1 = max(p1.1, p2.1);

            // Check that no edge from the outer polygon crosses through the rectangle:
            if outer_edges.iter().any(|((ax, ay), (bx, by))| {
                if ax == bx {
                    // Vertical edge:
                    x0 < *ax && *ax < x1 && *ay < y1 && *by > y0
                } else {
                    // Horizontal edge:
                    y0 < *ay && *ay < y1 && *ax < x1 && *bx > x0
                }
            }) {
                continue;
            }

            largest_area = (x1 - x0 + 1) * (y1 - y0 + 1);
            break;
        }
    
    AoCResult::Num(largest_area as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("7,1\r\n11,1\r\n11,7\r\n9,7\r\n9,5\r\n2,5\r\n2,3\r\n7,3");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 50);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("7,1\r\n11,1\r\n11,7\r\n9,7\r\n9,5\r\n2,5\r\n2,3\r\n7,3");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 24);
    }
}
