use crate::solutions::SolutionCollection;

mod day_01;

pub fn days() -> SolutionCollection {
    SolutionCollection::new().add(1, day_01::part_one, day_01::part_two)
}
