use crate::solutions::SolutionCollection;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
//mod day_21;
mod day_22;

pub fn days() -> SolutionCollection {
    SolutionCollection::new()
        .add(1, day_01::part_one, day_01::part_two)
        .add(2, day_02::part_one, day_02::part_two)
        .add(3, day_03::part_one, day_03::part_two)
        .add(4, day_04::part_one, day_04::part_two)
        .add(5, day_05::part_one, day_05::part_two)
        .add(6, day_06::part_one, day_06::part_two)
        .add(7, day_07::part_one, day_07::part_two)
        .add(8, day_08::part_one, day_08::part_two)
        .add(9, day_09::part_one, day_09::part_two)
        .add(10, day_10::part_one, day_10::part_two)
        .add(11, day_11::part_one, day_11::part_two)
        .add(12, day_12::part_one, day_12::part_two)
        .add(13, day_13::part_one, day_13::part_two)
        .add(14, day_14::part_one, day_14::part_two)
        .add(15, day_15::part_one, day_15::part_two)
        .add(16, day_16::part_one, day_16::part_two)
        .add(17, day_17::part_one, day_17::part_two)
        .add(18, day_18::part_one, day_18::part_two)
        .add(19, day_19::part_one, day_19::part_two)
        .add(20, day_20::part_one, day_20::part_two)
        //.add(21, day_21::part_one, day_21::part_two)
        .add(22, day_22::part_one, day_22::part_two)
}
