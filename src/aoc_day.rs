use crate::aoc_result::AoCResult;
pub trait AoCDay {
    fn run_part1(&self, input: &String) -> AoCResult;
    fn run_part2(&self, input: &String) -> AoCResult;
}

macro_rules! make_day {
    ($struct_name:ident) => {
        use crate::aoc_day::AoCDay;
        pub struct $struct_name {}

        impl AoCDay for $struct_name {
            fn run_part1(&self, input: &String) -> AoCResult {
                solve_part1(input)
            }

            fn run_part2(&self, input: &String) -> AoCResult {
                solve_part2(input)
            }
        }
    };
}
