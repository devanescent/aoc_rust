use crate::aoc_result::AoCResult;
use itertools::Itertools;
use std::str::FromStr;
use crate::shared::geometry::Point;

make_day!(Day03);

pub fn solve_part1(input: &String) -> AoCResult {
    let (wire1, wire2) = input.lines().next_tuple().unwrap();

    let wire1_parts = parse_wire_segments(wire1);
    let wire2_parts = parse_wire_segments(wire2);

    let mut intersections = Vec::<Point>::new();
    for w1 in wire1_parts.iter() {
        for w2 in wire2_parts.iter() {
            if let Some(intersect_point) = w1.intersect(w2) {
                intersections.push(intersect_point);
            }
        }
    }

    // Find intersection point that's closest to origin
    let closest = intersections
        .iter()
        .filter(|p| p.x != 0 && p.y != 0)
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .unwrap();

    AoCResult::Num(u64::try_from(closest).unwrap())
}

pub fn solve_part2(input: &String) -> AoCResult {
    let (wire1, wire2) = input.lines().next_tuple().unwrap();

    let wire1_parts = parse_wire_segments(wire1);
    let wire2_parts = parse_wire_segments(wire2);

    let mut intersections = Vec::<(Point, u64)>::new();

    let mut w1_distance = 0u64;
    for w1 in wire1_parts.iter() {
        let mut w2_distance = 0u64;
        for w2 in wire2_parts.iter() {
            if let Some(intersect_point) = w1.intersect(w2) {
                let total_dist = w1_distance
                    + w1.start.steps_to(&intersect_point)
                    + w2_distance
                    + w2.start.steps_to(&intersect_point);
                intersections.push((intersect_point, total_dist));
            }

            w2_distance += w2.len();
        }

        w1_distance += w1.len();
    }

    // Find intersection point that's closest to origin
    let smallest_dist = intersections
        .iter()
        .filter(|(p, _)| p.x != 0 && p.y != 0)
        .map(|(_, dist)| *dist)
        .min()
        .unwrap();

    AoCResult::Num(u64::try_from(smallest_dist).unwrap())
}

#[derive(Debug)]
struct WireSegment {
    start: Point,
    end: Point,
}

impl WireSegment {
    fn len(&self) -> u64 {
        self.start.steps_to(&self.end)
    }

    fn intersect(&self, other: &WireSegment) -> Option<Point> {
        let mut s1 = self.start;
        let mut e1 = self.end;
        if s1.x > e1.x || s1.y > e1.y {
            std::mem::swap(&mut e1, &mut s1);
        }

        let mut s2 = other.start;
        let mut e2 = other.end;
        if s2.x > e2.x || s2.y > e2.y {
            std::mem::swap(&mut e2, &mut s2);
        }

		if s1.x >= s2.x && e1.x <= e2.x &&
		   s1.y <= s2.y && e1.y >= e2.y {
            //   ^
            //   |
            // --+--> w2
            //   |
            //   w1
            Some(Point { x: s1.x, y: e2.y })
        } else if s1.x <= s2.x && e1.x >= e2.x &&
                  s1.y >= s2.y && e1.y <= e2.y {
            //   ^
            //   |
            // --+--> w1
            //   |
            //   w2
            Some(Point { x: s2.x, y: e1.y })
        } else {
            None
        }
    }
}

fn parse_wire_segments(wire: &str) -> Vec<WireSegment> {
    let mut p = Point { x: 0, y: 0 };
    wire.split(',')
        .map(|s| (s.chars().nth(0).unwrap(), i64::from_str(&s[1..]).unwrap()))
        .map(|w| {
            let next_point = match w.0 {
                'U' => Point {
                    x: p.x,
                    y: p.y + w.1,
                },
                'R' => Point {
                    x: p.x + w.1,
                    y: p.y,
                },
                'D' => Point {
                    x: p.x,
                    y: p.y - w.1,
                },
                'L' => Point {
                    x: p.x - w.1,
                    y: p.y,
                },
                _ => p,
            };

            let seg = WireSegment {
                start: p,
                end: next_point,
            };
            p = next_point;
            return seg;
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("R8,U5,L5,D3\r\nU7,R6,D4,L4");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 6);
    }

    #[test]
    fn part1_example2() {
		let input = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\r\nU62,R66,U55,R34,D71,R55,D58,R83");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 159);
    }

    #[test]
    fn part1_example3() {
		let input = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\r\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 135);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("R8,U5,L5,D3\r\nU7,R6,D4,L4");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 30);
    }

    #[test]
    fn part2_example2() {
		let input = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\r\nU62,R66,U55,R34,D71,R55,D58,R83");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 610);
    }

    #[test]
    fn part2_example3() {
		let input = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\r\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 410);
    }
}
