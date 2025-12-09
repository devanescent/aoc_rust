use std::{cmp::min, collections::{HashMap, HashSet, VecDeque}};

use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day08);

pub fn solve_part1(input: &String) -> AoCResult {
    let result = combine_shortest_conns(input, 1000);
    AoCResult::Num(result)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let result = combine_all_conns(input);
    AoCResult::Num(result)
}

fn combine_shortest_conns(input: &String, n: usize) -> u64 {
    let boxes: Vec<(i64,i64,i64)> = input
        .trim_end()
        .lines()
        .map(|l| l.split(",").map(|x| x.parse::<i64>().unwrap()).collect_tuple().unwrap())
        .collect_vec();

    // At the beginning, each box is a circuit by itself:
    let mut circuits = boxes.iter()
        .enumerate()
        .map(|(i, x)| (*x,i))
        .collect::<HashMap<(i64,i64,i64), usize>>();

    // Connect together the 1000 pairs of junction boxes which are closest together
    for (box1, box2) in boxes.iter()
        .tuple_combinations()
        .sorted_by_cached_key(|(a,b)| (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2) )
        .take(n) {
            // Take lower circuit number between the two boxes to combine them:
            let circuit1 = circuits[box1];
            let circuit2 = circuits[box2];
            let res_circuit = min(circuit1, circuit2);

            for (_, c) in circuits.iter_mut() {
                if c == &circuit1 || c == &circuit2 {
                    *c = res_circuit;
                }
            }
    }

    // Get circuit sizes:
    let freqs = circuits.iter()
        .counts_by(|(_, c)| *c);

    // Find three largest circuits:
    let top3 = freqs
        .iter().sorted_by_key(|(_, n)| *n)
        .rev()
        .map(|(_, n)| *n)
        .take(3);

    // Multiply together the sizes of the three largest circuits:
    top3.fold(1, |acc, x| acc * x) as u64
}

fn combine_all_conns(input: &String) -> u64 {
    let boxes: Vec<(i64,i64,i64)> = input
        .trim_end()
        .lines()
        .map(|l| l.split(",").map(|x| x.parse::<i64>().unwrap()).collect_tuple().unwrap())
        .collect_vec();

    // At the beginning, each box is a circuit by itself:
    let mut circuits = boxes.iter()
        .enumerate()
        .map(|(i, x)| (*x,i))
        .collect::<HashMap<(i64,i64,i64), usize>>();

    let mut result: u64 = 0;

    // Connect together all pairs of junction boxes until they are all in one circuit:
    for (box1, box2) in boxes.iter()
        .tuple_combinations()
        .sorted_by_cached_key(|(a,b)| (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2) ) {
            // Take lower circuit number between the two boxes to combine them:
            let circuit1 = circuits[box1];
            let circuit2 = circuits[box2];
            let res_circuit = min(circuit1, circuit2);

            for (_, c) in circuits.iter_mut() {
                if c == &circuit1 || c == &circuit2 {
                    *c = res_circuit;
                }
            }

            // Check if all boxes are connected:
            if circuits.iter().all(|(_, c)| c == &res_circuit) {
                result = (box1.0 * box2.0) as u64;
                break;
            }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("162,817,812\r\n57,618,57\r\n906,360,560\r\n592,479,940\r\n352,342,300\r\n466,668,158\r\n542,29,236\r\n431,825,988\r\n739,650,466\r\n52,470,668\r\n216,146,977\r\n819,987,18\r\n117,168,530\r\n805,96,715\r\n346,949,466\r\n970,615,88\r\n941,993,340\r\n862,61,35\r\n984,92,344\r\n425,690,689");
        let result = combine_shortest_conns(&input, 10);
        assert_eq!(u64::from(result), 40);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("162,817,812\r\n57,618,57\r\n906,360,560\r\n592,479,940\r\n352,342,300\r\n466,668,158\r\n542,29,236\r\n431,825,988\r\n739,650,466\r\n52,470,668\r\n216,146,977\r\n819,987,18\r\n117,168,530\r\n805,96,715\r\n346,949,466\r\n970,615,88\r\n941,993,340\r\n862,61,35\r\n984,92,344\r\n425,690,689");
        let result = combine_all_conns(&input);
        assert_eq!(u64::from(result), 25272);
    }
}
