use super::intcode::Machine;

pub fn part_one(input: &str) -> i32 {
    let mut machine = Machine::new(input);
    machine.poke(1, 12);
    machine.poke(2, 2);
    machine.run();
    machine.peek(0)
}

pub fn part_two(input: &str) -> i32 {
    let mut machine = Machine::new(input);

    for a in 0..10000 {
        machine.poke(1, a / 100);
        machine.poke(2, a % 100);
        machine.run();

        if machine.peek(0) == 19690720 {
            return a;
        }

        machine.reset();
    }

    panic!("Missed It")
}
