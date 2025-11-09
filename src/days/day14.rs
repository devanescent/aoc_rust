use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

use crate::aoc_result::AoCResult;

make_day!(Day14);

pub fn solve_part1(input: &String) -> AoCResult {
    let reactions = get_reactions(input);

    // Process all chemicals needed to produce 1 fuel:
    let mut chemicals_to_produce : VecDeque<Chemical> = VecDeque::new();
    chemicals_to_produce.push_back(Chemical { name: "FUEL".to_owned(), quantity: 1 });

    let ore_amount = produce_chemicals_from_ore(&reactions, &mut chemicals_to_produce);
    AoCResult::Num(ore_amount)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let reactions = get_reactions(input);

    // From part 1: produce 1 fuel to get unused chemicals during the process
    let mut chemicals_to_produce : VecDeque<Chemical> = VecDeque::new();
    chemicals_to_produce.push_back(Chemical { name: "FUEL".to_owned(), quantity: 1 });
    let ore_for_1_fuel = produce_chemicals_from_ore(&reactions, &mut chemicals_to_produce);

    let available_ore = 1000000000000u64;

    // Get lower bound for amount of fuel that can be produced from this:
    let mut fuel_lower_bound = available_ore / ore_for_1_fuel;

    // Find an upper bound value:
    let mut fuel_upper_bound = fuel_lower_bound * 2;
    loop {
        chemicals_to_produce.push_back(Chemical { name: "FUEL".to_owned(), quantity: fuel_upper_bound });
        let ore_amount = produce_chemicals_from_ore(&reactions, &mut chemicals_to_produce);
        if ore_amount > available_ore {
            break;
        } else {
            fuel_upper_bound += fuel_lower_bound;
        }
    }

    // Binary search the amount of fuel that can be produced with the given amount of ORE:
    while fuel_lower_bound < fuel_upper_bound {
        // Midpoint:
        let fuel_amount = fuel_lower_bound + ((fuel_upper_bound - fuel_lower_bound) / 2);

        chemicals_to_produce.push_back(Chemical { name: "FUEL".to_owned(), quantity: fuel_amount });
        let ore_amount = produce_chemicals_from_ore(&reactions, &mut chemicals_to_produce);


        if ore_amount > available_ore {
            // Too much, lower upper bound:
            fuel_upper_bound = fuel_amount
        } else if ore_amount < available_ore {
            // Too few, raise lower bound:
            if fuel_amount > fuel_lower_bound {
                fuel_lower_bound = fuel_amount;
            } else {
                break;
            }
        } else {
            // Direct match:
            fuel_lower_bound = fuel_amount;
            break;
        }
    }

    AoCResult::Num(fuel_lower_bound)
}

fn get_reactions(input: &String) -> HashMap<String, Reaction> {
    input
        .trim_end()
        .lines()
        .map(|l| {
            let in_out = l.split_once("=>").unwrap();
            
            let input = in_out.0.split(",")
                .map(|i| Chemical::from(i))
                .collect_vec();

            let output = Chemical::from(in_out.1);

            (output.name.clone(), Reaction { input, output: output })

        })
        .collect()
}

fn produce_chemicals_from_ore(
    reactions: &HashMap<String, Reaction>,
    chemicals_to_produce: &mut VecDeque<Chemical>) -> u64 {

    // Get amount of ORE needed for producing the given chemicals:
    let mut ore_amount = 0u64;

    // Keep note of chemicals that were "overproduced":
    let mut unused_chems =  HashMap::<String, Chemical>::new();

    while !chemicals_to_produce.is_empty() {
        let mut next_chem = chemicals_to_produce.pop_front().unwrap();

        // Check if there exist some unused chemicals from an earlier reaction:
        if let Some(unused) = unused_chems.get_mut(&next_chem.name) && unused.quantity > 0 {
            if unused.quantity >= next_chem.quantity {
                unused.quantity -= next_chem.quantity;
                continue;
            } else {
                next_chem.quantity -= unused.quantity;
                unused.quantity = 0;
            }
        }

        if next_chem.quantity > 0 {
            if next_chem.name == "ORE" {
                ore_amount += next_chem.quantity;
            } else {
                // Find reaction to procude this chem:
                let reaction = reactions.get(&next_chem.name).unwrap();
                let scale = if reaction.output.quantity >= next_chem.quantity {
                    1
                } else if next_chem.quantity % reaction.output.quantity != 0 {
                    next_chem.quantity / reaction.output.quantity + 1
                } else {
                    next_chem.quantity / reaction.output.quantity
                };

                for input in reaction.input.iter() {
                    chemicals_to_produce.push_back(input.clone_and_scale(scale));
                }

                // Keep track of unused quantities:
                let output = reaction.output.clone_and_scale(scale);
                if output.quantity > next_chem.quantity {
                    let unused_amount = output.quantity - next_chem.quantity;
                    unused_chems.entry(output.name.clone())
                        .and_modify(|c| { c.update_quantity(|q| q + unused_amount ); })
                        .or_insert_with_key(|key| Chemical { name: key.clone(), quantity: unused_amount });
                }
            }
        }
    }

    ore_amount

}

#[derive(Clone)]
struct Chemical {
    name: String,
    quantity: u64
}

impl Chemical {
    fn update_quantity<F: Fn(u64) -> u64>(&mut self, update_fn: F) {
        self.quantity = update_fn(self.quantity);
    }

    fn clone_and_scale(&self, scale: u64) -> Self {
        Chemical { name: self.name.clone(), quantity: self.quantity * scale }
    }
}

impl From<&str> for Chemical {
    fn from(value: &str) -> Self {
        let quan_and_ingr = value.trim().split_once(' ').unwrap();
        Chemical { 
            name: quan_and_ingr.1.to_owned(),
            quantity: quan_and_ingr.0.parse().unwrap()
        }
    }
}

struct Reaction {
    input: Vec<Chemical>,
    output: Chemical
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = String::from("10 ORE => 10 A\r\n1 ORE => 1 B\r\n7 A, 1 B => 1 C\r\n7 A, 1 C => 1 D\r\n7 A, 1 D => 1 E\r\n7 A, 1 E => 1 FUEL");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 31);
    }

    #[test]
    fn part1_example2() {
        let input = String::from("9 ORE => 2 A\r\n8 ORE => 3 B\r\n7 ORE => 5 C\r\n3 A, 4 B => 1 AB\r\n5 B, 7 C => 1 BC\r\n4 C, 1 A => 1 CA\r\n2 AB, 3 BC, 4 CA => 1 FUEL");
        let result = solve_part1(&input);
        assert_eq!(u64::from(result), 165);
    }
}
