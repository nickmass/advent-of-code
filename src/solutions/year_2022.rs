use crate::{solution, Solution};

mod day_1;

pub fn days() -> Vec<Solution> {
    vec![solution!(1, day_1::part_one, day_1::part_two)]
}
