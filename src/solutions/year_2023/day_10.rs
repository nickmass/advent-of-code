use crate::HashSet;

pub fn part_one(input: &str) -> i32 {
    let map = Map::new(input);
    map.max_route()
}

pub fn part_two(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.count_internal_grounds()
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    start: (i32, i32),
}

impl Map {
    fn new(input: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut cells = Vec::new();
        let mut start = None;
        for l in input.lines() {
            if l.trim().is_empty() {
                continue;
            }
            width = 0;
            for c in l.chars() {
                let cell = match c {
                    'S' => {
                        start = Some((width as i32, height as i32));
                        Cell::Start
                    }
                    '.' => Cell::Ground,
                    '|' => Cell::Pipe(Pipe::Vertical),
                    '-' => Cell::Pipe(Pipe::Horizontal),
                    'L' => Cell::Pipe(Pipe::NorthEast),
                    'J' => Cell::Pipe(Pipe::NorthWest),
                    '7' => Cell::Pipe(Pipe::SouthWest),
                    'F' => Cell::Pipe(Pipe::SouthEast),
                    _ => unreachable!(),
                };

                cells.push(cell);

                width += 1;
            }
            height += 1;
        }

        let start = start.unwrap();

        let mut map = Self {
            cells,
            width,
            height,
            start,
        };

        let east = map.get_pipe(start.0 + 1, start.1);
        let west = map.get_pipe(start.0 - 1, start.1);
        let north = map.get_pipe(start.0, start.1 - 1);
        let south = map.get_pipe(start.0, start.1 + 1);

        let east = east.map(|e| e.west()).unwrap_or(false);
        let west = west.map(|w| w.east()).unwrap_or(false);
        let north = north.map(|n| n.south()).unwrap_or(false);
        let south = south.map(|s| s.north()).unwrap_or(false);

        let start_pipe = if north && south {
            Pipe::Vertical
        } else if east && west {
            Pipe::Horizontal
        } else if north && east {
            Pipe::NorthEast
        } else if south && east {
            Pipe::SouthEast
        } else if north && west {
            Pipe::NorthWest
        } else if south && west {
            Pipe::SouthWest
        } else {
            panic!("invalid start position: ({},{})", start.0, start.1);
        };

        map.set(start.0, start.1, Cell::Pipe(start_pipe));

        map
    }

    fn set(&mut self, x: i32, y: i32, value: Cell) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            if let Some(cell) = self.cells.get_mut(idx) {
                *cell = value;
            }
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = y * self.width + x;
            self.cells.get(idx).copied()
        }
    }

    fn get_pipe(&self, x: i32, y: i32) -> Option<Pipe> {
        if let Some(Cell::Pipe(pipe)) = self.get(x, y) {
            Some(pipe)
        } else {
            None
        }
    }

    fn neighbors(&self, x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> + '_ {
        let mut i = 0;

        let cur_pipe = self.get_pipe(x, y);

        std::iter::from_fn(move || loop {
            let Some(cur_pipe) = cur_pipe else {
                return None;
            };

            let dir = match i {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                3 => Direction::West,
                _ => return None,
            };

            let (x_off, y_off) = dir.offset();
            let point = (x + x_off, y + y_off);

            i += 1;

            if let Some(next_pipe) = self.get_pipe(point.0, point.1) {
                let connected = match dir {
                    Direction::North if cur_pipe.north() && next_pipe.south() => true,
                    Direction::South if cur_pipe.south() && next_pipe.north() => true,
                    Direction::East if cur_pipe.east() && next_pipe.west() => true,
                    Direction::West if cur_pipe.west() && next_pipe.east() => true,
                    _ => false,
                };

                if connected {
                    return Some(point);
                }
            }
        })
    }

    fn walk_route(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let mut next = None;
        let mut last = None;

        std::iter::from_fn(move || {
            if next == Some(self.start) {
                return None;
            }

            let current = next.unwrap_or(self.start);

            if let Some(last) = last {
                let path = self
                    .neighbors(current.0, current.1)
                    .find(|p| *p != last)
                    .unwrap();

                next = Some(path);
            }

            last = Some(current);
            last
        })
    }

    fn max_route(&self) -> i32 {
        let count = self.walk_route().count();
        count as i32 / 2
    }

    fn clean_clutter(&mut self, route: &HashSet<(i32, i32)>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let (x, y) = (x as i32, y as i32);
                if !route.contains(&(x, y)) {
                    self.set(x, y, Cell::Ground);
                }
            }
        }
    }

    fn get_inflated(&self, x: i32, y: i32) -> Option<InflatedCell> {
        let real_x = x / 2;
        let real_y = y / 2;
        let off_x = x % 2;
        let off_y = y % 2;
        let on_grid = off_x == 0 && off_y == 0;

        match self.get(real_x, real_y) {
            Some(cell) if on_grid => Some(InflatedCell::from(cell)),
            Some(Cell::Pipe(p)) => match p {
                Pipe::Vertical | Pipe::SouthWest | Pipe::SouthEast if off_x == 0 && off_y == 1 => {
                    Some(InflatedCell::Wall)
                }
                Pipe::Horizontal | Pipe::NorthEast | Pipe::SouthEast
                    if off_x == 1 && off_y == 0 =>
                {
                    Some(InflatedCell::Wall)
                }
                _ => Some(InflatedCell::Void),
            },
            Some(_) => Some(InflatedCell::Void),
            None => Some(InflatedCell::Outside),
        }
    }

    fn inflated_neighbors(
        &self,
        x: i32,
        y: i32,
    ) -> impl Iterator<Item = ((i32, i32), InflatedCell)> + '_ {
        let mut i = 0;

        std::iter::from_fn(move || loop {
            let dir = match i {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                3 => Direction::West,
                _ => return None,
            };

            let (x_off, y_off) = dir.offset();
            let (x, y) = (x + x_off, y + y_off);

            i += 1;

            match self.get_inflated(x, y) {
                Some(InflatedCell::Wall) => continue,
                Some(cell) => return Some(((x, y), cell)),
                None => return Some(((x, y), InflatedCell::Outside)),
            }
        })
    }

    fn count_internal_grounds(&mut self) -> i32 {
        let route: HashSet<_> = self.walk_route().collect();
        self.clean_clutter(&route);

        let mut haystack = Vec::new();
        let mut visited = HashSet::with_capacity(self.width * self.height - route.len());

        let grounds = (0..self.height)
            .flat_map(|y| {
                (0..self.width)
                    .map(move |x| (x as i32, y as i32))
                    .filter(|p| !route.contains(p))
            })
            .map(|(x, y)| (x * 2, y * 2));

        for next in grounds {
            if visited.contains(&next) {
                continue;
            }

            let mut count = Some(0);
            haystack.clear();
            haystack.push((next, InflatedCell::Ground));

            while let Some(((x, y), kind)) = haystack.pop() {
                if !visited.insert((x, y)) {
                    continue;
                }

                match kind {
                    InflatedCell::Outside => {
                        count = None;
                        continue;
                    }
                    InflatedCell::Ground => {
                        if let Some(count) = count.as_mut() {
                            *count += 1;
                        }
                    }

                    _ => (),
                }

                haystack.extend(self.inflated_neighbors(x, y));
            }

            if let Some(count) = count {
                return count;
            }
        }

        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum InflatedCell {
    Wall,
    Ground,
    Void,
    Outside,
}

impl From<Cell> for InflatedCell {
    fn from(value: Cell) -> Self {
        if value == Cell::Ground {
            InflatedCell::Ground
        } else {
            InflatedCell::Wall
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Pipe(Pipe),
    Ground,
    Start,
}

impl Pipe {
    fn north(&self) -> bool {
        matches!(self, Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast)
    }

    fn east(&self) -> bool {
        matches!(self, Pipe::Horizontal | Pipe::NorthEast | Pipe::SouthEast)
    }

    fn south(&self) -> bool {
        matches!(self, Pipe::Vertical | Pipe::SouthWest | Pipe::SouthEast)
    }

    fn west(&self) -> bool {
        matches!(self, Pipe::Horizontal | Pipe::NorthWest | Pipe::SouthWest)
    }
}

#[test]
fn test() {
    let input = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;

    assert_eq!(4, part_one(input));

    let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    assert_eq!(8, part_one(input));

    let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;

    assert_eq!(4, part_two(input));

    let input = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"#;

    assert_eq!(4, part_two(input));

    let input = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;

    assert_eq!(8, part_two(input));

    let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;

    assert_eq!(10, part_two(input));
}
