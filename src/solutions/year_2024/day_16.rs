use std::{cmp::Reverse, collections::BinaryHeap};

pub fn part_one(input: &str) -> u64 {
    let map = Map::new(input);
    map.min_cost().unwrap_or(0)
}

pub fn part_two(input: &str) -> usize {
    let map = Map::new(input);
    map.min_path_count()
}

struct Map {
    walls: Vec<bool>,
    width: usize,
    height: usize,
    start: (i32, i32),
    end: (i32, i32),
}

impl Map {
    fn new(input: &str) -> Self {
        let input = input.trim().as_bytes();
        let mut walls = Vec::with_capacity(input.len());
        let mut height = 0;
        let mut width = 0;
        let mut start = None;
        let mut end = None;

        for &b in input.iter() {
            let cell = match b {
                b'\n' => {
                    height += 1;
                    width = 0;
                    continue;
                }
                b'#' => true,
                b'.' => false,
                b'S' => {
                    start = Some((width as i32, height as i32));
                    false
                }
                b'E' => {
                    end = Some((width as i32, height as i32));
                    false
                }
                _ => unreachable!(),
            };
            walls.push(cell);

            width += 1;
        }

        height += 1;

        let start = start.unwrap();
        let end = end.unwrap();

        Self {
            walls,
            width,
            height,
            start,
            end,
        }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<bool> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        let ind = y * self.width + x;

        self.walls.get(ind).copied()
    }

    fn min_cost(&self) -> Option<u64> {
        let start = Cursor::new(self.start);
        let mut visited = crate::HashSet::new();
        let mut heap = ReverseHeap::new();
        heap.push(start);

        while let Some(next) = heap.pop() {
            if next.position == self.end {
                return Some(next.cost);
            }

            if visited.insert(next.position) {
                let paths = next
                    .options()
                    .into_iter()
                    .filter(|c| !self.get(c.position).unwrap_or(true));
                heap.extend(paths);
            }
        }

        None
    }

    fn min_path_count(&self) -> usize {
        let max_cost = self.min_cost().unwrap();

        let mut cost_to_end = crate::HashMap::new();
        let mut visited = crate::HashSet::new();
        let mut heap = ReverseHeap::new();
        heap.extend(Direction::all().map(|d| Cursor {
            cost: 0,
            position: self.end,
            direction: d,
        }));

        while let Some(next) = heap.pop() {
            if next.cost > max_cost {
                break;
            }

            cost_to_end.entry(next.position).or_insert(next.cost);

            if visited.insert((next.position, next.direction)) {
                let paths = next
                    .options()
                    .into_iter()
                    .filter(|c| !self.get(c.position).unwrap_or(true));
                heap.extend(paths);
            }
        }

        let start = Cursor::new(self.start);
        let mut on_path = crate::HashSet::new();
        let mut path = Vec::new();
        let mut stack = Vec::new();
        stack.push((0, start));

        while let Some((path_len, next)) = stack.pop() {
            path.truncate(path_len);
            path.push(next.position);

            if next.position == self.end {
                on_path.extend(path.iter().copied());
                continue;
            }

            let paths = next
                .options()
                .into_iter()
                .filter(|c| !self.get(c.position).unwrap_or(true))
                .filter(|c| {
                    let remaining_cost = cost_to_end.get(&c.position).copied().unwrap_or(max_cost);
                    c.cost + remaining_cost <= max_cost
                })
                .map(|c| (path_len + 1, c));
            stack.extend(paths);
        }

        on_path.len()
    }
}

struct ReverseHeap<T: Ord> {
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> ReverseHeap<T> {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.heap.push(Reverse(item));
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter().map(|c| Reverse(c));
        self.heap.extend(iter)
    }

    fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(c)| c)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cursor {
    cost: u64,
    position: (i32, i32),
    direction: Direction,
}

impl Cursor {
    fn new(position: (i32, i32)) -> Self {
        Self {
            cost: 0,
            position,
            direction: Direction::East,
        }
    }

    fn options(self) -> [Self; 4] {
        let [a, b, c] = self.rotations();
        [self.forward(), a.forward(), b.forward(), c.forward()]
    }

    fn forward(self) -> Self {
        Self {
            cost: self.cost + 1,
            position: self.direction.apply(self.position),
            direction: self.direction,
        }
    }

    fn rotations(self) -> [Self; 3] {
        let Self {
            cost,
            position,
            direction,
        } = self;
        let a = Self {
            cost: cost + 1000,
            position,
            direction: direction.clockwise(),
        };

        let b = Self {
            cost: cost + 1000,
            position,
            direction: direction.counter_clockwise(),
        };

        let c = Self {
            cost: cost + 2000,
            position,
            direction: direction.clockwise().clockwise(),
        };

        [a, b, c]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
    }

    fn apply(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x - 1, y),
            Direction::West => (x + 1, y),
        }
    }
    fn clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::South => Direction::East,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }
}

#[test]
fn test() {
    let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

    assert_eq!(7036, part_one(input));
    assert_eq!(45, part_two(input));

    let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#;

    assert_eq!(11048, part_one(input));
    assert_eq!(64, part_two(input));
}
