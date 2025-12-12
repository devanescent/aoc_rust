use aoc_core::AoCResult;
use itertools::Itertools;

aoc_core::make_day!(Day12);

pub fn solve_part1(input: &String) -> AoCResult {
    // Use following approximations specific to my input:
    // - Shape [0]: two of these can fit into a 4x4 square
    // - Shape [2] fits together with shape [4] into a 3x5 rectangle
    // - Shape [4] fits together with itself into a 3x5 rectangle
    // - Shape [3] and [5] fit together with each other or with themselves into a 3x4 rectangle

    // Skip shapes when parsing:
    let regions = input
        .trim_end()
        .lines()
        .skip_while(|l| !l.contains('x'))
        .map(|l| l.split_once(':').unwrap())
        .map(|(dims, presents)| {
            let dimensions = dims.split_once('x').unwrap();
            Region {
                size: dimensions.0.parse::<u64>().unwrap() * dimensions.1.parse::<u64>().unwrap(),
                presents: presents.split_whitespace().map(|p| p.parse().unwrap()).collect()
            }
        })
        .collect_vec();

    let mut regions_that_fit_presents = 0;

    for r in regions {
        let mut approx_area_required = 0;

        // Shape 0:
        approx_area_required += (r.presents[0] / 2) * (4*4);
        if r.presents[0] % 2 == 1 {
            approx_area_required += 9;
        }

        // Shape 1: no custom fitting, just add up 3x3 area per shape
        approx_area_required += r.presents[1] * 9;

        // Shape 2: fit each of these together with a shape 4:
        if r.presents[4] >= r.presents[2] {
            approx_area_required += r.presents[2] * (3*5);

            let rem_shape_4 = r.presents[4] - r.presents[2];
            approx_area_required += (rem_shape_4 / 2) * (3*5);
            if rem_shape_4 % 2 == 1 {
                approx_area_required += 9;
            }
        } else {
            // More shape 2 than shape 4:
            approx_area_required += r.presents[4] * (3*5);
            let rem_shape_2 = r.presents[2] - r.presents[4];
            approx_area_required += rem_shape_2 * 9;
        }

        // Shape 3/5:
        let shape_3_and_5 = r.presents[3] + r.presents[5];
        approx_area_required += (shape_3_and_5 / 2) * (3*4);
        if shape_3_and_5 % 2 == 1 {
            approx_area_required += 9;
        }

        // Fits region?
        if approx_area_required <= r.size {
            regions_that_fit_presents += 1;
        }
    }

    AoCResult::Num(regions_that_fit_presents)
}

pub fn solve_part2(_input: &String) -> AoCResult {
    AoCResult::NotImplemented
}

struct Region {
    size: u64,
    presents: Vec<u64>,
}