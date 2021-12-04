use crate::{HashMap, HashSet};

use super::{intcode, Point2};

pub fn part_one(input: &str) -> u64 {
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

pub fn part_two(input: &str) -> u64 {
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
