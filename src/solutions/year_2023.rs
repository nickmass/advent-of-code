use crate::solutions::SolutionCollection;

mod day_1;
mod day_2;

pub fn days() -> SolutionCollection {
    SolutionCollection::new()
        .add(1, day_1::part_one, day_1::part_two)
        .add(2, day_2::part_one, day_2::part_two)
}
