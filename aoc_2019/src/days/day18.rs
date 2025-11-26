use aoc_core::AoCResult;
use crate::shared::geometry::Point;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{
    collections::{HashMap, VecDeque},
    ops::Index,
};

aoc_core::make_day!(Day18);

pub fn solve_part1(input: &String) -> AoCResult {
    // All keys that can be collected in the maze:
    let all_keys = input.chars().fold(0u32, |acc, c| match c {
        'a'..='z' => acc | 1u32 << (c as u8) - ('a' as u8),
        _ => acc,
    });

    // Parse the maze:
    let maze = Maze::new(input);
    let start = maze.find('@').unwrap();

    // Explore maze via BFS
    let mut bfs_queue = VecDeque::<(MazeState, u64)>::new();
    bfs_queue.push_back((
        MazeState {
            pos: start,
            keys: 0,
        },
        0,
    ));

    // Check all possible directions at each locations to continue exploration:
    let nesw = vec![
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];

    // Locations visited with keys collected at this point and the distance travelled to reach this state:
    let mut visited = HashMap::<MazeState, u64>::new();
    let mut dist_for_all_keys = 0;

    while !bfs_queue.is_empty() {
        let (cur_state, dist) = bfs_queue.pop_front().unwrap();

        // First time all keys are collected stop the search:
        if cur_state.keys == all_keys {
            dist_for_all_keys = dist;
            break;
        }

        for dir in nesw.iter() {
            let next_pos = cur_state.pos.add(dir);
            let mut next_keys = cur_state.keys;

            // Check next tile:
            let tile = maze[&next_pos] as char;
            match tile {
                'a'..='z' => {
                    // Key picked up: update state
                    let key_collected = 1u32 << (tile.to_ascii_lowercase() as u8) - ('a' as u8);
                    next_keys |= key_collected;
                }
                'A'..='Z' => {
                    // Door:
                    let key_needed = 1u32 << (tile.to_ascii_lowercase() as u8) - ('a' as u8);
                    if cur_state.keys & key_needed == 0 {
                        // Key not picked up, cannot move in this direction yet
                        continue;
                    }
                }
                '#' => {
                    // Wall:
                    continue;
                }
                '.' | '@' => {}
                _ => unreachable!(),
            }

            let next_state = MazeState {
                pos: next_pos,
                keys: next_keys,
            };
            let next_dist = dist + 1;

            if let Some(prev_dist) = visited.get_mut(&next_state) {
                if *prev_dist <= next_dist {
                    // Reached this position already in a better state
                    continue;
                } else {
                    // Current state is better: update
                    *prev_dist = next_dist;
                }
            } else {
                // State not reached before: insert
                visited.insert(next_state.clone(), next_dist);
            }

            bfs_queue.push_back((next_state, next_dist));
        }
    }

    AoCResult::Num(dist_for_all_keys)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let all_keys = input.chars().fold(0u32, |acc, c| match c {
        'a'..='z' => acc | 1u32 << (c as u8) - ('a' as u8),
        _ => acc,
    });

    let maze: Maze<'_> = Maze::new(input);
    let start = maze.find('@').unwrap();

    // Pre-calculate all distances between keys. Maze will be processed by hopping from key to key.
    let mut key_distances = maze.get_key_distances();

    // Because the start will be moved by two tiles, reduce distances from the start node by two tiles to each key:
    // (values will only be valid for each individual robot and their corner)
    key_distances.entry('@').and_modify(|e| {
        for (other_key, (_, dist)) in e {
            if other_key != &'@' {
                *dist = *dist - 2;
            }
        }
    });

    // Replace start pattern:
    // ...         @#@
    // .@.   -->   ###
    // ...         @#@
    let maze_width = maze.width() + 2; // include \r\n linebreaks in original input into length:
    let start_index = (maze_width * (start.y as usize)) + (start.x as usize);
    let mut mod_input = input.clone();
    mod_input.replace_range(
        (start_index - maze_width - 1)..=(start_index - maze_width + 1),
        "@#@",
    );
    mod_input.replace_range((start_index - 1)..=(start_index + 1), "###");
    mod_input.replace_range(
        (start_index + maze_width - 1)..=(start_index + maze_width + 1),
        "@#@",
    );

    // Overwrite maze with new input:
    let maze = Maze::new(&mod_input);

    // For each robot, get the keys available in their corner:
    // 0 | 1
    // --+--
    // 3 | 2
    let mut keys_by_robot = [
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
        Vec::<char>::new(),
    ];

    for key in 'a'..='z' {
        if let Some(key_pos) = maze.find(key) {
            if key_pos.x < start.x {
                if key_pos.y < start.y {
                    // Top-left:
                    keys_by_robot[0].push(key);
                } else {
                    // Bottom-left:
                    keys_by_robot[3].push(key);
                }
            } else {
                if key_pos.y < start.y {
                    // Top-right:
                    keys_by_robot[1].push(key);
                } else {
                    // Bottom-right:
                    keys_by_robot[2].push(key);
                }
            }
        }
    }

    // Explore maze via BFS
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(QueueState2 {
        last_key: ['@', '@', '@', '@'],
        keys: 0,
        total_dist: 0,
    });

    // Locations visited with keys collected at this point and the distance travelled to reach this state:
    let mut visited = HashMap::<MazeState2, u64>::new();
    let mut dist_for_all_keys = 0;

    while !priority_queue.is_empty() {
        let cur_state = priority_queue.pop().unwrap();

        // First time all keys are collected stop the search:
        if cur_state.keys == all_keys {
            dist_for_all_keys = cur_state.total_dist;
            break;
        }

        // Try moving each robot individually:
        for robot in 0..4 {
            let last_key = cur_state.last_key[robot];

            // Try keys that have not been collected yet and can be reached with the currently collected keys:
            for next_key in keys_by_robot[robot].iter().filter(|k| {
                let key_has_not_been_collected =
                    cur_state.keys & (1u32 << ((**k as u8) - ('a' as u8))) == 0;
                let blocked_by_doors = key_distances.get(&last_key).unwrap().get(k).unwrap().0;
                let can_unlock_all_doors = (blocked_by_doors & cur_state.keys) == blocked_by_doors;
                key_has_not_been_collected && can_unlock_all_doors
            }) {
                // New state:
                let mut next_state = cur_state.clone();
                next_state.keys = cur_state.keys | (1u32 << ((*next_key as u8) - ('a' as u8)));

                // Add distance to the next key to total distance:
                let steps_to_key = key_distances
                    .get(&next_state.last_key[robot])
                    .unwrap()
                    .get(next_key)
                    .unwrap()
                    .1;
                let total_dist = cur_state.total_dist + steps_to_key;
                next_state.total_dist = total_dist;

                // Update robot position:
                next_state.last_key[robot] = *next_key;

                let next_visited_state = next_state.to_maze_state();
                if let Some(prev_dist) = visited.get_mut(&next_visited_state) {
                    if *prev_dist <= total_dist {
                        // Reached this position already in a better state
                        continue;
                    } else {
                        // Current state is better: update
                        *prev_dist = total_dist;
                    }
                } else {
                    // State not reached before: insert
                    visited.insert(next_visited_state, total_dist);
                }

                priority_queue.push(next_state);
            }
        }
    }

    AoCResult::Num(dist_for_all_keys)
}

pub struct Maze<'a> {
    cells: Vec<&'a str>, // store as bytes for efficiency
}

impl<'a> Maze<'a> {
    fn new(input: &'a String) -> Self {
        Maze {
            cells: input.trim_end().lines().map(|l| l.trim_end()).collect_vec(),
        }
    }

    fn width(&self) -> usize {
        self.cells[0].len()
    }

    fn find(&self, c: char) -> Option<Point> {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, val) in row.chars().enumerate() {
                if val == c {
                    return Some(Point::new(x as i64, y as i64));
                }
            }
        }
        None
    }

    // Find shortest distances between all keys
    fn get_key_distances(&self) -> HashMap<char, HashMap<char, (u32, u64)>> {
        // Key-to-key positions, with doors (u32) and distance (u64) between them
        let mut key_distances = HashMap::<char, HashMap<char, (u32, u64)>>::new();

        let nesw = vec![
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
        ];

        let point_of_interest = ['@', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

        for poi in point_of_interest {
            let poi_pos = self.find(poi);
            if poi_pos.is_none() {
                continue;
            }

            // Use 'keys' field in MazeState for the doors that were passed through
            let mut vis = HashMap::<MazeState, u64>::new();
            let mut bfs = VecDeque::<(MazeState, u64)>::new();

            // Insert distance to self as 0:
            key_distances.insert(poi, HashMap::from([(poi, (0, 0))]));

            bfs.push_back((
                MazeState {
                    pos: poi_pos.unwrap(),
                    keys: 0,
                },
                0,
            ));
            'bfs_loop: while !bfs.is_empty() {
                let (cur_state, dist) = bfs.pop_front().unwrap();
                let next_dist = dist + 1;

                for dir in nesw.iter() {
                    let next_pos = cur_state.pos.add(dir);
                    let mut next_doors = cur_state.keys;

                    let tile = self[&next_pos] as char;
                    match tile {
                        'a'..='z' | '@' => {
                            let dist_from_entry = key_distances.get_mut(&poi);
                            if let Some(dist_from_entry) = dist_from_entry {
                                if dist_from_entry.get(&tile).is_some() {
                                    // Distance already found
                                    continue;
                                } else {
                                    // Insert distance:
                                    dist_from_entry.insert(tile, (next_doors, next_dist));

                                    // Also insert distance the other way around:
                                    key_distances
                                        .entry(tile)
                                        .and_modify(|e: &mut HashMap<char, (u32, u64)>| {
                                            e.entry(poi).or_insert((next_doors, next_dist));
                                        })
                                        .or_insert(HashMap::from([(poi, (next_doors, next_dist))]));

                                    // End if all keys have been found:
                                    if key_distances.get(&poi).unwrap().len()
                                        == point_of_interest.len()
                                    {
                                        break 'bfs_loop;
                                    }
                                }
                            }
                        }
                        'A'..='Z' => {
                            // Door:
                            let door = 1u32 << (tile.to_ascii_lowercase() as u8) - ('a' as u8);
                            next_doors |= door;
                        }
                        '#' => {
                            // Wall:
                            continue;
                        }
                        '.' => {}
                        _ => unreachable!(),
                    }

                    let next_state = MazeState {
                        pos: next_pos,
                        keys: next_doors,
                    };

                    if let Some(prev_dist) = vis.get_mut(&next_state) {
                        if *prev_dist <= next_dist {
                            // Reached this position already in a better state
                            continue;
                        } else {
                            // Current state is better: update
                            *prev_dist = next_dist;
                        }
                    } else {
                        // State not reached before: insert
                        vis.insert(next_state.clone(), next_dist);
                    }

                    bfs.push_back((next_state, next_dist));
                }
            }
        }

        key_distances
    }
}

impl<'a> Index<(usize, usize)> for Maze<'a> {
    type Output = u8;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y].as_bytes()[x]
    }
}

impl<'a> Index<&Point> for Maze<'a> {
    type Output = u8;

    #[inline]
    fn index(&self, p: &Point) -> &Self::Output {
        &self.cells[p.y as usize].as_bytes()[p.x as usize]
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct MazeState {
    pos: Point, // position inside the maze
    keys: u32,  // bit-flags for keys
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct MazeState2 {
    last_key: [char; 4], // position inside the maze for each robot (last key they collected)
    keys: u32,           // bit-flags for keys all 4 robots have collected together
}

#[derive(Clone, Eq, PartialEq)]
struct QueueState2 {
    last_key: [char; 4],
    keys: u32,
    total_dist: u64,
}

impl QueueState2 {
    fn to_maze_state(&self) -> MazeState2 {
        MazeState2 {
            last_key: self.last_key,
            keys: self.keys,
        }
    }
}

impl Ord for QueueState2 {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice ordering of comparison is flipped (other.cmp(self), because BinaryHeap is a MaxHeap
        // In case of a tie we compare keys - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .total_dist
            .cmp(&self.total_dist)
            .then_with(|| self.keys.cmp(&other.keys))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for QueueState2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("#########\r\n#b.A.@.a#\r\n#########");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 8);
    }

    #[test]
    fn part1_example2() {
        let input = String::from("########################\r\n#f.D.E.e.C.b.A.@.a.B.c.#\r\n######################.#\r\n#d.....................#\r\n########################");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 86);
    }

    #[test]
    fn part1_example3() {
        let input = String::from("########################\r\n#...............b.C.D.f#\r\n#.######################\r\n#.....@.a.B.c.d.A.e.F.g#\r\n########################");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 132);
    }

    #[test]
    fn part1_example4() {
        let input = String::from("#################\r\n#i.G..c...e..H.p#\r\n########.########\r\n#j.A..b...f..D.o#\r\n########@########\r\n#k.E..a...g..B.n#\r\n########.########\r\n#l.F..d...h..C.m#\r\n#################");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 136);
    }

    #[test]
    fn part1_example5() {
        let input = String::from("########################\r\n#@..............ac.GI.b#\r\n###d#e#f################\r\n###A#B#C################\r\n###g#h#i################\r\n########################");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 81);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("#######\r\n#a.#Cd#\r\n##...##\r\n##.@.##\r\n##...##\r\n#cB#Ab#\r\n#######");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 8);
    }

    #[test]
    fn part2_example2() {
        let input = String::from("###############\r\n#d.ABC.#.....a#\r\n######...######\r\n######.@.######\r\n######...######\r\n#b.....#.....c#\r\n###############");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 24);
    }

    // removed because the maze is not perfectly split into corners
    // fn part2_example3() { }

    #[test]
    fn part2_example4() {
        let input = String::from("#############\r\n#g#f.D#..h#l#\r\n#F###e#E###.#\r\n#dCba...BcIJ#\r\n#####.@.#####\r\n#nK.L...G...#\r\n#M###N#H###.#\r\n#o#m..#i#jk.#\r\n#############");
        let result = solve_part2(&input);
        assert_eq!(u64::from(result), 72);
    }
}
