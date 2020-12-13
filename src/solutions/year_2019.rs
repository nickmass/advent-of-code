use ahash::{AHashMap as HashMap, AHashSet as HashSet};

use std::str::FromStr;

use crate::{solution, Solution};

mod day_3;
mod intcode;

pub fn days() -> Vec<Solution> {
    vec![
        solution!(1, day_one_a, day_one_b),
        solution!(2, day_two_a, day_two_b),
        solution!(3, day_three_a, day_three_b),
        solution!(4, day_four_a, day_four_b),
        solution!(5, day_five_a, day_five_b),
        solution!(6, day_six_a, day_six_b),
        solution!(7, day_seven_a, day_seven_b),
        solution!(8, day_eight_a, day_eight_b),
        solution!(9, day_nine_a, day_nine_b),
        solution!(10, day_ten_a, day_ten_b),
        solution!(11, day_eleven_a, day_eleven_b),
        solution!(12, day_twelve_a, day_twelve_b),
        solution!(13, day_thirteen_a, day_thirteen_b),
        solution!(14, day_fourteen_a, day_fourteen_b),
        solution!(15, day_fifteen_a, day_fifteen_b),
        solution!(16, day_sixteen_a, day_sixteen_b),
        solution!(17, day_seventeen_a, day_seventeen_b),
    ]
}

fn day_one_a(input: &str) -> i32 {
    input
        .split('\n')
        .map(i32::from_str)
        .filter_map(Result::ok)
        .map(|f| f / 3 - 2)
        .sum::<i32>()
}

fn day_one_b(input: &str) -> i32 {
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

fn day_two_a(input: &str) -> i32 {
    let mut machine = intcode::Machine::new(input);
    machine.poke(1, 12);
    machine.poke(2, 2);
    machine.run();
    machine.peek(0)
}

fn day_two_b(input: &str) -> i32 {
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

fn day_three_a(input: &str) -> i32 {
    let mut lines = input.trim().split('\n');

    let first_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(day_3::Movement::from_str)
        .filter_map(Result::ok);
    let second_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(day_3::Movement::from_str)
        .filter_map(Result::ok);

    let first_segments = day_3::LineIter::new(first_line);
    let second_segments: Vec<_> = day_3::LineIter::new(second_line).collect();

    let intersections = first_segments.filter_map(|(_, f)| {
        second_segments
            .iter()
            .filter_map(|(_, s)| f.crosses(s))
            .next()
    });

    intersections.map(|p| p.distance()).min().unwrap_or(0)
}

fn day_three_b(input: &str) -> i32 {
    let mut lines = input.trim().split('\n');

    let first_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(day_3::Movement::from_str)
        .filter_map(Result::ok);
    let second_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(day_3::Movement::from_str)
        .filter_map(Result::ok);

    let first_segments = day_3::LineIter::new(first_line);
    let second_segments: Vec<_> = day_3::LineIter::new(second_line)
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

fn day_four_a(input: &str) -> usize {
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

fn day_four_b(input: &str) -> usize {
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

fn day_five_a(input: &str) -> i32 {
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

fn day_five_b(input: &str) -> i32 {
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

fn day_six_a(input: &str) -> usize {
    use std::convert::TryInto;
    let lines = input.as_bytes().chunks(8);

    let mut orbits = HashMap::new();
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

fn day_six_b(input: &str) -> usize {
    use std::convert::TryInto;
    let lines = input.as_bytes().chunks(8);

    let mut orbits = HashMap::new();
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

fn day_seven_a(input: &str) -> i32 {
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

fn day_seven_b(input: &str) -> i32 {
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

fn day_eight_a(input: &str) -> usize {
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

fn day_eight_b(input: &str) -> String {
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

fn day_nine_a(input: &str) -> i64 {
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

fn day_nine_b(input: &str) -> i64 {
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
    occupied: HashSet<Point2<i32>>,
    angles: Vec<(Point2<i32>, f32)>,
    width: i32,
    height: i32,
}

impl AsteroidGrid {
    fn new<S: AsRef<str>>(input: S) -> Self {
        let input = input.as_ref();
        let lines = input.trim().split('\n').map(str::trim).enumerate();

        let mut height = 0;
        let mut width = 0;

        let mut occupied = HashSet::new();

        for (y, row) in lines {
            width = row.len() as i32;
            for (x, cell) in row.chars().enumerate() {
                if cell == '#' {
                    occupied.insert(Point2::new(x as i32, y as i32));
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

    fn in_bounds(&self, p: Point2<i32>) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width && p.y < self.height
    }

    fn contains(&self, p: Point2<i32>) -> bool {
        self.occupied.contains(&p)
    }

    fn remove(&mut self, p: Point2<i32>) -> bool {
        self.occupied.remove(&p)
    }

    fn roid_on_angle(&self, p: Point2<i32>) -> usize {
        let mut count = 0;
        'outer: for &(pp, _) in &self.angles {
            let mut p_acc = pp + p;
            while self.in_bounds(p_acc) {
                if self.contains(p_acc) {
                    count += 1;
                    continue 'outer;
                }
                p_acc += pp;
            }
        }

        count
    }
}

fn create_angle_grid(width: i32, height: i32) -> Vec<(Point2<i32>, f32)> {
    use std::f32::consts::PI;
    let mut results = HashMap::new();

    for y in 1..=height {
        for x in 1..=width {
            let angle = PI - (x as f32 / y as f32).atan();
            let gcd = x.gcd(y);
            results.insert(Point2::new(x / gcd, y / gcd), angle);
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

    for &(p, corner) in &[
        (Point2::new(1, -1), Corner::TopRight),
        (Point2::new(-1, 1), Corner::BottomLeft),
        (Point2::new(-1, -1), Corner::TopLeft),
    ] {
        for idx in 0..corner_elements {
            let (pp, angle) = corners[idx];
            let angle = PI - angle;
            let angle = match corner {
                Corner::TopRight => angle,
                Corner::BottomLeft => PI + angle,
                Corner::TopLeft => (PI * 2.0) - angle,
            };
            corners.push((p * pp, angle));
        }
    }

    let corner_offset = PI / 2.0;
    corners.push((Point2::new(0, -1), 0.0 * corner_offset));
    corners.push((Point2::new(1, 0), 1.0 * corner_offset));
    corners.push((Point2::new(0, 1), 2.0 * corner_offset));
    corners.push((Point2::new(-1, 0), 3.0 * corner_offset));

    corners
}

fn day_ten_a(input: &str) -> DayTenResult {
    let grid = AsteroidGrid::new(input);

    let mut max_roids = 0;
    let mut target = Point2::new(0, 0);
    for &p in grid.occupied.iter() {
        let roids = grid.roid_on_angle(p);
        if roids > max_roids {
            target = p;
            max_roids = roids;
        }
    }

    DayTenResult { max_roids, target }
}

struct DayTenResult {
    max_roids: usize,
    target: Point2<i32>,
}

impl std::fmt::Display for DayTenResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.max_roids)
    }
}

fn day_ten_b(input: &str) -> i32 {
    let DayTenResult { target, .. } = day_ten_a(input);
    let mut grid = AsteroidGrid::new(input);

    grid.angles
        .sort_unstable_by(|(_, a1), (_, a2)| a1.partial_cmp(a2).unwrap());

    let mut loop_count = 1;
    let mut remove_count = 0;
    let angles = grid.angles.len();
    while loop_count < grid.height * grid.width {
        for idx in 0..angles {
            let (pp, _) = grid.angles[idx];
            let mut target = target + (pp * loop_count);

            while grid.in_bounds(target) {
                if grid.remove(target) {
                    remove_count += 1;
                    if remove_count == 200 {
                        return target.x * 100 + target.y;
                    }
                    break;
                }

                target += pp;
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

fn day_eleven_a(input: &str) -> usize {
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

fn day_eleven_b(input: &str) -> String {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point2<T> {
    x: T,
    y: T,
}

macro_rules! impl_point_op {
    ($name:ident, $func:ident, $op:tt) => {
        impl<T: std::ops::$name<Output = T>> std::ops::$name for Point3<T> {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self::Output {
                Point3 {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }

        impl<T: std::ops::$name<Output = T> + Copy> std::ops::$name<T> for Point3<T> {
            type Output = Self;
            fn $func(self, rhs: T) -> Self::Output {
                Point3 {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                    z: self.z $op rhs,
                }
            }
        }

        impl<T: std::ops::$name<Output = T>> std::ops::$name for Point2<T> {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self::Output {
                Point2 {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                }
            }
        }

        impl<T: std::ops::$name<Output = T> + Copy> std::ops::$name<T> for Point2<T> {
            type Output = Self;
            fn $func(self, rhs: T) -> Self::Output {
                Point2 {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                }
            }
        }
    };
}

macro_rules! impl_point_op_assign {
    ($name:ident, $func:ident, $op:tt) => {
        impl<T: std::ops::$name> std::ops::$name for Point3<T> {
            fn $func(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }

        impl<T: std::ops::$name + Copy> std::ops::$name<T> for Point3<T> {
            fn $func(&mut self, rhs: T) {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
            }
        }

        impl<T: std::ops::$name> std::ops::$name for Point2<T> {
            fn $func(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }

        impl<T: std::ops::$name + Copy> std::ops::$name<T> for Point2<T> {
            fn $func(&mut self, rhs: T) {
                self.x $op rhs;
                self.y $op rhs;
            }
        }
    };
}

impl_point_op!(Sub, sub, -);
impl_point_op!(Add, add, +);
impl_point_op!(Mul, mul, *);
impl_point_op!(Div, div, /);
impl_point_op!(Rem, rem, %);

impl_point_op_assign!(SubAssign, sub_assign, -=);
impl_point_op_assign!(AddAssign, add_assign, +=);
impl_point_op_assign!(MulAssign, mul_assign, *=);
impl_point_op_assign!(DivAssign, div_assign, /=);
impl_point_op_assign!(RemAssign, rem_assign, %=);

impl<T> Point3<T> {
    const fn new(x: T, y: T, z: T) -> Self {
        Point3 { x, y, z }
    }
}

impl<T> Point2<T> {
    const fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Point3<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Point2<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

trait Gcd<Rhs = Self> {
    type Output;
    fn gcd(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_gcd {
    ($($type:ty )+) => {
        $(
            impl Gcd for $type {
                type Output = Self;
                fn gcd(self, rhs: Self) -> Self {
                    let mut n = self.min(rhs);
                    while n > 1 {
                        if self % n == 0 && rhs % n == 0 {
                            return n;
                        }
                        n -= 1;
                    }

                    return 1;
                }
            }
        )+
    };
}

impl_gcd!(i32 u32 i64 u64 i128 u128 isize usize);

fn day_twelve_a(input: &str) -> i64 {
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
            Point3::new(n[0], n[1], n[2])
        })
        .collect();

    let mut velo: Vec<_> = (0..points.len()).map(|_| Point3::new(0, 0, 0)).collect();
    for _ in 0..1000 {
        for (i, (&p, vel)) in points.iter().zip(velo.iter_mut()).enumerate() {
            for (j, pp) in points.iter().enumerate() {
                if i == j {
                    continue;
                }

                vel.x -= match p.x.cmp(&pp.x) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
                vel.y -= match p.y.cmp(&pp.y) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
                vel.z -= match p.z.cmp(&pp.z) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
            }
        }

        for (p, &v) in points.iter_mut().zip(velo.iter()) {
            *p += v;
        }
    }

    points
        .iter()
        .map(|p| p.x.abs() + p.y.abs() + p.z.abs())
        .zip(velo.iter().map(|v| v.x.abs() + v.y.abs() + v.z.abs()))
        .map(|(p, v)| p * v)
        .sum::<i64>()
}

fn sequence_length(initial_points: &[i64]) -> usize {
    let mut points = initial_points.to_vec();
    let mut velo: Vec<_> = points.iter().map(|_| 0).collect();

    let mut count = 0;

    let mut done = false;
    while !done {
        for (i, (&p, vel)) in points.iter().zip(velo.iter_mut()).enumerate() {
            for (j, pp) in points.iter().enumerate() {
                if i == j {
                    continue;
                }

                *vel -= match p.cmp(pp) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
            }
        }

        done = true;
        for (p, &v) in points.iter_mut().zip(velo.iter()) {
            *p += v;
            done = done && v == 0
        }
        count += 1;
    }

    count * 2
}

fn day_twelve_b(input: &str) -> usize {
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
            Point3::new(n[0], n[1], n[2])
        })
        .collect();

    let x_coords: Vec<_> = points.iter().map(|&p| p.x).collect();
    let y_coords: Vec<_> = points.iter().map(|&p| p.y).collect();
    let z_coords: Vec<_> = points.iter().map(|&p| p.z).collect();

    let x_handle = std::thread::spawn(move || sequence_length(x_coords.as_slice()));
    let y_handle = std::thread::spawn(move || sequence_length(y_coords.as_slice()));
    let z_handle = std::thread::spawn(move || sequence_length(z_coords.as_slice()));

    let x_length = x_handle.join().unwrap();
    let y_length = y_handle.join().unwrap();
    let z_length = z_handle.join().unwrap();

    let xy_gcd = x_length.gcd(y_length);
    let xy_lcm = (x_length * y_length) / xy_gcd;
    let xyz_gcd = xy_lcm.gcd(z_length);
    (xy_lcm * z_length) / xyz_gcd
}

fn day_thirteen_a(input: &str) -> i64 {
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

fn day_thirteen_b(input: &str) -> i64 {
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

fn day_fourteen_a(input: &str) -> u64 {
    let reactions: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let mut sides = l.split(" => ");
            let input = sides.next().unwrap();
            let output = sides.next().unwrap();

            let mut output = output.split_whitespace();
            let output_n: u64 = output.next().unwrap().parse().unwrap();
            let output_chem = output.next().unwrap();

            let mut inputs = Vec::new();

            for input in input.split(", ") {
                let mut input = input.split_whitespace();
                let input_n: u64 = input.next().unwrap().parse().unwrap();
                let input_chem = input.next().unwrap();

                inputs.push((input_chem, input_n))
            }

            (output_chem, (output_n, inputs))
        })
        .collect();

    let mut inventory = HashMap::new();
    fill_request(&reactions, &mut inventory, "FUEL", 1)
}

fn fill_request<'a>(
    reactions: &HashMap<&'a str, (u64, Vec<(&'a str, u64)>)>,
    inventory: &mut HashMap<&'a str, u64>,
    chem: &'a str,
    count: u64,
) -> u64 {
    let (n, inputs) = reactions.get(chem).expect("chem must exist");

    let needs = if count % n == 0 {
        count / n
    } else {
        (count / n) + 1
    };

    let mut ore_count = 0;

    for (in_chem, in_count) in inputs {
        let required_amount = needs * in_count;

        if in_chem == &"ORE" {
            ore_count += required_amount;
            continue;
        }

        let in_chem_current = inventory.remove(in_chem).unwrap_or(0);

        let new_amount = if required_amount > in_chem_current {
            ore_count += fill_request(
                reactions,
                inventory,
                in_chem,
                required_amount - in_chem_current,
            );

            let in_chem_new = inventory.remove(in_chem).unwrap_or(0);
            (in_chem_current + in_chem_new) - required_amount
        } else {
            in_chem_current - required_amount
        };

        inventory.insert(in_chem, new_amount);
    }

    inventory.insert(chem, needs * n);

    ore_count
}

fn day_fourteen_b(input: &str) -> u64 {
    let reactions: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let mut sides = l.split(" => ");
            let input = sides.next().unwrap();
            let output = sides.next().unwrap();

            let mut output = output.split_whitespace();
            let output_n: u64 = output.next().unwrap().parse().unwrap();
            let output_chem = output.next().unwrap();

            let mut inputs = Vec::new();

            for input in input.split(", ") {
                let mut input = input.split_whitespace();
                let input_n: u64 = input.next().unwrap().parse().unwrap();
                let input_chem = input.next().unwrap();

                inputs.push((input_chem, input_n))
            }

            (output_chem, (output_n, inputs))
        })
        .collect();

    let n = 1_000_000_000_000;
    let mut inventory = HashMap::new();
    let ore_per_fuel_max = fill_request(&reactions, &mut inventory, "FUEL", 1);
    let mut fuel_min = n / ore_per_fuel_max;
    let mut fuel_max;

    loop {
        inventory.clear();
        let ore_count = fill_request(&reactions, &mut inventory, "FUEL", fuel_min);
        if ore_count < n {
            fuel_min *= 2;
        } else {
            fuel_max = fuel_min;
            fuel_min /= 2;
            break;
        }
    }

    loop {
        if fuel_max - fuel_min <= 1 {
            return fuel_min;
        }

        inventory.clear();
        let next_attempt = fuel_min + (fuel_max - fuel_min) / 2;
        let ore_count = fill_request(&reactions, &mut inventory, "FUEL", next_attempt);
        if ore_count < n {
            fuel_min = next_attempt;
        } else {
            fuel_max = next_attempt;
        }
    }
}
#[derive(Hash, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Oxygen,
    Unknown,
}

struct RobotMap {
    map: HashMap<Point2<i32>, Tile>,
    min: Point2<i32>,
    max: Point2<i32>,
}

fn point_neighbors(point: Point2<i32>) -> impl Iterator<Item = Point2<i32>> {
    let mut count = 0;
    std::iter::from_fn(move || {
        let res = match count {
            0 => Some(Point2::new(point.x, point.y + 1)),
            1 => Some(Point2::new(point.x, point.y - 1)),
            2 => Some(Point2::new(point.x + 1, point.y)),
            3 => Some(Point2::new(point.x - 1, point.y)),
            _ => None,
        };
        count += 1;
        res
    })
}

impl RobotMap {
    fn new() -> Self {
        RobotMap {
            map: HashMap::new(),
            min: Point2::new(0, 0),
            max: Point2::new(0, 0),
        }
    }

    fn get(&self, point: Point2<i32>) -> Tile {
        *self.map.get(&point).unwrap_or(&Tile::Unknown)
    }

    fn set(&mut self, point: Point2<i32>, tile: Tile) {
        match tile {
            Tile::Unknown => (),
            _ => {
                self.min.x = self.min.x.min(point.x);
                self.min.y = self.min.y.min(point.y);
                self.max.x = self.max.x.max(point.x);
                self.max.y = self.max.y.max(point.y);
            }
        }
        self.map.insert(point, tile);
    }

    fn create_distances(
        &self,
        point: Point2<i32>,
        early_escape: Option<Tile>,
    ) -> HashMap<Point2<i32>, i32> {
        let mut current_point = point;

        let mut unvisited: HashSet<_> = self
            .map
            .iter()
            .filter(|(_p, t)| **t != Tile::Wall)
            .map(|(k, _v)| k.clone())
            .collect();
        let mut unknowns = HashSet::new();
        for node in unvisited.iter().flat_map(|p| point_neighbors(*p)) {
            match self.get(node) {
                Tile::Unknown => {
                    unknowns.insert(node);
                }
                _ => (),
            }
        }

        for unknown in unknowns {
            unvisited.insert(unknown);
        }

        let mut distances: HashMap<_, _> =
            unvisited.iter().map(|k| (k.clone(), i32::MAX)).collect();

        distances.get_mut(&current_point).map(|d| *d = 0);

        loop {
            let current_distance = distances.get(&current_point).cloned().unwrap();
            for neighbor in point_neighbors(current_point) {
                if !unvisited.contains(&neighbor) {
                    continue;
                }
                match distances.get_mut(&neighbor) {
                    None => continue,
                    Some(dist) => {
                        let tentative_dist = current_distance + 1;
                        *dist = (*dist).min(tentative_dist);
                    }
                }
            }
            unvisited.remove(&current_point);
            if unvisited.len() == 0 {
                break;
            } else if let Some(target_tile) = early_escape {
                if self.get(current_point) == target_tile {
                    break;
                }
            }

            current_point = unvisited
                .iter()
                .flat_map(|p| distances.get(p).map(|d| (p, d)))
                .min_by_key(|(_p, d)| *d)
                .map(|(p, _d)| p.clone())
                .unwrap();
        }

        distances
    }

    fn route_nearest_tile(
        &self,
        point: Point2<i32>,
        target_tile: Tile,
    ) -> Option<Vec<Point2<i32>>> {
        let distances = self.create_distances(point, Some(target_tile));

        let (dest, _dist) = distances
            .iter()
            .filter(|(p, _d)| self.get(**p) == target_tile)
            .min_by_key(|(_p, d)| *d)?;

        let mut route = Vec::new();

        let mut next: Point2<i32> = *dest;
        loop {
            route.push(next.clone());
            next = point_neighbors(next)
                .filter_map(|p| distances.get(&p).map(|d| (p, *d)))
                .min_by_key(|(_p, d)| *d)
                .map(|(p, _d)| p.clone())
                .unwrap();

            if next == point {
                break;
            }
        }

        Some(route)
    }

    fn max_distance_from_point(&self, point: Point2<i32>, target_tile: Tile) -> Option<i32> {
        let distances = self.create_distances(point, None);

        let (_dest, dist) = distances
            .iter()
            .filter(|(p, _d)| self.get(**p) == target_tile)
            .max_by_key(|(_p, d)| *d)?;

        Some(*dist)
    }

    #[allow(dead_code)]
    fn print(&self, position: Point2<i32>) {
        for y in -50..50 {
            for x in -50..50 {
                let point = Point2::new(x, 0 - y);
                let tile = if x == 0 && y == 0 {
                    'S'
                } else if point == position {
                    'D'
                } else {
                    match self.get(point) {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Oxygen => 'O',
                        Tile::Unknown => ' ',
                    }
                };

                print!("{}", tile);
            }
            println!();
        }
    }
}

fn day_fifteen_a(input: &str) -> u64 {
    let mut machine = intcode::Machine::new(input);
    let mut position = Point2::new(0, 0);
    let mut target_pos = position;

    let mut current_path = vec![];
    let mut map = RobotMap::new();
    map.set(position, Tile::Empty);

    loop {
        match machine.run() {
            intcode::Interrupt::Output(0) => {
                map.set(target_pos, Tile::Wall);
            }
            intcode::Interrupt::Output(1) => {
                map.set(target_pos, Tile::Empty);
                position = target_pos;
            }
            intcode::Interrupt::Output(2) => {
                map.set(target_pos, Tile::Oxygen);
                position = target_pos;
            }
            intcode::Interrupt::Input => {
                if current_path.len() == 0 {
                    current_path = match map.route_nearest_tile(position, Tile::Unknown) {
                        Some(route) => route,
                        None => break,
                    };
                }

                target_pos = current_path.pop().unwrap();

                let dir = if target_pos.y > position.y {
                    1
                } else if target_pos.y < position.y {
                    2
                } else if target_pos.x < position.x {
                    3
                } else if target_pos.x > position.x {
                    4
                } else {
                    unreachable!("invalid route")
                };
                machine.set_input(dir);
            }
            intcode::Interrupt::Halt => {
                break;
            }
            _ => unreachable!(),
        }
    }

    map.route_nearest_tile(Point2::new(0, 0), Tile::Oxygen)
        .map(|r| r.len())
        .unwrap_or(0) as u64
}

fn day_fifteen_b(input: &str) -> u64 {
    let mut machine = intcode::Machine::new(input);
    let mut position = Point2::new(0, 0);
    let mut target_pos = position;

    let mut current_path = vec![];
    let mut map = RobotMap::new();
    map.set(position, Tile::Empty);

    let mut oxy_point = Point2::new(0, 0);

    loop {
        match machine.run() {
            intcode::Interrupt::Output(0) => {
                map.set(target_pos, Tile::Wall);
            }
            intcode::Interrupt::Output(1) => {
                map.set(target_pos, Tile::Empty);
                position = target_pos;
            }
            intcode::Interrupt::Output(2) => {
                map.set(target_pos, Tile::Oxygen);
                oxy_point = target_pos;
                position = target_pos;
            }
            intcode::Interrupt::Input => {
                if current_path.len() == 0 {
                    current_path = match map.route_nearest_tile(position, Tile::Unknown) {
                        Some(route) => route,
                        None => break,
                    };
                }

                target_pos = current_path.pop().unwrap();

                let dir = if target_pos.y > position.y {
                    1
                } else if target_pos.y < position.y {
                    2
                } else if target_pos.x < position.x {
                    3
                } else if target_pos.x > position.x {
                    4
                } else {
                    unreachable!("invalid route")
                };
                machine.set_input(dir);
            }
            intcode::Interrupt::Halt => {
                break;
            }
            _ => unreachable!(),
        }
    }

    map.max_distance_from_point(oxy_point, Tile::Empty)
        .unwrap_or(0) as u64
}

fn day_sixteen_a(input: &str) -> String {
    let mut signal: Vec<_> = input
        .chars()
        .filter_map(|c| {
            if c >= '0' && c <= '9' {
                Some((c as u32 - '0' as u32) as u8)
            } else {
                None
            }
        })
        .collect();

    let mut next_signal = Vec::with_capacity(signal.len());
    let patterns: Vec<Vec<_>> = (0..signal.len())
        .map(|n| {
            (n..signal.len() + 1)
                .map(|i| pattern_at(i, n + 1))
                .collect()
        })
        .collect();
    for _ in 0..100 {
        next_signal.clear();

        for position in 0..signal.len() {
            let sum: i32 = signal[position..]
                .iter()
                .copied()
                .zip(patterns[position].iter())
                .map(|(d, p)| d as i32 * p)
                .sum();

            next_signal.push((sum.abs() % 10) as u8);
        }

        std::mem::swap(&mut signal, &mut next_signal);
    }

    signal
        .into_iter()
        .take(8)
        .map(|n| (n + '0' as u8) as char)
        .collect()
}

fn pattern_at(index: usize, dupe_count: usize) -> i32 {
    let pattern = [0, 1, 0, -1];
    let true_idx = ((index + 1) / dupe_count) % 4;

    pattern[true_idx]
}

fn day_sixteen_b(input: &str) -> String {
    let input_len = input.trim().len();
    let repeat = 10000;
    let total_len = input_len * repeat;

    let input = input
        .chars()
        .filter_map(|c| {
            if c >= '0' && c <= '9' {
                Some((c as u32 - '0' as u32) as u8)
            } else {
                None
            }
        })
        .cycle()
        .take(total_len);

    let offset = input.clone().take(7).enumerate().fold(0, |acc, (idx, n)| {
        acc + (n as usize * 10usize.pow(6 - idx as u32))
    });

    // Would run into performance issues if the offset ended having leading zeros,
    // would even give incorrect results if offset < (total_len / 2)
    assert!(offset > total_len / 2);

    let mut signal: Vec<_> = input.skip(offset).collect();

    for _phase in 0..100 {
        let mut sum = 0;
        for n in signal.iter_mut().rev() {
            sum += *n as i32;
            *n = (sum % 10) as u8
        }
    }

    signal
        .into_iter()
        .take(8)
        .map(|n| (n + '0' as u8) as char)
        .collect()
}

fn day_seventeen_a(input: &str) -> i32 {
    let mut machine = intcode::Machine::new(input);
    let mut map = HashMap::new();
    let mut col = 0;
    let mut row = 0;
    let mut max_col = 0;
    let mut max_row = 0;
    loop {
        match machine.run() {
            intcode::Interrupt::Input => {}
            intcode::Interrupt::Output(10) => {
                max_col = col.max(max_col);
                col = 0;
                row += 1;
                max_row = row.max(max_row);
            }
            intcode::Interrupt::Output(c) => {
                map.insert((row, col), c);
                col += 1;
            }
            intcode::Interrupt::Halt => {
                break;
            }
        }
    }

    let mut sum = 0;

    for (&(row, col), &c) in map.iter() {
        if c == 35 {
            let top = (row - 1, col);
            let bottom = (row + 1, col);
            let left = (row, col - 1);
            let right = (row, col + 1);

            let top = map.get(&top);
            let bottom = map.get(&bottom);
            let left = map.get(&left);
            let right = map.get(&right);

            match top.zip(bottom).zip(left).zip(right) {
                Some((((35, 35), 35), 35)) => sum += row * col,
                _ => (),
            }
        }
    }

    sum
}

#[derive(Copy, Clone, Debug)]
enum Robot {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Move {
    Left,
    Right,
    Forward(u8),
    Subroutine(u8),
}

impl Move {
    fn is_sub(&self) -> bool {
        match *self {
            Move::Subroutine(_) => true,
            _ => false,
        }
    }
}

fn find_route(map: &HashMap<Point2<i32>, u8>, robot: &(Point2<i32>, Robot)) -> String {
    let mut point = robot.0;
    let mut dir = robot.1;

    let mut route = Vec::new();

    let off_left = Point2::new(-1, 0);
    let off_right = Point2::new(1, 0);
    let off_up = Point2::new(0, -1);
    let off_down = Point2::new(0, 1);

    let is_wall = |point, offset| {
        map.get(&(point + offset))
            .map(|c| *c == b'#')
            .unwrap_or(false)
    };

    loop {
        let left = is_wall(point, off_left);
        let right = is_wall(point, off_right);
        let up = is_wall(point, off_up);
        let down = is_wall(point, off_down);

        match (dir, left, right, up, down) {
            (Robot::Up, true, _, _, _) => {
                route.push(Move::Left);
                dir = Robot::Left;
            }
            (Robot::Up, _, true, _, _) => {
                route.push(Move::Right);
                dir = Robot::Right;
            }
            (Robot::Down, true, _, _, _) => {
                route.push(Move::Right);
                dir = Robot::Left;
            }
            (Robot::Down, _, true, _, _) => {
                route.push(Move::Left);
                dir = Robot::Right;
            }
            (Robot::Left, _, _, true, _) => {
                route.push(Move::Right);
                dir = Robot::Up;
            }
            (Robot::Left, _, _, _, true) => {
                route.push(Move::Left);
                dir = Robot::Down;
            }
            (Robot::Right, _, _, true, _) => {
                route.push(Move::Left);
                dir = Robot::Up;
            }
            (Robot::Right, _, _, _, true) => {
                route.push(Move::Right);
                dir = Robot::Down;
            }
            _ => break,
        }

        let move_offset = match dir {
            Robot::Left => off_left,
            Robot::Right => off_right,
            Robot::Up => off_up,
            Robot::Down => off_down,
        };

        let mut move_count = 0;
        while is_wall(point, move_offset) {
            move_count += 1;
            point = point + move_offset;
        }

        route.push(Move::Forward(move_count));
    }

    let remove_sub = |route: &mut Vec<Move>, subroutine: u8, len: usize| {
        let start = route.iter().position(|m| !m.is_sub()).unwrap_or(0);
        let start_end = start + len;
        if start_end > route.len() {
            return;
        }
        if route[start..start_end].iter().any(|m| m.is_sub()) {
            return;
        }
        let mut search_idx = start_end;
        while search_idx + len <= route.len() {
            let search_end = search_idx + len;
            if route[start..start_end] == route[search_idx..search_end] {
                for _ in search_idx..search_end {
                    route.remove(search_idx);
                }
                route.insert(search_idx, Move::Subroutine(subroutine));
            } else {
                search_idx += 1;
            }
        }

        for _ in start..start_end {
            route.remove(start);
        }
        route.insert(start, Move::Subroutine(subroutine));
    };

    let mut answer = None;
    'outer: for a in 1..10 {
        let mut route = route.clone();
        remove_sub(&mut route, 0, a);

        for b in 1..10 {
            let mut route = route.clone();
            remove_sub(&mut route, 1, b);

            for c in 1..10 {
                let mut route = route.clone();
                remove_sub(&mut route, 2, c);

                if route.iter().all(|m| m.is_sub()) {
                    answer = Some((a, b, c));
                    break 'outer;
                }
            }
        }
    }

    if let Some((a_len, b_len, c_len)) = answer {
        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();
        let mut main = String::new();

        for m in route.iter().filter(|m| !m.is_sub()).take(a_len) {
            if a.len() != 0 {
                a.push(',');
            }
            match m {
                Move::Right => a.push('R'),
                Move::Left => a.push('L'),
                Move::Forward(n) => a.push_str(n.to_string().as_str()),
                _ => (),
            }
        }
        remove_sub(&mut route, 0, a_len);

        for m in route.iter().filter(|m| !m.is_sub()).take(b_len) {
            if b.len() != 0 {
                b.push(',');
            }
            match m {
                Move::Right => b.push('R'),
                Move::Left => b.push('L'),
                Move::Forward(n) => b.push_str(n.to_string().as_str()),
                _ => (),
            }
        }
        remove_sub(&mut route, 1, b_len);

        for m in route.iter().filter(|m| !m.is_sub()).take(c_len) {
            if c.len() != 0 {
                c.push(',');
            }
            match m {
                Move::Right => c.push('R'),
                Move::Left => c.push('L'),
                Move::Forward(n) => c.push_str(n.to_string().as_str()),
                _ => (),
            }
        }
        remove_sub(&mut route, 2, c_len);

        for m in route {
            if main.len() != 0 {
                main.push(',');
            }
            match m {
                Move::Subroutine(0) => main.push('A'),
                Move::Subroutine(1) => main.push('B'),
                Move::Subroutine(2) => main.push('C'),
                _ => (),
            }
        }
        format!("{}\n{}\n{}\n{}\n", main, a, b, c)
    } else {
        panic!("route not found")
    }
}

fn day_seventeen_b(input: &str) -> i32 {
    let mut machine = intcode::Machine::new(input);
    machine.poke(0, 2);

    let mut input: Vec<char> = "\nn".chars().collect();

    let mut map = HashMap::new();
    let mut col = 0;
    let mut row = 0;
    let mut max_col = 0;
    let mut max_row = 0;
    let mut running = false;
    let mut mapping = true;

    let mut robot = (Point2::new(0, 0), Robot::Up);

    /*
    let print_map = |map: &HashMap<Point2<i32>, u8>, max_row, max_col| {
        for row in 0..max_row {
            for col in 0..max_col {
                if let Some(&c) = map.get(&Point2::new(col, row)) {
                    print!("{}", c as char);
                }
            }
            println!();
        }
    };
    */

    loop {
        match machine.run() {
            intcode::Interrupt::Output(n) if n > 0x7f => {
                return n;
            }
            intcode::Interrupt::Output(10) if mapping => {
                max_col = col.max(max_col);
                if col == 0 {
                    mapping = false;
                    row = 0;
                    let route = find_route(&map, &robot);
                    for c in route.chars().rev() {
                        input.push(c);
                    }
                //print_map(&map, max_row, max_col);
                } else {
                    row += 1;
                    max_row = row.max(max_row);
                }
                col = 0;
            }
            intcode::Interrupt::Output(c)
                if mapping
                    && (c as u8 == b'v'
                        || c as u8 == b'^'
                        || c as u8 == b'<'
                        || c as u8 == b'>') =>
            {
                let dir = match c as u8 {
                    b'v' => Robot::Down,
                    b'^' => Robot::Up,
                    b'<' => Robot::Left,
                    b'>' => Robot::Right,
                    _ => unreachable!(),
                };
                robot = (Point2::new(col, row), dir);
                map.insert(Point2::new(col, row), b'#');
                col += 1;
            }
            intcode::Interrupt::Output(c) if mapping => {
                map.insert(Point2::new(col, row), c as u8);
                col += 1;
            }
            intcode::Interrupt::Output(10) if running => {
                if col == 0 {
                    //println!();
                } else {
                    col = 0;
                    row += 1;

                    if row == max_row {
                        row = 0;
                        //print_map(&map, max_row, max_col);
                    }
                }
            }
            intcode::Interrupt::Output(c) if running => {
                map.insert(Point2::new(col, row), c as u8);
                col += 1;
            }
            intcode::Interrupt::Input => {
                let inp = input.pop();
                //print!("{}", inp.unwrap_or('\n'));
                machine.set_input(inp.unwrap_or('\n') as i32);
            }
            intcode::Interrupt::Output(10) => {
                if input.len() == 0 {
                    running = true;
                }
                //println!();
            }
            intcode::Interrupt::Output(_c) => {
                //print!("{}", c as u8 as char);
            }
            intcode::Interrupt::Halt => break,
        }
    }

    panic!("robot failed to find exit")
}
