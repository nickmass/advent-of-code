use crate::solutions::SolutionCollection;

mod day_01;
mod day_02;
mod day_03;

pub fn days() -> SolutionCollection {
    SolutionCollection::new()
        .add(1, day_01::part_one, day_01::part_two)
        .add(2, day_02::part_one, day_02::part_two)
        .add(3, day_03::part_one, day_03::part_two)
}
