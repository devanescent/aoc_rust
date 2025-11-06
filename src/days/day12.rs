use regex::Regex;

use crate::aoc_result::AoCResult;

make_day!(Day12);

pub fn solve_part1(input: &String) -> AoCResult {
    let mut moons = parse_moons(input);
    let total_energy = run_simulation(&mut moons, 1000);
    AoCResult::Num(total_energy)
}

pub fn solve_part2(_input: &String) -> AoCResult {
    AoCResult::NotImplemented
}

#[derive(Clone)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,

    vx: i64,
    vy: i64,
    vz: i64,
}

impl Moon {
    fn apply_gravity_from(&mut self, other: &Moon) {
        if other.x > self.x {
            self.vx += 1;
        } else if other.x < self.x {
            self.vx -= 1;
        }

        if other.y > self.y {
            self.vy += 1;
        } else if other.y < self.y {
            self.vy -= 1;
        }

        if other.z > self.z {
            self.vz += 1;
        } else if other.z < self.z {
            self.vz -= 1;
        }
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
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

fn run_simulation(moons: &mut Vec<Moon>, cycles: usize) -> u64 {
    for _ in 0..cycles {
        // Update gravity:
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i < j {
                    // Split at j: first slice contains i, second slice contains j at first position
                    let (moons_1, moons_2) = moons.split_at_mut(j);
                    moons_1[i].apply_gravity_from(&mut moons_2[0]);
                } else if j < i {
                    // Split at i: first slice contains j, second slice contains i
                    let (moons_1, moons_2) = moons.split_at_mut(i);
                    moons_2[0].apply_gravity_from(&mut moons_1[j]);
                }
            }
        }

        // Update positions:
        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    moons
        .iter()
        .map(|m| m.get_kinetic_energy() * m.get_potential_energy())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("<x=-1, y=0, z=2>\r\n<x=2, y=-10, z=-7>\r\n<x=4, y=-8, z=8>\r\n<x=3, y=5, z=-1>");
        let mut moons = parse_moons(&input);
        let total_energy = run_simulation(&mut moons, 10);
        assert_eq!(total_energy, 179);
    }

    #[test]
    fn part1_example2() {
        let input = String::from("<x=-8, y=-10, z=0>\r\n<x=5, y=5, z=10>\r\n<x=2, y=-7, z=3>\r\n<x=9, y=-8, z=-3>");
        let mut moons = parse_moons(&input);
        let total_energy = run_simulation(&mut moons, 100);
        assert_eq!(total_energy, 1940);
    }
}
