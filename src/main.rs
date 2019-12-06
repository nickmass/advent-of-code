use fxhash::*;

use std::cell::RefCell;
#[allow(unused)]
use std::collections::*;
use std::fmt::Display;
use std::str::FromStr;

mod day3;
mod intcode;

thread_local! {
    static PROBLEMS: RefCell<Vec<&'static dyn Problem>> = RefCell::new(Vec::new());
}

macro_rules! problem {
    ($name:ident, $day:expr, $part_one:ident, $part_two:ident) => {
        struct $name;

        impl $name {
            const INPUT: &'static str = include_str!(concat!("input/input_", $day, ".txt"));
        }

        impl Problem for $name {
            fn day(&self) -> usize {
                $day
            }

            fn part_one(&self) -> Box<dyn Display> {
                Box::new($part_one(Self::INPUT))
            }

            fn part_two(&self) -> Box<dyn Display> {
                Box::new($part_two(Self::INPUT))
            }
        }

        PROBLEMS.with(|p| p.borrow_mut().push(&$name));
    };
}

trait Problem {
    fn day(&self) -> usize;
    fn part_one(&self) -> Box<dyn Display>;
    fn part_two(&self) -> Box<dyn Display>;
}

fn main() {
    problem!(DayOne, 1, day_one_a, day_one_b);
    problem!(DayTwo, 2, day_two_a, day_two_b);
    problem!(DayThree, 3, day_three_a, day_three_b);
    problem!(DayFour, 4, day_four_a, day_four_b);
    problem!(DayFive, 5, day_five_a, day_five_b);
    problem!(DaySix, 6, day_six_a, day_six_b);

    PROBLEMS.with(|problems| {
        let problems = problems.borrow();
        let mut results = Vec::with_capacity(problems.len());

        let start = std::time::Instant::now();
        for problem in problems.iter() {
            let start = std::time::Instant::now();
            let part_one = problem.part_one();
            let duration_one = start.elapsed();
            let start = std::time::Instant::now();
            let part_two = problem.part_two();
            let duration_two = start.elapsed();
            results.push((
                problem.day(),
                part_one,
                part_two,
                duration_one.as_millis(),
                duration_two.as_millis(),
            ));
        }
        let duration = start.elapsed();

        for result in &results {
            println!("{:>2}-1:{:>10}{:>9}ms", result.0, result.1, result.3);
            println!("{:>2}-2:{:>10}{:>9}ms", result.0, result.2, result.4);
        }
        println!("Total Duration:{:>9}ms", duration.as_millis());
    });
}

fn day_one_a(input: &'static str) -> i32 {
    input
        .split('\n')
        .map(i32::from_str)
        .filter_map(Result::ok)
        .map(|f| f / 3 - 2)
        .sum::<i32>()
}

fn day_one_b(input: &'static str) -> i32 {
    input
        .split('\n')
        .map(i32::from_str)
        .filter_map(Result::ok)
        .map(|f| {
            let mut f_mod = f / 3 - 2;
            let mut fuel_acc = 0;
            while f_mod > 0 {
                fuel_acc += f_mod;
                f_mod = f_mod / 3 - 2;
            }

            fuel_acc
        })
        .sum::<i32>()
}

fn day_two_a(input: &'static str) -> i32 {
    let mut machine = intcode::Machine::new(input);
    machine.poke(1, 12);
    machine.poke(2, 2);
    machine.run();
    machine.peek(0)
}

fn day_two_b(input: &'static str) -> i32 {
    let mut machine = intcode::Machine::new(input);

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

fn day_three_a(input: &'static str) -> i32 {
    let mut lines = input.split('\n');

    let first_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(day3::Movement::from_str)
        .filter_map(Result::ok);
    let second_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(day3::Movement::from_str)
        .filter_map(Result::ok);

    let second_points: FxHashSet<_> = day3::Point::move_iter(second_line).collect();

    day3::Point::move_iter(first_line)
        .filter_map(|p| second_points.get(&p))
        .map(|p| p.distance())
        .min()
        .unwrap_or(0)
}

fn day_three_b(input: &'static str) -> usize {
    let mut lines = input.split('\n');

    let first_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(day3::Movement::from_str)
        .filter_map(Result::ok);

    let second_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(day3::Movement::from_str)
        .filter_map(Result::ok);

    let second_points: FxHashSet<_> = day3::Point::move_iter(second_line).stepped().collect();

    day3::Point::move_iter(first_line)
        .stepped()
        .filter_map(|p| second_points.get(&p).map(|pp| p.steps + pp.steps))
        .min()
        .unwrap_or(0)
}

fn day_four_a(input: &'static str) -> usize {
    let mut range = input
        .split('-')
        .map(str::trim)
        .map(usize::from_str)
        .filter_map(Result::ok);

    let minimum = range.next().unwrap();
    let maximum = range.next().unwrap();

    let mut matches = Vec::new();
    for n in minimum..maximum {
        let mut value = n % 10;
        let mut remainder = n / 10;
        let mut has_pair = false;
        let mut never_decreases = true;
        for _i in 0..6 {
            let next_value = remainder % 10;

            if value == next_value {
                has_pair = true;
            }

            if value < next_value {
                never_decreases = false;
            }

            value = next_value;
            remainder = remainder / 10;
        }

        if has_pair && never_decreases {
            matches.push(n);
        }
    }

    matches.len()
}

fn day_four_b(input: &'static str) -> usize {
    let mut range = input
        .split('-')
        .map(str::trim)
        .map(usize::from_str)
        .filter_map(Result::ok);

    let minimum = range.next().unwrap();
    let maximum = range.next().unwrap();

    let mut matches = Vec::new();
    for n in minimum..maximum {
        let mut value = n % 10;
        let mut remainder = n / 10;
        let mut in_pair = 0;
        let mut has_pair = false;
        let mut never_decreases = true;
        for _i in 0..6 {
            let next_value = remainder % 10;

            if value == next_value {
                in_pair += 1;
            }
            if value != next_value {
                if in_pair == 1 {
                    has_pair = true;
                }
                in_pair = 0;
            }

            if value < next_value {
                never_decreases = false;
            }

            value = next_value;
            remainder = remainder / 10;
        }

        if has_pair && never_decreases {
            matches.push(n);
        }
    }

    matches.len()
}

fn day_five_a(input: &'static str) -> i32 {
    let mut machine = intcode::Machine::new(input);
    let mut result = 0;
    loop {
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Input => machine.set_input(1),
            intcode::Interrupt::Output(value) => result = value,
        }
    }

    result
}

fn day_five_b(input: &'static str) -> i32 {
    let mut machine = intcode::Machine::new(input);
    let mut result = 0;
    loop {
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Input => machine.set_input(5),
            intcode::Interrupt::Output(value) => result = value,
        }
    }

    result
}

#[derive(Debug, Clone)]
enum Orbit {
    Root,
    Child(usize),
}

fn day_six_a(input: &'static str) -> usize {
    use std::convert::TryInto;
    let lines = input.as_bytes().chunks(8);

    let mut orbits = FxHashMap::default();
    let mut store = Vec::new();

    for line in lines {
        let parent: [u8; 3] = line[0..3].try_into().unwrap();
        let child: [u8; 3] = line[4..7].try_into().unwrap();

        let child_orbit = if let Some(&parent_orbit) = orbits.get(&parent) {
            Orbit::Child(parent_orbit)
        } else {
            let parent_orbit = store.len();
            let child = Orbit::Child(parent_orbit);
            store.push(Orbit::Root);
            orbits.insert(parent, parent_orbit);
            child
        };

        if let Some(&existing_child) = orbits.get(&child) {
            store[existing_child] = child_orbit;
        } else {
            store.push(child_orbit);
            orbits.insert(child, store.len() - 1);
        }
    }

    let mut orbit_count = 0;
    for orb in &store {
        orbit_count += match *orb {
            Orbit::Root => 0,
            Orbit::Child(parent_idx) => {
                let mut acc = 1;
                let mut next = Some(parent_idx);
                while let Some(next_idx) = next.take() {
                    match store[next_idx] {
                        Orbit::Root => next = None,
                        Orbit::Child(parent_idx) => {
                            next = Some(parent_idx);
                            acc += 1;
                        }
                    }
                }

                acc
            }
        }
    }

    orbit_count
}

fn day_six_b(input: &'static str) -> usize {
    use std::convert::TryInto;
    let lines = input.as_bytes().chunks(8);

    let mut orbits = FxHashMap::default();
    let mut store = Vec::new();

    for line in lines {
        let parent: [u8; 3] = line[0..3].try_into().unwrap();
        let child: [u8; 3] = line[4..7].try_into().unwrap();

        let child_orbit = if let Some(&parent_orbit) = orbits.get(&parent) {
            Orbit::Child(parent_orbit)
        } else {
            let parent_orbit = store.len();
            let child = Orbit::Child(parent_orbit);
            store.push(Orbit::Root);
            orbits.insert(parent, parent_orbit);
            child
        };

        if let Some(&existing_child) = orbits.get(&child) {
            store[existing_child] = child_orbit;
        } else {
            store.push(child_orbit);
            orbits.insert(child, store.len() - 1);
        }
    }

    let me = orbits[b"YOU"];
    let santa = orbits[b"SAN"];

    let path_to_root = |start: usize| -> Vec<usize> {
        let mut next = start;
        let mut acc = Vec::new();
        loop {
            match store[next] {
                Orbit::Root => return acc,
                Orbit::Child(idx) => {
                    next = idx;
                    acc.push(idx);
                }
            }
        }
    };

    let my_path = path_to_root(me);
    let santas_path = path_to_root(santa);

    let mut count = 0;
    for node in my_path {
        if let Some(meeting_point) = santas_path.iter().position(|&p| p == node) {
            count += meeting_point;
            break;
        }
        count += 1;
    }

    count
}
