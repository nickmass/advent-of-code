use std::{cmp::Reverse, collections::BinaryHeap};

use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    solve_part_one::<100>(input)
}

pub fn part_two(input: &str) -> usize {
    solve_part_two::<100>(input)
}

fn solve_part_one<const LIMIT: u64>(input: &str) -> usize {
    Map::new(input).part_one::<LIMIT>()
}

fn solve_part_two<const LIMIT: u64>(input: &str) -> usize {
    Map::new(input).part_two::<LIMIT>()
}

struct Map {
    walls: Grid<bool>,
    costs: Grid<u64>,
}

impl Map {
    fn new(input: &str) -> Self {
        let input = input.trim().as_bytes();
        let mut cells = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 0;

        let mut end = None;

        for &b in input {
            if b == b'\n' {
                height += 1;
                width = 0;
                continue;
            }

            match b {
                b'E' => {
                    end = Some(Point(width as i32, height as i32));
                    cells.push(false);
                }
                b'#' => cells.push(true),
                b'.' | b'S' => cells.push(false),
                _ => unreachable!(),
            }

            width += 1;
        }

        height += 1;
        let end = end.unwrap();

        let walls = Grid {
            cells,
            width,
            height,
        };
        let costs = walls.costs(end);

        Self { walls, costs }
    }

    fn get(&self, point: Point) -> Option<bool> {
        self.walls.get(point)
    }

    fn near_skips(
        &self,
        origin: Point,
        max_distance: u16,
        min_savings: u64,
    ) -> impl Iterator<Item = Point> + '_ {
        let starting_cost = self.costs.get(origin).unwrap_or(0);
        let max_distance = max_distance as i32;
        let mut y = -max_distance;
        let mut x = -max_distance;

        std::iter::from_fn(move || loop {
            let offset = Point(x, y);
            x += 1;
            if x > max_distance {
                x = -max_distance;
                y += 1;
                if y > max_distance {
                    return None;
                }
            }

            let steps = offset.magnitude() as u64;
            if steps > max_distance as u64 {
                continue;
            }

            let point = offset + origin;

            if !self.get(point).unwrap_or(true) {
                let cost = self.costs.get(point).unwrap_or(u64::MAX);
                let savings = starting_cost.saturating_sub(cost).saturating_sub(steps);

                if savings >= min_savings {
                    return Some(point);
                }
            }
        })
    }

    fn part_one<const LIMIT: u64>(&self) -> usize {
        self.find_cheats::<LIMIT, 2>()
    }

    fn part_two<const LIMIT: u64>(&self) -> usize {
        self.find_cheats::<LIMIT, 20>()
    }

    fn find_cheats<const LIMIT: u64, const CHEAT_LEN: u16>(&self) -> usize {
        let mut cheats = 0;
        for y in 0..self.walls.height {
            for x in 0..self.walls.width {
                let x = x as i32;
                let y = y as i32;
                let s = Point(x, y);

                if self.get(s).unwrap_or(true) {
                    continue;
                }

                cheats += self.near_skips(s, CHEAT_LEN, LIMIT).count();
            }
        }

        cheats
    }
}

struct Grid<T: Copy> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid<T> {
    fn get(&self, Point(x, y): Point) -> Option<T> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        let ind = y * self.width + x;
        self.cells.get(ind).copied()
    }

    fn set(&mut self, Point(x, y): Point, cell: T) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }
        let ind = y * self.width + x;
        self.cells[ind] = cell;
    }
}

impl Grid<bool> {
    fn costs(&self, end: Point) -> Grid<u64> {
        let mut costs = Grid {
            width: self.width,
            height: self.height,
            cells: vec![u64::MAX; self.cells.len()],
        };

        let mut search = BinaryHeap::new();
        let mut visited = HashSet::new();
        search.push(Reverse((0, end)));
        while let Some(Reverse((cost, point))) = search.pop() {
            if !visited.insert(point) {
                continue;
            }

            costs.set(point, cost);

            let neighbors = Direction::all()
                .into_iter()
                .map(|dir| dir.offset() + point)
                .filter(|&p| !self.get(p).unwrap_or(true))
                .filter(|p| !visited.contains(p))
                .map(|p| (cost + 1, p))
                .map(Reverse);

            search.extend(neighbors);
        }

        costs
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(i32, i32);

impl Point {
    fn magnitude(&self) -> u32 {
        (self.0.abs() + self.1.abs()) as u32
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        Point(x, y)
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn offset(&self) -> Point {
        match self {
            Direction::Up => Point(0, -1),
            Direction::Down => Point(0, 1),
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
        }
    }
}

#[test]
fn test() {
    let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;

    assert_eq!(8, solve_part_one::<12>(input));
    assert_eq!(41, solve_part_two::<70>(input));
}
