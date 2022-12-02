use crate::{solution, Solution};

mod day_1;
mod day_2;

pub fn days() -> Vec<Solution> {
    vec![
        solution!(1, day_1::part_one, day_1::part_two),
        solution!(2, day_2::part_one, day_2::part_two),
    ]
}
