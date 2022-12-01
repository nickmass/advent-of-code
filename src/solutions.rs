mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;

pub use year_2019::days as days_2019;
pub use year_2020::days as days_2020;
pub use year_2021::days as days_2021;
pub use year_2022::days as days_2022;

pub type SolutionFunc = Box<dyn Fn(&str) -> Box<dyn std::fmt::Display>>;

pub struct Solution {
    pub day: u32,
    pub part_one: SolutionFunc,
    pub part_two: SolutionFunc,
}

impl Solution {
    pub fn new(day: u32, part_one: SolutionFunc, part_two: SolutionFunc) -> Self {
        Self {
            day,
            part_one,
            part_two,
        }
    }
}

#[macro_export]
macro_rules! solution {
    ($day:expr, $part_one:path, $part_two:path) => {
        crate::solutions::Solution::new(
            $day,
            Box::new(|input| Box::new($part_one(input))),
            Box::new(|input| Box::new($part_two(input))),
        )
    };
}
