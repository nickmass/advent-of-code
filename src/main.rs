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
    problem!(DayTen, 10, day_ten_a, day_ten_b);
    problem!(DayEleven, 11, day_eleven_a, day_eleven_b);
    problem!(DayTwelve, 12, day_twelve_a, day_twelve_b);

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
            if result.1.to_string().len() > 14 {
                println!("{:>2}-1:{:>14}{:>9}ms", result.0, ' ', result.3);
                println!("{}", result.1);
            } else {
                println!("{:>2}-1:{:>14}{:>9}ms", result.0, result.1, result.3);
            }
            if result.2.to_string().len() > 14 {
                println!("{:>2}-2:{:>14}{:>9}ms", result.0, ' ', result.4);
                println!("{}", result.2);
            } else {
                println!("{:>2}-2:{:>14}{:>9}ms", result.0, result.2, result.4);
            }
        }
        println!("Total Duration:{:>13}ms", duration.as_millis());
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

fn day_seven_a(input: &'static str) -> i32 {
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

fn day_seven_b(input: &'static str) -> i32 {
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
            image.push_str(if pixel { "##" } else { "  " });
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

#[derive(Debug)]
struct AsteroidGrid {
    occupied: FxHashSet<(i32, i32)>,
    angles: Vec<((i32, i32), f32)>,
    width: i32,
    height: i32,
}

impl AsteroidGrid {
    fn new<S: AsRef<str>>(input: S) -> Self {
        let input = input.as_ref();
        let lines = input.trim().split('\n').map(str::trim).enumerate();

        let mut height = 0;
        let mut width = 0;

        let mut occupied = FxHashSet::default();

        for (y, row) in lines {
            width = row.len() as i32;
            for (x, cell) in row.chars().enumerate() {
                if cell == '#' {
                    occupied.insert((x as i32, y as i32));
                }
            }
            height += 1;
        }

        let angles = create_angle_grid(width, height);

        AsteroidGrid {
            width,
            height,
            occupied,
            angles,
        }
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        self.occupied.contains(&(x, y))
    }

    fn remove(&mut self, x: i32, y: i32) -> bool {
        self.occupied.remove(&(x, y))
    }

    fn roid_on_angle(&self, x: i32, y: i32) -> usize {
        let mut count = 0;
        'outer: for &((xx, yy), _) in &self.angles {
            let (mut x_acc, mut y_acc) = (xx + x, yy + y);
            while self.in_bounds(x_acc, y_acc) {
                if self.contains(x_acc, y_acc) {
                    count += 1;
                    continue 'outer;
                }
                x_acc += xx;
                y_acc += yy;
            }
        }

        count
    }
}

fn create_angle_grid(width: i32, height: i32) -> Vec<((i32, i32), f32)> {
    use std::f32::consts::PI;
    let mut results = FxHashMap::default();

    for y in 1..=height {
        'outer: for x in 1..=width {
            let mut n = x.max(y);
            let angle = PI - (x as f32 / y as f32).atan();
            while n > 1 {
                if x % n == 0 && y % n == 0 {
                    results.insert((x / n, y / n), angle);
                    continue 'outer;
                }
                n -= 1;
            }

            results.insert((x, y), angle);
        }
    }

    let mut corners: Vec<_> = results.into_iter().collect();
    let corner_elements = corners.len();
    corners.reserve(corner_elements * 3);

    #[derive(Clone, Copy)]
    enum Corner {
        TopRight,
        BottomLeft,
        TopLeft,
    }

    for &(x, y, corner) in &[
        (1, -1, Corner::TopRight),
        (-1, 1, Corner::BottomLeft),
        (-1, -1, Corner::TopLeft),
    ] {
        for idx in 0..corner_elements {
            let ((xx, yy), angle) = corners[idx];
            let angle = PI - angle;
            let angle = match corner {
                Corner::TopRight => angle,
                Corner::BottomLeft => PI + angle,
                Corner::TopLeft => (PI * 2.0) - angle,
            };
            corners.push(((x * xx, y * yy), angle));
        }
    }

    let corner_offset = PI / 2.0;
    corners.push(((0, -1), 0.0 * corner_offset));
    corners.push(((1, 0), 1.0 * corner_offset));
    corners.push(((0, 1), 2.0 * corner_offset));
    corners.push(((-1, 0), 3.0 * corner_offset));

    corners
}

fn day_ten_a(input: &'static str) -> String {
    let grid = AsteroidGrid::new(input);

    let mut max_roids = 0;
    let mut target = (0, 0);
    for &(x, y) in grid.occupied.iter() {
        let roids = grid.roid_on_angle(x, y);
        if roids > max_roids {
            target = (x, y);
            max_roids = roids;
        }
    }

    format!("{} @ {},{}", max_roids, target.0, target.1)
}

fn day_ten_b(input: &'static str) -> i32 {
    const X: i32 = 37;
    const Y: i32 = 25;
    let mut grid = AsteroidGrid::new(input);

    grid.angles
        .sort_unstable_by(|(_, a1), (_, a2)| a1.partial_cmp(a2).unwrap());

    let mut loop_count = 1;
    let mut remove_count = 0;
    let angles = grid.angles.len();
    while loop_count < grid.height * grid.width {
        for idx in 0..angles {
            let ((xx, yy), _) = grid.angles[idx];
            let mut target_x = X + (xx * loop_count);
            let mut target_y = Y + (yy * loop_count);

            while grid.in_bounds(target_x, target_y) {
                if grid.remove(target_x, target_y) {
                    remove_count += 1;
                    if remove_count == 200 {
                        return target_x * 100 + target_y;
                    }
                    break;
                }

                target_x += xx;
                target_y += yy;
            }
        }
        loop_count += 1;
    }

    panic!("Missed it");
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn do_move(self, turn_left: bool, point: &mut (i32, i32)) -> Self {
        match (self, turn_left) {
            (Direction::Up, true) | (Direction::Down, false) => {
                point.0 -= 1;
                Direction::Left
            }
            (Direction::Up, false) | (Direction::Down, true) => {
                point.0 += 1;
                Direction::Right
            }
            (Direction::Left, false) | (Direction::Right, true) => {
                point.1 -= 1;
                Direction::Up
            }
            (Direction::Left, true) | (Direction::Right, false) => {
                point.1 += 1;
                Direction::Down
            }
        }
    }
}

fn day_eleven_a(input: &'static str) -> usize {
    let mut machine = intcode::Machine::<i64, _>::new(input);
    let mut painted_spots = FxHashMap::default();
    let mut point = (0, 0);
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

fn day_eleven_b(input: &'static str) -> String {
    let mut machine = intcode::Machine::<i64, _>::new(input);
    let mut painted_spots = FxHashMap::default();
    let mut point = (0, 0);
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

        min_x = point.0.min(min_x);
        max_x = point.0.max(max_x);
        min_y = point.1.min(min_x);
        max_y = point.1.max(max_y);

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
            let color = *painted_spots.get(&(x, y)).unwrap_or(&0);
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point<T: Eq + PartialEq + std::hash::Hash> {
    x: T,
    y: T,
    z: T,
}

impl<T: std::ops::Sub<Output = T> + Eq + PartialEq + std::hash::Hash> std::ops::Sub for Point<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: std::ops::Add<Output = T> + Eq + PartialEq + std::hash::Hash> std::ops::Add for Point<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

fn day_twelve_a(input: &'static str) -> i64 {
    let mut points: Vec<_> = input
        .trim()
        .split('\n')
        .map(|l| {
            let mut s = String::new();
            let mut n = Vec::new();
            for c in l.chars() {
                if c.is_digit(10) || c == '-' {
                    s.push(c);
                } else if s.len() > 0 {
                    n.push(s.parse::<i64>().unwrap());
                    s.clear();
                }
            }
            Point {
                x: n[0],
                y: n[1],
                z: n[2],
            }
        })
        .collect();

    let mut velo: Vec<_> = (0..points.len())
        .map(|_| Point { x: 0, y: 0, z: 0 })
        .collect();
    for _ in 0..1000 {
        for (i, (&p, vel)) in points.iter().zip(velo.iter_mut()).enumerate() {
            for (j, &pp) in points.iter().enumerate() {
                if i == j {
                    continue;
                }

                vel.x -= if p.x > pp.x {
                    1
                } else if p.x < pp.x {
                    -1
                } else {
                    0
                };
                vel.y -= if p.y > pp.y {
                    1
                } else if p.y < pp.y {
                    -1
                } else {
                    0
                };
                vel.z -= if p.z > pp.z {
                    1
                } else if p.z < pp.z {
                    -1
                } else {
                    0
                };
            }
        }

        for (p, v) in points.iter_mut().zip(velo.iter()) {
            *p = *p + *v;
        }
    }

    points
        .iter()
        .map(|p| p.x.abs() + p.y.abs() + p.z.abs())
        .zip(velo.iter().map(|v| v.x.abs() + v.y.abs() + v.z.abs()))
        .map(|(p, v)| p * v)
        .sum::<i64>()
}

fn day_twelve_b(input: &'static str) -> usize {
    let points: Vec<_> = input
        .trim()
        .split('\n')
        .map(|l| {
            let mut s = String::new();
            let mut n = Vec::new();
            for c in l.chars() {
                if c.is_digit(10) || c == '-' {
                    s.push(c);
                } else if s.len() > 0 {
                    n.push(s.parse::<i64>().unwrap());
                    s.clear();
                }
            }
            Point {
                x: n[0],
                y: n[1],
                z: n[2],
            }
        })
        .collect();

    points.len()
}
