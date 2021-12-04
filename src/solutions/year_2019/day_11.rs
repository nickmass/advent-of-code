use crate::HashMap;

use super::{intcode, Point2};

pub fn part_one(input: &str) -> usize {
    let mut machine = intcode::Machine::<i64, _>::new(input);
    let mut painted_spots = HashMap::new();
    let mut point = Point2::new(0, 0);
    let mut dir = Direction::Up;
    loop {
        let color = *painted_spots.get(&point).unwrap_or(&0);
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Input => machine.set_input(color),
            _ => unreachable!(),
        }
        let new_color = match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Output(color) => color,
            _ => unreachable!(),
        };

        painted_spots.insert(point, new_color);
        let movement = match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Output(movement) => movement,
            _ => unreachable!(),
        };

        dir = dir.do_move(movement == 0, &mut point);
    }

    painted_spots.len()
}

pub fn part_two(input: &str) -> String {
    let mut machine = intcode::Machine::<i64, _>::new(input);
    let mut painted_spots = HashMap::new();
    let mut point = Point2::new(0, 0);
    let mut dir = Direction::Up;
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    painted_spots.insert(point, 1);

    loop {
        let color = *painted_spots.get(&point).unwrap_or(&0);
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Input => machine.set_input(color),
            _ => unreachable!(),
        }
        let new_color = match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Output(color) => color,
            _ => unreachable!(),
        };

        painted_spots.insert(point, new_color);

        min_x = point.x.min(min_x);
        max_x = point.x.max(max_x);
        min_y = point.y.min(min_x);
        max_y = point.y.max(max_y);

        let movement = match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Output(movement) => movement,
            _ => unreachable!(),
        };

        dir = dir.do_move(movement == 0, &mut point);
    }

    let mut result = String::new();

    result.push('\n');
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let color = *painted_spots.get(&Point2::new(x, y)).unwrap_or(&0);
            if color == 0 {
                result.push_str("  ");
            } else {
                result.push_str("##");
            }
        }
        result.push('\n');
    }

    result
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn do_move(self, turn_left: bool, point: &mut Point2<i32>) -> Self {
        match (self, turn_left) {
            (Direction::Up, true) | (Direction::Down, false) => {
                point.x -= 1;
                Direction::Left
            }
            (Direction::Up, false) | (Direction::Down, true) => {
                point.x += 1;
                Direction::Right
            }
            (Direction::Left, false) | (Direction::Right, true) => {
                point.y -= 1;
                Direction::Up
            }
            (Direction::Left, true) | (Direction::Right, false) => {
                point.y += 1;
                Direction::Down
            }
        }
    }
}
