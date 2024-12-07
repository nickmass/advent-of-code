mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;
mod year_2023;
mod year_2024;

pub use year_2019::days as days_2019;
pub use year_2020::days as days_2020;
pub use year_2021::days as days_2021;
pub use year_2022::days as days_2022;
pub use year_2023::days as days_2023;
pub use year_2024::days as days_2024;

pub type SolutionFunc = Box<dyn Fn(&str) -> Box<dyn std::fmt::Display>>;

pub struct Solution {
    pub day: u32,
    pub part_one: SolutionFunc,
    pub part_two: SolutionFunc,
}

impl Solution {
    pub fn new<
        F1: Fn(&str) -> D1 + 'static,
        D1: std::fmt::Display + 'static,
        F2: Fn(&str) -> D2 + 'static,
        D2: std::fmt::Display + 'static,
    >(
        day: u32,
        part_one: F1,
        part_two: F2,
    ) -> Self {
        Self {
            day,
            part_one: Box::new(move |input| Box::new(part_one(input))),
            part_two: Box::new(move |input| Box::new(part_two(input))),
        }
    }
}

pub struct SolutionCollection {
    pub solutions: Vec<Solution>,
}

impl SolutionCollection {
    pub fn new() -> Self {
        Self {
            solutions: Vec::new(),
        }
    }

    pub fn add<F1, F2, D1, D2>(mut self, day: u32, part_one: F1, part_two: F2) -> Self
    where
        F1: Fn(&str) -> D1 + 'static,
        D1: std::fmt::Display + 'static,
        F2: Fn(&str) -> D2 + 'static,
        D2: std::fmt::Display + 'static,
    {
        self.solutions.push(Solution::new(day, part_one, part_two));
        self
    }

    pub fn solutions(&self) -> impl Iterator<Item = &Solution> {
        self.solutions.iter()
    }
}
