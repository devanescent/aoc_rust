use std::collections::HashMap;
use std::f64::consts::PI;

use itertools::Itertools;

use crate::aoc_result::AoCResult;
use crate::shared::geometry::Point;

make_day!(Day10);

pub fn solve_part1(input: &String) -> AoCResult {
    let asteroids = get_asteroids(input);
    let all_connections = get_all_connections(&asteroids);

    AoCResult::Num(*all_connections.iter().map(|(_, d)| { d }).max().unwrap())
}

pub fn solve_part2(input: &String) -> AoCResult {
    let asteroids = get_asteroids(input);
    let all_connections = get_all_connections(&asteroids);
    
    // Point with maximum connections:
    let p_max_conn = all_connections.iter().max_by(|(_, con1), (_, con2)| { con1.cmp(con2)}).unwrap().0;

    // Returns a list of "angles" and distances at which the other asteroids are
    let rotation_list = asteroids
        .iter()
        .filter(|p| p != &&p_max_conn)
        .map(|a| {
            let v = p_max_conn.distance_to(a);
            let dist = p_max_conn.steps_to(a);

            if v.y < 0 && v.x >= 0 {
                // top-right quadrant
                (*a, (v.x as f64/-v.y as f64).atan(), 1, dist)

            } else if v.y >= 0 && v.x > 0 {
                // bottom-right quadrant
                (*a, (v.y as f64/v.x as f64).atan(), 2, dist)

            } else if v.y > 0 && v.x <= 0 {
                // bottom-left quadrant
                (*a, (-v.x as f64/v.y as f64).atan(), 3, dist)

            } else if v.y <= 0 && v.x < 0 {
                // top-left quadrant
                (*a, (-v.y as f64/-v.x as f64).atan(), 4, dist)

            } else {
                unreachable!()
            }
        })
        .map(|(p, angle,quad,d)| {
            let normalized_angle = angle * 2f64 / PI; // normalized = [0,1)
            let full_angle = normalized_angle + f64::from(quad); // add quad number
            let x = (full_angle * 1000f64) as u64; // convert to integer value for easier handling

            // Concrete value for x is not relevant, but it will used to group elements
            // that are at the same angle from the monitoring station
            (p, x, d)
            
        })
        // Sort targets at same angle by distance:
        .sorted_by_key(|(_, _, d)| *d)
        .collect::<Vec<_>>();

    // Group asteroids by angle:
    let mut asteroid_by_angle = HashMap::<u64, Vec<Point>>::new();
    for (p, angle, _) in rotation_list.iter() {
        asteroid_by_angle.entry(*angle)
            .and_modify(|x| x.push(*p) )
            .or_insert(vec![*p]);
    }

    // Loop over the asteroid groups (simulating one circle of the laser) until all targets are destroyed:
    let mut order_of_destruction = Vec::<Point>::new();
    while order_of_destruction.len () < asteroids.len() - 1 {
        for (_, asteroids) in asteroid_by_angle.iter_mut().sorted_by_key(|(angle, _)| *angle) {
            if asteroids.len() > 0 {
                order_of_destruction.push(asteroids[0]);
                asteroids.remove(0);
            }
        }
    }

    let asteroid_200th = order_of_destruction[199];
    AoCResult::Num(u64::try_from(asteroid_200th.x * 100 + asteroid_200th.y).unwrap())
}

fn get_asteroids(input: &String) -> Vec::<Point> {
    input
        .trim_end()
        .lines()
        .enumerate()
        // Rows:
        .map(|(y, row)| {
            row
            .chars()
            .enumerate()
            // Cols:
            .map(move|(x, c)| { (c, Point::from((x,y))) })
            .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|(c, _)| c == &'#')
        .map(|(_,p)| p)
        .collect()
}

// Reduces the point like a fraction:
fn reduce(v: &Point) -> Point {
    let min_val = std::cmp::min(v.x.abs(), v.y.abs());
    if min_val == 0 {
        if v.x == 0 {
            return Point { x: 0, y: v.y / v.y.abs() };
        } else {
            return Point { x: v.x / v.x.abs(), y: 0 };
        }
    } else {
        for i in (1..=min_val).rev() {
            if (v.x % i == 0) && (v.y % i == 0) {
                return Point { x: v.x / i, y: v.y / i }
            }
        }
    }

    return *v;
}

fn get_all_connections(asteroids: &Vec::<Point>) -> Vec::<(Point, u64)> {
    let mut all_connections = Vec::<(Point, u64)>::new();
    for a1 in asteroids.iter() {
        let mut connections = 0u64;
        'inner: for a2 in asteroids.iter() {
            if a1 != a2 {
                // Vector between two asteroids:
                let actual_dist = a1.distance_to(a2);
                let dist_step = reduce(&actual_dist);

                // Check if line-of-sight is blocked by any other telephone:
                let mut d = dist_step;

                while d != actual_dist {

                    if asteroids.iter().find(|p| **p == a1.add(&d)).is_some() {
                        // Path is blocked
                        continue 'inner;
                    }

                    d = d.add(&dist_step);
                }
                connections += 1;
            }
        }

        all_connections.push((a1.clone(), connections));
    }
    all_connections

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from(".#..#\r\n.....\r\n#####\r\n....#\r\n...##");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 8);
    }

    #[test]
    fn part1_example2() {
        let input = String::from("......#.#.\r\n#..#.#....\r\n..#######.\r\n.#.#.###..\r\n.#..#.....\r\n..#....#.#\r\n#..#....#.\r\n.##.#..###\r\n##...#..#.\r\n.#....####");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 33);
    }

    #[test]
    fn part1_example3() {
        let input = String::from("#.#...#.#.\r\n.###....#.\r\n.#....#...\r\n##.#.#.#.#\r\n....#.#.#.\r\n.##..###.#\r\n..#...##..\r\n..##....##\r\n......#...\r\n.####.###.");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 35);
    }

    #[test]
    fn part1_example4() {
        let input = String::from(".#..#..###\r\n####.###.#\r\n....###.#.\r\n..###.##.#\r\n##.##.#.#.\r\n....###..#\r\n..#.#..#.#\r\n#..#.#.###\r\n.##...##.#\r\n.....#.#..");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 41);
    }

    #[test]
    fn part1_example5() {
        let input = String::from(".#..##.###...#######\r\n##.############..##.\r\n.#.######.########.#\r\n.###.#######.####.#.\r\n#####.##.#.##.###.##\r\n..#####..#.#########\r\n####################\r\n#.####....###.#.#.##\r\n##.#################\r\n#####.##.###..####..\r\n..######..##.#######\r\n####.##.####...##..#\r\n.#####..#.######.###\r\n##...#.##########...\r\n#.##########.#######\r\n.####.#.###.###.#.##\r\n....##.##.###..#####\r\n.#.#.###########.###\r\n#.#.#.#####.####.###\r\n###.##.####.##.#..##");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 210);
    }

    #[test]
    fn part2_example1() {
        let input = String::from(".#..##.###...#######\r\n##.############..##.\r\n.#.######.########.#\r\n.###.#######.####.#.\r\n#####.##.#.##.###.##\r\n..#####..#.#########\r\n####################\r\n#.####....###.#.#.##\r\n##.#################\r\n#####.##.###..####..\r\n..######..##.#######\r\n####.##.####...##..#\r\n.#####..#.######.###\r\n##...#.##########...\r\n#.##########.#######\r\n.####.#.###.###.#.##\r\n....##.##.###..#####\r\n.#.#.###########.###\r\n#.#.#.#####.####.###\r\n###.##.####.##.#..##");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 802);
    }
}
