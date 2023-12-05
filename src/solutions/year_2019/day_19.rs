use super::intcode::{Interrupt, Machine, VecMem};

pub fn part_one(input: &str) -> usize {
    let mut machine = Machine::new(input);
    (0..50)
        .flat_map(|x| (0..50).map(move |y| (x, y)))
        .map(|(x, y)| inspect_position(&mut machine, x, y))
        .filter(|s| *s == Status::Pulled)
        .count()
}

pub fn part_two(input: &str) -> i32 {
    let mut machine = Machine::new(input);

    let (mut min_x, mut min_y, _) = (1..10)
        .flat_map(|x| (1..10).map(move |y| (x, y)))
        .map(|(x, y)| (x, y, inspect_position(&mut machine, x, y)))
        .find(|(_, _, s)| *s == Status::Pulled)
        .unwrap();

    'outer: loop {
        for x in min_x.. {
            match inspect_position(&mut machine, x, min_y) {
                Status::Pulled => (),
                Status::Stationary => continue,
            }

            min_x = x;

            match inspect_position(&mut machine, x + 99, min_y) {
                Status::Pulled => break,
                Status::Stationary => {
                    min_y += 1;
                    continue 'outer;
                }
            }
        }
        match inspect_position(&mut machine, min_x, min_y + 99) {
            Status::Pulled => return min_x * 10000 + min_y,
            Status::Stationary => min_x += 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Status {
    Pulled,
    Stationary,
}

fn inspect_position(machine: &mut Machine<i32, VecMem<i32>>, x: i32, y: i32) -> Status {
    machine.reset();
    machine.set_input(x);
    match machine.run() {
        Interrupt::Input => machine.set_input(y),
        i => unreachable!("{:?}", i),
    }
    match machine.run() {
        Interrupt::Output(0) => Status::Stationary,
        Interrupt::Output(1) => Status::Pulled,
        i => unreachable!("{:?}", i),
    }
}
