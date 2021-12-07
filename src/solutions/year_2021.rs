use crate::{solution, Solution};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

pub fn days() -> Vec<Solution> {
    vec![
        solution!(1, day_1::part_one, day_1::part_two),
        solution!(2, day_2::part_one, day_2::part_two),
        solution!(3, day_3::part_one, day_3::part_two),
        solution!(4, day_4::part_one, day_4::part_two),
        solution!(5, day_5::part_one, day_5::part_two),
        solution!(6, day_6::part_one, day_6::part_two),
        solution!(7, day_7::part_one, day_7::part_two),
    ]
}
