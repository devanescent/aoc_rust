use aoc_core::AoCResult;
use std::collections::HashMap;
use itertools::Itertools;

aoc_core::make_day!(Day08);

pub fn solve_part1(input: &String) -> AoCResult {
    AoCResult::Num(check_image_corruption(input, 25, 6))
}

fn get_layers(input: &String, image_width: usize, image_height: usize) -> Vec<String> {
    input
        .trim_end()
        .chars()
        .chunks(image_width * image_height)
        .into_iter()
        .map(|layer| layer.collect::<String>())
        .collect()
}

fn check_image_corruption(input: &String, image_width: usize, image_height: usize) -> u64 {
    // Split into layers:
    let layers = get_layers(input, image_width, image_height);

    // Count frequencies of digits of each layer:
    let freqs: Vec<_> = layers
        .iter()
        .cloned()
        .map(|layer| {
            layer.chars().fold(HashMap::new(), |mut map, c| {
                map.entry(c).and_modify(|frq| *frq += 1).or_insert(1);
                map
            })
        })
        .collect();

    // Find layer with maximum number of 0:
    let layer_max_0 = freqs
        .iter()
        .min_by(|layer1, layer2| {
            layer1
                .get(&'0')
                .unwrap_or(&0)
                .cmp(layer2.get(&'0').unwrap_or(&0))
        })
        .unwrap();

    layer_max_0.get(&'1').unwrap_or(&0) * layer_max_0.get(&'2').unwrap_or(&0)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let layers = get_layers(input, 25, 6);

    for y in 0..6 {
        let mut line = String::new();
        for x in 0..25 {
            for layer in layers.iter() {
                let c = layer.chars().nth(y * 25 + x).unwrap();
                if c != '2' {
                    line.push(if c == '1' { '#' } else { '.' });
                    break;
                }
            }
        }
        println!("{}", line);
    }

    AoCResult::PrintedToConsole
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("120256780012");
        let result = check_image_corruption(&input, 3, 2);
        assert_eq!(result, 2);
    }
}
