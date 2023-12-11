use crate::solutions::SolutionCollection;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

mod day_10;
mod day_11;

pub fn days() -> SolutionCollection {
    SolutionCollection::new()
        .add(1, day_1::part_one, day_1::part_two)
        .add(2, day_2::part_one, day_2::part_two)
        .add(3, day_3::part_one, day_3::part_two)
        .add(4, day_4::part_one, day_4::part_two)
        .add(5, day_5::part_one, day_5::part_two)
        .add(6, day_6::part_one, day_6::part_two)
        .add(7, day_7::part_one, day_7::part_two)
        .add(8, day_8::part_one, day_8::part_two)
        .add(9, day_9::part_one, day_9::part_two)
        .add(10, day_10::part_one, day_10::part_two)
        .add(11, day_11::part_one, day_11::part_two)
}
