use crate::solutions::SolutionCollection;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

pub fn days() -> SolutionCollection {
    SolutionCollection::new()
        .add(1, day_1::part_one, day_1::part_two)
        .add(2, day_2::part_one, day_2::part_two)
        .add(3, day_3::part_one, day_3::part_two)
        .add(4, day_4::part_one, day_4::part_two)
}
