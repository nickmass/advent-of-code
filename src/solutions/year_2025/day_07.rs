pub fn part_one(input: &str) -> u64 {
    GameGrid::new(input.trim()).part_one()
}

pub fn part_two(input: &str) -> u64 {
    GameGrid::new(input.trim()).part_two()
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Start,
    Empty,
    Split,
    Beam,
    Wall,
}

struct GameGrid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    splits: u64,
    start: Point,
}

impl GameGrid {
    fn new(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut start = None;

        let mut cells = Vec::with_capacity(input.len());

        for b in input.as_bytes() {
            let cell = match b {
                b'\n' => {
                    height += 1;
                    continue;
                }
                b'S' => {
                    start = Some(Point(width as i32, height as i32));
                    Cell::Start
                }
                b'.' => Cell::Empty,
                b'^' => Cell::Split,
                _ => panic!("unexpected grid cell: '{}'", *b as char),
            };
            if height == 0 {
                width += 1;
            }

            cells.push(cell);
        }

        let start = start.expect("start is required");

        Self {
            cells,
            width,
            height,
            start,
            splits: 0,
        }
    }

    fn get(&self, p: Point) -> Option<Cell> {
        let idx = p.idx(self.width, self.height)?;
        self.cells.get(idx).copied()
    }

    fn set(&mut self, p: Point, cell: Cell) -> bool {
        let Some(idx) = p.idx(self.width, self.height) else {
            return false;
        };
        let Some(slot) = self.cells.get_mut(idx) else {
            return false;
        };

        *slot = cell;
        true
    }

    fn activate(&mut self, p: Point) -> bool {
        match self.get(p) {
            Some(Cell::Empty) => self.set(p, Cell::Beam),
            Some(Cell::Split) => {
                self.splits += 1;
                self.set(p, Cell::Wall)
                    | self.set(p + Point(-1, 0), Cell::Beam)
                    | self.set(p + Point(1, 0), Cell::Beam)
            }
            _ => return false,
        }
    }

    fn part_one(&mut self) -> u64 {
        let mut progress = true;
        let mut min_y = 0;

        while progress {
            progress = false;
            let mut next_y = self.height;
            for y in (min_y..self.height).rev() {
                for x in 0..self.width {
                    let point = Point(x as i32, y as i32);
                    let changed = match self.get(point) {
                        Some(Cell::Beam) | Some(Cell::Start) => self.activate(point + Point(0, 1)),
                        _ => false,
                    };

                    if changed {
                        next_y = y;
                    }

                    progress |= changed;
                }
            }
            min_y = next_y;
        }

        self.splits
    }

    fn part_two(&mut self) -> u64 {
        let mut scores = vec![0; self.cells.len()];

        if let Some(start) = self
            .start
            .idx(self.width, self.height)
            .and_then(|idx| scores.get_mut(idx))
        {
            *start = 1;
        } else {
            return 0;
        };

        let mut timelines = 0;

        for y in 1..self.height {
            let last_row = y == self.height - 1;
            for x in 0..self.width {
                let point = Point(x as i32, y as i32);
                let Some(cell) = self.get(point) else {
                    continue;
                };

                let source_point = point + Point(0, -1);
                let source_score = source_point
                    .idx(self.width, self.height)
                    .and_then(|idx| scores.get(idx))
                    .copied()
                    .unwrap_or(0);

                if source_score == 0 {
                    continue;
                }

                let destinations: &[Point] = match cell {
                    Cell::Split => &[point + Point(1, 0), point + Point(-1, 0)],
                    _ => &[point],
                };

                for dest in destinations {
                    if let Some(dest_score) = dest
                        .idx(self.width, self.height)
                        .and_then(|idx| scores.get_mut(idx))
                    {
                        if last_row {
                            timelines += source_score;
                        } else {
                            *dest_score += source_score;
                        }
                    }
                }
            }
        }

        timelines
    }
}

#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

impl Point {
    fn idx(&self, width: usize, height: usize) -> Option<usize> {
        let &Point(x, y) = self;
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= width || y >= height {
            return None;
        }

        Some(y * width + x)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[test]
fn test() {
    let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    assert_eq!(21, part_one(input));
    assert_eq!(40, part_two(input));
}
