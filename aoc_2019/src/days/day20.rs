use aoc_core::AoCResult;
use crate::shared::geometry::Point;
use std::{collections::{HashMap, VecDeque}};
use itertools::Itertools;

aoc_core::make_day!(Day20);

pub fn solve_part1(input: &String) -> AoCResult {
    let input_width = input.lines().next().unwrap().len() + 2; // add \r\n back to the length of a line
    let portal_list = get_portal_list(&input, input_width);
    
    let mut portals_map = HashMap::<Point, Point>::new();
    for portal in portal_list.iter() {
        if let Some(other_portal) = portal_list.iter().find(|p| { p.name == portal.name && p.pos != portal.pos }) {
            portals_map.insert(portal.pos, other_portal.pos);
        }
    }

    let start = portal_list.iter().find(|p| p.name == "AA").unwrap().pos;
    let end = portal_list.iter().find(|p| p.name == "ZZ").unwrap().pos;

    let mut visited = HashMap::<Point, u64>::new();
    let mut bfs_queue = VecDeque::<(Point, u64)>::new();
    bfs_queue.push_back((start, 0));

    let nesw = vec![
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];

    let mut steps_to_zz = 0;

    while !bfs_queue.is_empty() {
        let (cur_pos, dist) = bfs_queue.pop_front().unwrap();
        let next_dist = dist + 1;

        // Check if the target location is reached:
        if cur_pos == end {
            steps_to_zz = dist;
            break;
        } else if let Some(portal) = portals_map.get(&cur_pos) {
            // Step through portal:
            if let Some(prev_dist) = visited.get_mut(portal) {
                if *prev_dist > next_dist {
                    // Current state is better: update
                    *prev_dist = next_dist;
                    bfs_queue.push_back((*portal, next_dist));
                }
            } else {
                // Position not reached before: insert
                visited.insert(*portal, next_dist);
                bfs_queue.push_back((*portal, next_dist));
            }
        }

        // Move regularly:
        for dir in nesw.iter() {
            let next_pos = cur_pos.add(dir);

            // Check if next pos is walkable:
            if let Some(next_tile) = input.chars().nth((next_pos.y as usize) * input_width + (next_pos.x as usize))
                && next_tile == '.' {
                if let Some(prev_dist) = visited.get_mut(&next_pos) {
                    if *prev_dist <= next_dist {
                        // Reached this position already with fewer steps
                        continue;
                    } else {
                        // Current state is better: update
                        *prev_dist = next_dist;
                    }
                } else {
                    // Position not reached before: insert
                    visited.insert(next_pos, next_dist);
                }

                bfs_queue.push_back((next_pos, next_dist));
            }
        }
    }

    AoCResult::Num(steps_to_zz)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let input_width = input.lines().next().unwrap().len() + 2; // add \r\n back to the length of a line
    let portal_list = get_portal_list(&input, input_width);
    
    let mut portals_map = HashMap::<Point, (PortalType, Point)>::new();
    for portal in portal_list.iter() {
        if let Some(other_portal) = portal_list.iter().find(|p| { p.name == portal.name && p.pos != portal.pos }) {
            portals_map.insert(portal.pos, (portal.portal_type, other_portal.pos));
        }
    }

    let start = portal_list.iter().find(|p| p.name == "AA").unwrap().pos;
    let end = portal_list.iter().find(|p| p.name == "ZZ").unwrap().pos;

    let mut visited = HashMap::<QueueState, u64>::new();
    let mut bfs_queue = VecDeque::<(QueueState, u64)>::new();
    bfs_queue.push_back((QueueState { pos: start, level: 0 }, 0));

    let nesw = vec![
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];

    let mut steps_to_zz = 0;

    while !bfs_queue.is_empty() {
        let (cur_state, dist) = bfs_queue.pop_front().unwrap();
        let next_dist = dist + 1;

        // Check if the target location is reached:
        if cur_state.level == 0 && cur_state.pos == end {
            steps_to_zz = dist;
            break;
        } else if let Some((portal_type, portal_output)) = portals_map.get(&cur_state.pos) && 
            (portal_type == &PortalType::Inside || cur_state.level > 0) {
            // Step through portal:
            // (outside portals other than AA and ZZ are not available on level 0)

            let next_state = QueueState { 
                pos: *portal_output, 
                level: if portal_type == &PortalType::Inside { cur_state.level + 1 } else { cur_state.level - 1 }
            };

            if let Some(prev_dist) = visited.get_mut(&next_state) {
                if *prev_dist > next_dist {
                    // Current state is better: update
                    *prev_dist = next_dist;
                    bfs_queue.push_back((next_state, next_dist));
                }
            } else {
                // Position not reached before: insert
                visited.insert(next_state.clone(), next_dist);
                bfs_queue.push_back((next_state, next_dist));
            }
        }

        // Move regularly:
        for dir in nesw.iter() {
            let next_pos = cur_state.pos.add(dir);

            // Check if next pos is walkable (places of start and end are also walls if level is not 0)
            if let Some(next_tile) = input.chars().nth((next_pos.y as usize) * input_width + (next_pos.x as usize))
                && next_tile == '.' && (next_pos != end || cur_state.level == 0) {

                let next_state = QueueState { 
                    pos: next_pos, 
                    level: cur_state.level
                };

                if let Some(prev_dist) = visited.get_mut(&next_state) {
                    if *prev_dist <= next_dist {
                        // Reached this position already with fewer steps
                        continue;
                    } else {
                        // Current state is better: update
                        *prev_dist = next_dist;
                    }
                } else {
                    // Position not reached before: insert
                    visited.insert(next_state.clone(), next_dist);
                }

                bfs_queue.push_back((next_state, next_dist));
            }
        }
    }

    AoCResult::Num(steps_to_zz)
}

#[derive(Clone, Copy, PartialEq)]
enum PortalType {
    Inside,
    Outside
}

struct Portal {
    name: String,
    pos: Point,
    portal_type: PortalType
}

fn get_portal_list(input: &String, input_width: usize) -> Vec<Portal> {
    input.chars()
        .enumerate()
        .map(|(i, c)| {
            match c {
                'A'..='Z' => {
                    // Check character above / Below / left / right:
                    let up = if i >= input_width { input.chars().nth(i - input_width) } else { None };
                    let down = input.chars().nth(i + input_width);
                    let left = input.chars().nth(i - 1);
                    let right = input.chars().nth(i + 1);

                    if let Some(up_char) = up && up_char == '.' {
                        // Portal above, name is this char and the char below
                        Some((i - input_width, c, down.unwrap()))
                    } else if let Some(down_char) = down && down_char == '.' {
                        // Portal below, name is the char above and this char
                        Some((i + input_width, up.unwrap(), c))
                    } else if let Some(left_char) = left && left_char == '.' {
                        // Portal to the left, name is this char and the char to the right
                        Some((i - 1, c, right.unwrap()))
                    } else if let Some(right_char) = right && right_char == '.' {
                        // Portal to the right, name is the char to the left and this char
                        Some((i + 1, left.unwrap(), c))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .flatten()  // filters out None values
        .map(|(i, c1, c2)| {
            let portal_pos = Point::new((i % input_width) as i64, (i / input_width) as i64);
            let portal_type = if portal_pos.y == 2 || portal_pos.y == (input.lines().count() - 3) as i64 || portal_pos.x == 2 || portal_pos.x == (input_width - 5) as i64 {
                PortalType::Outside
            } else {
                PortalType::Inside
            };

            Portal { 
                name: format!("{}{}", c1, c2),
                pos: portal_pos,
                portal_type: portal_type
            }
        })
        .collect_vec()
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct QueueState {
    pos: Point,
    level: u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("         A           \r\n         A           \r\n  #######.#########  \r\n  #######.........#  \r\n  #######.#######.#  \r\n  #######.#######.#  \r\n  #######.#######.#  \r\n  #####  B    ###.#  \r\nBC...##  C    ###.#  \r\n  ##.##       ###.#  \r\n  ##...DE  F  ###.#  \r\n  #####    G  ###.#  \r\n  #########.#####.#  \r\nDE..#######...###.#  \r\n  #.#########.###.#  \r\nFG..#########.....#  \r\n  ###########.#####  \r\n             Z       \r\n             Z       ");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 23);
    }

    #[test]
    fn part1_example2() {
        let input = String::from("                   A               \r\n                   A               \r\n  #################.#############  \r\n  #.#...#...................#.#.#  \r\n  #.#.#.###.###.###.#########.#.#  \r\n  #.#.#.......#...#.....#.#.#...#  \r\n  #.#########.###.#####.#.#.###.#  \r\n  #.............#.#.....#.......#  \r\n  ###.###########.###.#####.#.#.#  \r\n  #.....#        A   C    #.#.#.#  \r\n  #######        S   P    #####.#  \r\n  #.#...#                 #......VT\r\n  #.#.#.#                 #.#####  \r\n  #...#.#               YN....#.#  \r\n  #.###.#                 #####.#  \r\nDI....#.#                 #.....#  \r\n  #####.#                 #.###.#  \r\nZZ......#               QG....#..AS\r\n  ###.###                 #######  \r\nJO..#.#.#                 #.....#  \r\n  #.#.#.#                 ###.#.#  \r\n  #...#..DI             BU....#..LF\r\n  #####.#                 #.#####  \r\nYN......#               VT..#....QG\r\n  #.###.#                 #.###.#  \r\n  #.#...#                 #.....#  \r\n  ###.###    J L     J    #.#.###  \r\n  #.....#    O F     P    #.#...#  \r\n  #.###.#####.#.#####.#####.###.#  \r\n  #...#.#.#...#.....#.....#.#...#  \r\n  #.#####.###.###.#.#.#########.#  \r\n  #...#.#.....#...#.#.#.#.....#.#  \r\n  #.###.#####.###.###.#.#.#######  \r\n  #.#.........#...#.............#  \r\n  #########.###.###.#############  \r\n           B   J   C               \r\n           U   P   P               ");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 58);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("             Z L X W       C                 \r\n             Z P Q B       K                 \r\n  ###########.#.#.#.#######.###############  \r\n  #...#.......#.#.......#.#.......#.#.#...#  \r\n  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \r\n  #.#...#.#.#...#.#.#...#...#...#.#.......#  \r\n  #.###.#######.###.###.#.###.###.#.#######  \r\n  #...#.......#.#...#...#.............#...#  \r\n  #.#########.#######.#.#######.#######.###  \r\n  #...#.#    F       R I       Z    #.#.#.#  \r\n  #.###.#    D       E C       H    #.#.#.#  \r\n  #.#...#                           #...#.#  \r\n  #.###.#                           #.###.#  \r\n  #.#....OA                       WB..#.#..ZH\r\n  #.###.#                           #.#.#.#  \r\nCJ......#                           #.....#  \r\n  #######                           #######  \r\n  #.#....CK                         #......IC\r\n  #.###.#                           #.###.#  \r\n  #.....#                           #...#.#  \r\n  ###.###                           #.#.#.#  \r\nXF....#.#                         RF..#.#.#  \r\n  #####.#                           #######  \r\n  #......CJ                       NM..#...#  \r\n  ###.#.#                           #.###.#  \r\nRE....#.#                           #......RF\r\n  ###.###        X   X       L      #.#.#.#  \r\n  #.....#        F   Q       P      #.#.#.#  \r\n  ###.###########.###.#######.#########.###  \r\n  #.....#...#.....#.......#...#.....#.#...#  \r\n  #####.#.###.#######.#######.###.###.#.#.#  \r\n  #.......#.......#.#.#.#.#...#...#...#.#.#  \r\n  #####.###.#####.#.#.#.#.###.###.#.###.###  \r\n  #.......#.....#.#...#...............#...#  \r\n  #############.#.#.###.###################  \r\n               A O F   N                     \r\n               A A D   M                     ");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 396);
    }
}
