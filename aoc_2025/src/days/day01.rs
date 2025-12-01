use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day01);

pub fn solve_part1(input: &String) -> AoCResult {
    let rotations = get_rotations(input);

    let mut dial = 50;
    let mut zero_passes = 0;

    for rot in rotations {
        dial = (dial + rot) % 100;
        // Count number of times the dial points to zero after rotation:
        if dial == 0 {
            zero_passes += 1;
        }
    }

    AoCResult::Num(zero_passes)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let rotations = get_rotations(input);

    let mut dial = 50;
    let mut zero_passes = 0;

    for rot in rotations {
        // Count number of times the dial passes through zero during rotation:
        let full_rotations = (rot / 100).abs() as u64;
        zero_passes += full_rotations;
        
        // Remaining rotatation after performing all full rotations:
        let rot_remainder = rot % 100;

        if dial == 0 {
            // If dial already started on zero, do not count as passing through zero again:
            dial = (100 + rot_remainder) % 100;
        } else {
            // Perform remaining rotation, check for over- / underflow:
            dial += rot_remainder;

            if dial <= 0 || dial >= 100 {
                zero_passes += 1;
                
                // Adjust dial value:
                if dial < 0 {
                    dial += 100;
                } else if dial >= 100 {
                    dial -= 100;
                }
            }
        }
    }

    AoCResult::Num(zero_passes)
}

fn get_rotations(input: &String) -> Vec<i64> {
    input
        .trim_end()
        .lines()
        .map(|l| {
            match l.chars().nth(0).unwrap() {
                'L' => l.strip_prefix("L").unwrap().parse::<i64>().unwrap() * -1,
                'R' => l.strip_prefix("R").unwrap().parse().unwrap(),
                _ => 0i64
            }
        })
        .collect_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("L68\r\nL30\r\nR48\r\nL5\r\nR60\r\nL55\r\nL1\r\nL99\r\nR14\r\nL82");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 3);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("L68\r\nL30\r\nR48\r\nL5\r\nR60\r\nL55\r\nL1\r\nL99\r\nR14\r\nL82");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 6);
    }
}
