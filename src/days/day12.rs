use regex::Regex;

use crate::aoc_result::AoCResult;
use crate::shared::math;

make_day!(Day12);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut moons = parse_moons(input);
    let total_energy = run_simulation(&mut moons, 1000, SimulationMode::All);
    AoCResult::Num(total_energy)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let mut moons = parse_moons(input);
    let cycle = get_cycle_times(&mut moons);
    AoCResult::Num(cycle)
}

#[derive(Clone, PartialEq)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,

    vx: i64,
    vy: i64,
    vz: i64,
}

impl Moon {
    fn apply_gravity_from_x(&mut self, other: &Moon) {
        if other.x > self.x {
            self.vx += 1;
        } else if other.x < self.x {
            self.vx -= 1;
        }
    }

    fn apply_gravity_from_y(&mut self, other: &Moon) {
        if other.y > self.y {
            self.vy += 1;
        } else if other.y < self.y {
            self.vy -= 1;
        }
    }

    fn apply_gravity_from_z(&mut self, other: &Moon) {
        if other.z > self.z {
            self.vz += 1;
        } else if other.z < self.z {
            self.vz -= 1;
        }
    }

    fn apply_velocity_x(&mut self) {
        self.x += self.vx;
    }

    fn apply_velocity_y(&mut self) {
        self.y += self.vy;
    }

    fn apply_velocity_z(&mut self) {
        self.z += self.vz;
    }

    fn get_potential_energy(&self) -> u64 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u64
    }

    fn get_kinetic_energy(&self) -> u64 {
        (self.vx.abs() + self.vy.abs() + self.vz.abs()) as u64
    }
}

fn parse_moons(input: &String) -> Vec<Moon> {
    let re = Regex::new("<x=([-0-9]+), y=([-0-9]+), z=([-0-9]+)>").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l.trim_end()).unwrap();
            Moon {
                x: caps[1].parse::<i64>().unwrap(),
                y: caps[2].parse::<i64>().unwrap(),
                z: caps[3].parse::<i64>().unwrap(),
                vx: 0, vy: 0, vz: 0
            }
        })
        .collect()
}

#[derive(PartialEq)]
enum SimulationMode {
    OnlyX,
    OnlyY,
    OnlyZ,
    All
}

fn run_simulation(moons: &mut Vec<Moon>, cycles: usize, mode: SimulationMode) -> u64 {
    for _ in 0..cycles {
        // Update gravity:
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i < j {
                    // Split at j: first slice contains i, second slice contains j at first position
                    let (moons_1, moons_2) = moons.split_at_mut(j);
                    if mode == SimulationMode::All || mode == SimulationMode::OnlyX { moons_1[i].apply_gravity_from_x(&mut moons_2[0]); }
                    if mode == SimulationMode::All || mode == SimulationMode::OnlyY { moons_1[i].apply_gravity_from_y(&mut moons_2[0]); }
                    if mode == SimulationMode::All || mode == SimulationMode::OnlyZ { moons_1[i].apply_gravity_from_z(&mut moons_2[0]); }
                } else if j < i {
                    // Split at i: first slice contains j, second slice contains i
                    let (moons_1, moons_2) = moons.split_at_mut(i);
                    if mode == SimulationMode::All || mode == SimulationMode::OnlyX { moons_2[0].apply_gravity_from_x(&mut moons_1[j]); }
                    if mode == SimulationMode::All || mode == SimulationMode::OnlyY { moons_2[0].apply_gravity_from_y(&mut moons_1[j]); }
                    if mode == SimulationMode::All || mode == SimulationMode::OnlyZ { moons_2[0].apply_gravity_from_z(&mut moons_1[j]); }
                }
            }
        }

        // Update positions:
        for moon in moons.iter_mut() {
            if mode == SimulationMode::All || mode == SimulationMode::OnlyX { moon.apply_velocity_x(); }
            if mode == SimulationMode::All || mode == SimulationMode::OnlyY { moon.apply_velocity_y(); }
            if mode == SimulationMode::All || mode == SimulationMode::OnlyZ { moon.apply_velocity_z(); }
        }
    }

    moons
        .iter()
        .map(|m| m.get_kinetic_energy() * m.get_potential_energy())
        .sum()
}

fn get_cycle_times(moons: &mut Vec<Moon>) -> u64 {
    // Find X cycle
    let mut current_state = moons.clone();
    let mut cycle_x = 0u64;
    while cycle_x == 0 || &current_state != moons {
        run_simulation(&mut current_state, 1, SimulationMode::OnlyX);
        cycle_x += 1;
    }

    // Find Y cycle
    current_state = moons.clone();
    let mut cycle_y = 0u64;
    while cycle_y == 0 || &current_state != moons {
        run_simulation(&mut current_state, 1, SimulationMode::OnlyY);
        cycle_y += 1;
    }

    // Find Z cycle
    current_state = moons.clone();
    let mut cycle_z = 0u64;
    while cycle_z == 0 || &current_state != moons {
        run_simulation(&mut current_state, 1, SimulationMode::OnlyZ);
        cycle_z += 1;
    }

    math::lcm(math::lcm(cycle_x, cycle_y),cycle_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("<x=-1, y=0, z=2>\r\n<x=2, y=-10, z=-7>\r\n<x=4, y=-8, z=8>\r\n<x=3, y=5, z=-1>");
        let mut moons = parse_moons(&input);
        let total_energy = run_simulation(&mut moons, 10, SimulationMode::All);
        assert_eq!(total_energy, 179);
    }

    #[test]
    fn part1_example2() {
        let input = String::from("<x=-8, y=-10, z=0>\r\n<x=5, y=5, z=10>\r\n<x=2, y=-7, z=3>\r\n<x=9, y=-8, z=-3>");
        let mut moons = parse_moons(&input);
        let total_energy = run_simulation(&mut moons, 100, SimulationMode::All);
        assert_eq!(total_energy, 1940);
    }

    #[test]
    fn part2_example1() {
        let input = String::from("<x=-1, y=0, z=2>\r\n<x=2, y=-10, z=-7>\r\n<x=4, y=-8, z=8>\r\n<x=3, y=5, z=-1>");
        let mut moons = parse_moons(&input);
        let cycle = get_cycle_times(&mut moons);
        assert_eq!(cycle, 2772);
    }
}
