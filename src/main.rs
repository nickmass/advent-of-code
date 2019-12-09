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
    problem!(DaySeven, 7, day_seven_a, day_seven_b);
    problem!(DayEight, 8, day_eight_a, day_eight_b);
    problem!(DayNine, 9, day_nine_a, day_nine_b);

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

fn day_two_a(input: &'static str) -> i64 {
    let mut machine = intcode::Machine::new(input);
    machine.poke(1, 12);
    machine.poke(2, 2);
    machine.run();
    machine.peek(0)
}

fn day_two_b(input: &'static str) -> i64 {
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
    let mut lines = input.trim().split('\n');

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

    let first_segments = day3::LineIter::new(first_line);
    let second_segments: Vec<_> = day3::LineIter::new(second_line).collect();

    let intersections = first_segments.filter_map(|(_, f)| {
        second_segments
            .iter()
            .filter_map(|(_, s)| f.crosses(s))
            .next()
    });

    intersections.map(|p| p.distance()).min().unwrap_or(0)
}

fn day_three_b(input: &'static str) -> i32 {
    let mut lines = input.trim().split('\n');

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

    let first_segments = day3::LineIter::new(first_line);
    let second_segments: Vec<_> = day3::LineIter::new(second_line)
        .scan(0, |distance, (p, l)| {
            *distance += l.length();
            Some((*distance, p, l))
        })
        .collect();

    let mut min_distance = std::i32::MAX;

    let mut fl_distance = 0;
    for (fl_start, fl) in first_segments {
        for (sl_distance, sl_start, sl) in &second_segments {
            let total_distance = sl_distance + fl_distance;

            if let Some(cross) = fl.crosses(sl) {
                let distance = total_distance + fl_start.distance_to(&cross)
                    - (sl.length() - sl_start.distance_to(&cross));
                if distance < min_distance {
                    min_distance = distance;
                    break;
                }
            }

            if total_distance > min_distance {
                break;
            }
        }
        fl_distance += fl.length();
    }

    min_distance
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
    'outer: for n in minimum..maximum {
        let mut value = n % 10;
        let mut remainder = n / 10;
        let mut has_pair = false;
        for _ in 0..6 {
            let next_value = remainder % 10;

            if value < next_value {
                continue 'outer;
            }

            if value == next_value {
                has_pair = true;
            }

            value = next_value;
            remainder = remainder / 10;
        }

        if has_pair {
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
    'outer: for n in minimum..maximum {
        let mut value = n % 10;
        let mut remainder = n / 10;
        let mut in_pair = 0;
        let mut has_pair = false;
        for _ in 0..6 {
            let next_value = remainder % 10;

            if value < next_value {
                continue 'outer;
            }

            if value == next_value {
                in_pair += 1;
            }
            if value != next_value {
                if in_pair == 1 {
                    has_pair = true;
                }
                in_pair = 0;
            }

            value = next_value;
            remainder = remainder / 10;
        }

        if has_pair {
            matches.push(n);
        }
    }

    matches.len()
}

fn day_five_a(input: &'static str) -> i64 {
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

fn day_five_b(input: &'static str) -> i64 {
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

fn generate_permutations<T: Clone, A: AsRef<[T]>>(arr: A) -> Vec<Vec<T>> {
    let mut working_arr = arr.as_ref().to_vec();
    let len = working_arr.len();
    let mut results = Vec::with_capacity((1..=len).product());
    generate_permutations_rec(&mut working_arr, len, &mut results);
    results
}

fn generate_permutations_rec<T: Clone>(arr: &mut [T], n: usize, results: &mut Vec<Vec<T>>) {
    if n == 1 {
        results.push(arr.to_vec());

        return;
    }
    for i in 0..n {
        arr.swap(i, n - 1);
        generate_permutations_rec(arr, n - 1, results);
        arr.swap(i, n - 1);
    }
}

fn day_seven_a(input: &'static str) -> i64 {
    let mut machine = intcode::Machine::new(input);
    let all_perms = generate_permutations([0, 1, 2, 3, 4]);
    let mut results = Vec::with_capacity(all_perms.len());

    for perm in all_perms {
        let mut next_input = 0;

        for phase in perm {
            machine.run();
            machine.set_input(phase);
            machine.run();
            machine.set_input(next_input);
            if let intcode::Interrupt::Output(value) = machine.run() {
                next_input = value;
            }
            machine.reset();
        }

        results.push(next_input);
    }

    results.into_iter().max().unwrap_or(0)
}

fn day_seven_b(input: &'static str) -> i64 {
    let mut machines: Vec<_> = (0..5).map(|_| intcode::Machine::new(input)).collect();
    let all_perms = generate_permutations([5, 6, 7, 8, 9]);
    let mut results = Vec::with_capacity(all_perms.len());

    for perm in all_perms {
        for (m, phase) in machines.iter_mut().zip(perm) {
            m.run();
            m.set_input(phase);
        }

        let mut next_input = 0;
        let mut done = false;
        while !done {
            for m in &mut machines {
                loop {
                    match m.run() {
                        intcode::Interrupt::Input => m.set_input(next_input),
                        intcode::Interrupt::Output(value) => {
                            next_input = value;
                            break;
                        }
                        intcode::Interrupt::Halt => {
                            done = true;
                            break;
                        }
                    }
                }
            }
        }

        for m in &mut machines {
            m.reset();
        }

        results.push(next_input);
    }

    results.into_iter().max().unwrap_or(0)
}

fn day_eight_a(input: &'static str) -> usize {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let layers = input.trim().as_bytes().chunks(WIDTH * HEIGHT);

    let mut result = (std::usize::MAX, 0);
    for layer in layers {
        let mut counts = [0; 3];

        layer
            .iter()
            .map(|p| p - 48)
            .map(usize::from)
            .for_each(|i| counts[i] += 1);

        if counts[0] < result.0 {
            result = (counts[0], counts[1] * counts[2]);
        }
    }

    result.1
}

fn day_eight_b(input: &'static str) -> String {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let layers = input.trim().as_bytes().chunks(WIDTH * HEIGHT).rev();

    let mut canvas = vec![false; WIDTH * HEIGHT];

    for layer in layers {
        for (index, pixel) in layer.into_iter().enumerate() {
            canvas[index] = match pixel {
                b'0' => false,
                b'1' => true,
                _ => canvas[index],
            };
        }
    }

    let mut image = String::from("\n");
    for row in canvas.chunks(WIDTH) {
        for &pixel in row {
            image.push(if pixel { '#' } else { ' ' });
        }
        image.push('\n');
    }

    image
}

fn day_nine_a(input: &'static str) -> i64 {
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

fn day_nine_b(input: &'static str) -> i64 {
    let mut machine = intcode::Machine::new(input);
    let mut result = 0;
    loop {
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Input => machine.set_input(2),
            intcode::Interrupt::Output(value) => result = value,
        }
    }

    result
}
