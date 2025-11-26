use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day16);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut data = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect_vec();

    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];

    // Calculate 100 phases:
    for _ in 0..100 {
        data = (0..data.len())
            .map(|out_index| {
                // Calculate next value for each output index:
                let mut next_value = 0i32;

                // Skip first element in pattern:
                let mut skip_first = true;

                // Process all data:
                let mut data_iter = data.iter();
                'data_loop: loop {
                    for p in base_pattern.iter() {
                        // Repeat each value of the pattern depending on the output index:
                        for _ in 0..(out_index + 1) {
                            if skip_first {
                                skip_first = false;
                                continue;
                            }

                            if let Some(x) = data_iter.next() {
                                next_value += (*x as i32) * p;
                            } else {
                                break 'data_loop;
                            }
                        }
                    }
                }

                // Only last digit is kept:
                (next_value.abs() % 10) as u8
            })
            .collect();
    }

    // Get first 8 digits:
    let res = data
        .iter()
        .take(8)
        .map(|d| char::from_digit((*d) as u32, 10).unwrap())
        .collect::<String>();
    AoCResult::Str(res)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let mut data = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect_vec();

    // Copy data 9999 times
    let orig_len = data.len();
    for _ in 0..9999 {
        data.extend_from_within(0..orig_len);
    }

    // Because the pattern at each position x starts with (x-1) zeroes, earlier numbers do not influence later values:
    let offset: usize = data
        .iter()
        .take(7)
        .map(|d| char::from_digit((*d) as u32, 10).unwrap())
        .collect::<String>()
        .parse()
        .unwrap();

    // Drop everything before the offset:
    data = data.into_iter().skip(offset).collect();

    // Assuming the offset is much larger than the number of remaining elements, the rest of the elements
    // will all receive a '1' multiplier (so they are just summed up), which allows to simplify calculations:
    for _ in 0..100 {
        // Sum all remaining elements once:
        let mut total_sum: u64 = data.iter().fold(0u64, |acc, x| acc + (*x as u64));

        data = data
            .into_iter()
            .map(|d| {
                let next_value = total_sum;

                // Next element will be the same sum minus the current element:
                total_sum -= d as u64;

                // Only last digit is kept:
                (next_value % 10) as u8
            })
            .collect();
    }

    // Get first 8 digits:
    let res = data
        .iter()
        .take(8)
        .map(|d| char::from_digit((*d) as u32, 10).unwrap())
        .collect::<String>();
    AoCResult::Str(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("80871224585914546619083218645595");
        let result = solve_part1(&input);
        assert_eq!(String::from(result), "24176176");
    }

    #[test]
    fn part1_example2() {
        let input = String::from("19617804207202209144916044189917");
        let result = solve_part1(&input);
        assert_eq!(String::from(result), "73745418");
    }

    #[test]
    fn part1_example3() {
        let input = String::from("69317163492948606335995924319873");
        let result = solve_part1(&input);
        assert_eq!(String::from(result), "52432133");
    }

    #[test]
    fn part2_example1() {
        let input = String::from("03036732577212944063491565474664");
        let result = solve_part2(&input);
        assert_eq!(String::from(result), "84462026");
    }

    #[test]
    fn part2_example2() {
        let input = String::from("02935109699940807407585447034323");
        let result = solve_part2(&input);
        assert_eq!(String::from(result), "78725270");
    }

    #[test]
    fn part2_example3() {
        let input = String::from("03081770884921959731165446850517");
        let result = solve_part2(&input);
        assert_eq!(String::from(result), "53553731");
    }
}
