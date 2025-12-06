use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day06);

pub fn solve_part1(input: &String) -> AoCResult {
    let rows = input
        .trim()
        .lines()
        .map(|l| l.split_whitespace().collect_vec())
        .collect_vec();

    let mut grand_total = 0;
    for i in 0..rows[0].len() {
        // Get operator from last line:
        let op = rows.last().unwrap()[i];
        
        // Add or multiply numbers together:
        let mut problem_result = if op == "+" { 0 } else { 1 };
        for row in rows.iter().take(rows.len() - 1) {
            if op == "+" {
                problem_result += row[i].parse::<u64>().unwrap();
            } else if op == "*" {
                problem_result *= row[i].parse::<u64>().unwrap();
            }
        }

        grand_total += problem_result;
    }

    AoCResult::Num(grand_total)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let rows = input
        .trim()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut grand_total = 0;
    let mut current_op = ' ';
    let mut current_result = 0;
    for i in 0..rows[0].len() {
        let op = rows.last().unwrap().get(i).unwrap_or(&' ');
        if op != &' ' {
            current_op = *op;
            current_result = if op == &'+' { 0 } else { 1 };
        }

        let num: u64 = rows.iter()
            .take(rows.len() - 1)
            .filter(|row| row[i] != ' ')
            .rev()
            .enumerate()
            .map(|(j, row)| 10u64.pow(j as u32) * (row[i].to_digit(10).unwrap() as u64))
            .sum();

        if num != 0 {
            if current_op == '+' {
                current_result += num;
            } else if current_op == '*' {
                current_result *= num;
            }
        } else {
            // Separation between columns:
            grand_total += current_result;
        }
    }
    
    // Result from last column:
    grand_total += current_result;

    AoCResult::Num(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("123 328  51 64 \r\n 45 64  387 23 \r\n  6 98  215 314\r\n*   +   *   +  ");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 4277556);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("123 328  51 64 \r\n 45 64  387 23 \r\n  6 98  215 314\r\n*   +   *   +  ");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 3263827);
    }
}
