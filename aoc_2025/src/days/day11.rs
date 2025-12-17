use aoc_core::AoCResult;
use std::collections::HashMap;
use itertools::Itertools;

aoc_core::make_day!(Day11);

pub fn solve_part1(input: &String) -> AoCResult {
    let connections = get_connections(&input);
    let paths = count_paths(&connections, "you", "out");
    AoCResult::Num(paths)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let connections = get_connections(&input);

    let paths_svr_dac = count_paths(&connections, "svr", "dac");
    let paths_svr_fft = count_paths(&connections, "svr", "fft");
    let paths_dac_fft = count_paths(&connections, "dac", "fft"); 
    let paths_fft_dac = count_paths(&connections, "fft", "dac");
    let paths_fft_out = count_paths(&connections, "fft", "out");
    let paths_dac_out = count_paths(&connections, "dac", "out");

    let paths_total = paths_svr_dac * paths_dac_fft * paths_fft_out +
                           paths_svr_fft * paths_fft_dac * paths_dac_out;

    AoCResult::Num(paths_total)
}

fn get_connections(input: &String) -> HashMap<&str, Vec<&str>> {
    input
        .trim_end()
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(from, to)| (from, to.split_whitespace().collect_vec()))
        .collect()
}

// Count paths between two nodes, keeping track of already found paths:
fn count_paths(connections: &HashMap<&str, Vec<&str>>, from: &str, to: &str) -> u64 {  
    let mut cache = HashMap::<String, u64>::new();
    count_paths_rec(connections, from, to, &mut cache)
}

fn count_paths_rec(connections: &HashMap<&str, Vec<&str>>, from: &str, to: &str, cache: &mut HashMap<String, u64>) -> u64 {  
    if let Some(exists) = cache.get(from) {
        return *exists;
    }

    let mut paths = 0;
    if let Some(outputs) = connections.get(from) {
        for output in outputs.iter() {
            if output == &to {
                return 1;
            }

            paths += count_paths_rec(connections, &output, to, cache);
        }
    }

    cache.insert(from.to_owned(), paths);
    paths
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("aaa: you hhh\r\nyou: bbb ccc\r\nbbb: ddd eee\r\nccc: ddd eee fff\r\nddd: ggg\r\neee: out\r\nfff: out\r\nggg: out\r\nhhh: ccc fff iii\r\niii: out");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 5);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("svr: aaa bbb\r\naaa: fft\r\nfft: ccc\r\nbbb: tty\r\ntty: ccc\r\nccc: ddd eee\r\nddd: hub\r\nhub: fff\r\neee: dac\r\ndac: fff\r\nfff: ggg hhh\r\nggg: out\r\nhhh: out");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 2);
    }
}
