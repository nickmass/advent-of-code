use std::collections::BinaryHeap;

use crate::HashSet;

pub fn part_one(input: &str) -> i32 {
    let map = Map::new(input);
    map.find_route::<1, 3>()
}

pub fn part_two(input: &str) -> i32 {
    let map = Map::new(input);
    map.find_route::<4, 10>()
}

struct Map {
    cells: Vec<i32>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut cells = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 0;

        for line in input.trim().lines() {
            if width < line.len() {
                width = line.len();
            }

            height += 1;

            for c in line.bytes() {
                let n = match c {
                    b'0'..=b'9' => c - b'0',
                    _ => unreachable!(),
                };

                cells.push(n as i32);
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<i32> {
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);

        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = y * self.width + x;
            self.cells.get(idx).copied()
        }
    }

    fn find_route<const MIN: u32, const MAX: u32>(&self) -> i32 {
        let cursor = Cursor::<MIN, MAX>::new();
        let end = (self.width as i32 - 1, self.height as i32 - 1);

        let mut visited = HashSet::new();
        let mut haystack = BinaryHeap::new();
        haystack.push(Sorted(0, cursor));

        while let Some(Sorted(cost, next)) = haystack.pop() {
            if !visited.insert(next) {
                continue;
            }

            if next.point == end && next.dir_moves >= MIN {
                return cost;
            }

            let moves = [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ]
            .iter()
            .filter_map(|&dir| next.make_move(dir))
            .filter_map(|cursor| {
                self.get(cursor.point)
                    .map(|cell_cost| Sorted(cell_cost + cost, cursor))
            });

            haystack.extend(moves);
        }

        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cursor<const MIN: u32, const MAX: u32> {
    point: (i32, i32),
    dir_moves: u32,
    last_move: Option<Direction>,
}

impl<const MIN: u32, const MAX: u32> Cursor<MIN, MAX> {
    fn new() -> Self {
        Cursor {
            point: (0, 0),
            dir_moves: 0,
            last_move: None,
        }
    }

    fn make_move(&self, dir: Direction) -> Option<Self> {
        let mut dir_moves = self.dir_moves;

        if let Some(last_move) = self.last_move {
            if dir == last_move.opposite() {
                return None;
            }
        }

        if Some(dir) == self.last_move {
            dir_moves += 1;

            if dir_moves > MAX {
                return None;
            }
        } else if dir_moves < MIN && self.last_move.is_some() {
            return None;
        } else {
            dir_moves = 1;
        }

        let (x, y) = self.point;

        let point = match dir {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
        };

        let cursor = Cursor {
            point,
            dir_moves,
            last_move: Some(dir),
        };

        Some(cursor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Sorted<T>(i32, T);

impl<T: PartialEq + Eq> std::cmp::Ord for Sorted<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: PartialEq + Eq> std::cmp::PartialOrd for Sorted<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test() {
    let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;

    assert_eq!(102, part_one(input));
    assert_eq!(94, part_two(input));

    let input = r#"111111111111
999999999991
999999999991
999999999991
999999999991
"#;
    assert_eq!(71, part_two(input));
}
