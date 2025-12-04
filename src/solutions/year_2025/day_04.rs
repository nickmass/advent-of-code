pub fn part_one(input: &str) -> usize {
    Grid::new(input).part_one()
}

pub fn part_two(input: &str) -> usize {
    Grid::new(input).part_two()
}

struct Grid {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut height = 0;
        let mut width = 0;

        let input = input.as_bytes();
        let mut cells = Vec::with_capacity(input.len());

        for &b in input {
            if b == b'\n' {
                height += 1;
                continue;
            }

            if height == 0 {
                width += 1;
            }

            let cell = match b {
                b'.' => false,
                b'@' => true,
                _ => panic!("unexpected grid cell: {}", b as char),
            };

            cells.push(cell);
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, p: Point) -> Option<bool> {
        let idx = p.to_index(self.width, self.height)?;

        self.cells.get(idx).copied()
    }

    fn remove(&mut self, p: Point) {
        let Some(idx) = p.to_index(self.width, self.height) else {
            return;
        };

        self.cells[idx] = false;
    }

    fn count_neighbors(&self, p: Point) -> usize {
        p.neighbors()
            .filter(|p| self.get(*p).unwrap_or(false))
            .count()
    }

    fn can_forklift(&self, p: Point) -> bool {
        self.get(p).unwrap_or(false) && self.count_neighbors(p) < 4
    }

    fn part_one(&self) -> usize {
        Point::grid_iter(self.width, self.height)
            .filter(|p| self.can_forklift(*p))
            .count()
    }

    fn part_two(mut self) -> usize {
        let mut total = 0;
        let mut changed = true;

        while changed {
            changed = false;
            for p in Point::grid_iter(self.width, self.height) {
                if self.can_forklift(p) {
                    total += 1;
                    self.remove(p);
                    changed = true;
                }
            }
        }
        total
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn in_bounds(&self, width: usize, height: usize) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }

        let x = self.x as usize;
        let y = self.y as usize;

        x < width && y < height
    }

    fn to_index(&self, width: usize, height: usize) -> Option<usize> {
        if !self.in_bounds(width, height) {
            None
        } else {
            Some((self.y as usize * width) + self.x as usize)
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        let p = *self;
        [
            p + Point::new(-1, -1),
            p + Point::new(-1, 0),
            p + Point::new(-1, 1),
            p + Point::new(0, -1),
            p + Point::new(0, 1),
            p + Point::new(1, -1),
            p + Point::new(1, 0),
            p + Point::new(1, 1),
        ]
        .into_iter()
    }

    fn grid_iter(width: usize, height: usize) -> impl Iterator<Item = Self> {
        PointGridIter {
            width,
            height,
            x: 0,
            y: 0,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, Point { x, y }: Self) -> Self::Output {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

struct PointGridIter {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Iterator for PointGridIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.height {
            None
        } else {
            let next = Point::new(self.x as i32, self.y as i32);
            self.x += 1;
            if self.x == self.width {
                self.y += 1;
                self.x = 0;
            }

            Some(next)
        }
    }
}

#[test]
fn test() {
    let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    assert_eq!(13, part_one(input));
    assert_eq!(43, part_two(input));
}
