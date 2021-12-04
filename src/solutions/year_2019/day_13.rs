use super::{intcode, Point2};

pub fn part_one(input: &str) -> i64 {
    let mut machine = intcode::Machine::new(input);
    let mut block_count = 0;
    let mut output_index = 0;
    loop {
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Output(2) if output_index % 3 == 2 => {
                block_count += 1;
                output_index += 1;
            }
            intcode::Interrupt::Output(_) => {
                output_index += 1;
            }
            _ => unreachable!(),
        }
    }
    block_count
}

pub fn part_two(input: &str) -> i64 {
    let mut machine = intcode::Machine::new(input);
    machine.poke(0, 2);
    let mut score = 0;
    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut output_index = 0;
    let mut render_point = Point2::new(0, 0);
    loop {
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Input => {
                let input = match paddle_x.cmp(&ball_x) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Greater => -1,
                    std::cmp::Ordering::Equal => 0,
                };
                machine.set_input(input);
            }
            intcode::Interrupt::Output(tile) if output_index % 3 == 2 => {
                if render_point == Point2::new(-1, 0) {
                    score = tile;
                } else {
                    match tile {
                        3 => paddle_x = render_point.x,
                        4 => ball_x = render_point.x,
                        _ => (),
                    }
                }
                output_index += 1;
            }
            intcode::Interrupt::Output(value) => {
                match output_index % 3 {
                    0 => render_point.x = value,
                    1 => render_point.y = value,
                    _ => unreachable!(),
                }
                output_index += 1;
            }
        }
    }

    score
}
