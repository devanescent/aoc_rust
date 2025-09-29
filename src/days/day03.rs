use itertools::Itertools;
use crate::aoc_result::AoCResult;
use std::str::FromStr;

pub fn solve_part1(input: &String) -> AoCResult {
   	let (wire1, wire2) = input.lines().next_tuple().unwrap();

	let wire1_parts = parse_wire_segments(wire1);
	let wire2_parts = parse_wire_segments(wire2);

	let mut intersections = Vec::<Point>::new();
	for (w1_start, w1_end) in wire1_parts.iter() {
		for (w2_start, w2_end) in wire2_parts.iter() {
			if w1_start.x >= w2_start.x && w1_end.x <= w2_end.x &&
			   w1_start.y <= w2_start.y && w1_end.y >= w2_end.y {
					//   ^
					//   |
					// --+--> w2
					//   |
					//   w1    
					intersections.push(Point{x: w1_start.x, y: w2_end.y });
			} else if w1_start.x <= w2_start.x && w1_end.x >= w2_end.x && 
			          w1_start.y >= w2_start.y && w1_end.y <= w2_end.y {
					//   ^
					//   |
					// --+--> w1
					//   |
					//   w2    
					intersections.push(Point{x: w2_start.x, y: w1_end.y });
			}
		}
	}

	// Find intersection point that's closest to origin
	let closest = intersections.iter()
		.filter(|p| p.x != 0 && p.y != 0)
		.map(|p|  p.x.abs() + p.y.abs())
		.min()
		.unwrap();

	AoCResult::Num(u64::try_from(closest).unwrap())
}

// pub fn solve_part2(input: &String) -> AoCResult {
	
// }

#[derive(Copy, Clone, Debug)]
struct Point {
	x: i64,
	y: i64
}

fn parse_wire_segments(wire: &str) -> Vec<(Point, Point)> {
	let mut p = Point{x: 0, y: 0};
	wire
		.split(',')
		.map(|s| (s.chars().nth(0).unwrap(), i64::from_str(&s[1..]).unwrap()))
		.map(|w| {
			match w.0 {
				'U' => {
					let next = Point{x: p.x, y: p.y + w.1};
					let res = (p, next);
					p = next;
					return res;
				},
				'R' => {
					let next = Point{x: p.x + w.1, y: p.y};
					let res = (p, next);
					p = next;
					return res;
				},
				'D' => {
					let next = Point{x: p.x, y: p.y - w.1};
					let res = (next, p);
					p = next;
					return res;
				},
				'L' => {
					let next = Point{x: p.x - w.1, y: p.y};
					let res = (next, p);
					p = next;
					return res;
				},
				_ =>  (p,p),
			}
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
}