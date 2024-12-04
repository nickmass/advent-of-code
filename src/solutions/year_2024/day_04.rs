pub fn part_one(input: &str) -> usize {
    Grid::new(input).search_xmas()
}

pub fn part_two(input: &str) -> usize {
    Grid::new(input).search_mas()
}

struct Grid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let input = input.trim();
        let mut cells = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 1;
        for b in input.bytes() {
            match b {
                b'\n' => {
                    height += 1;
                    width = 0;
                }
                b => {
                    width += 1;
                    cells.push(b)
                }
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<u8> {
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

    fn xmas_count(&self, p: (i32, i32)) -> usize {
        let mut sum = 0;
        for dir in Direction::all() {
            if OffsetIter::new(*dir, p)
                .zip(b"XMAS")
                .map(|(off, c)| (self.get(off), c))
                .all(|(m, c)| m == Some(*c))
            {
                sum += 1;
            }
        }

        sum
    }

    fn is_mas(&self, p: (i32, i32)) -> bool {
        let Some(b'A') = self.get(p) else {
            return false;
        };

        let corners = [
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
        .map(|dir| self.get(dir.offset(p)));

        let [Some(ul), Some(ur), Some(dl), Some(dr)] = corners else {
            return false;
        };

        let falling = ul == b'M' && dr == b'S' || ul == b'S' && dr == b'M';
        let rising = ur == b'M' && dl == b'S' || ur == b'S' && dl == b'M';

        falling && rising
    }

    fn points(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let mut x = 0;
        let mut y = 0;

        std::iter::from_fn(move || {
            if y >= self.height {
                return None;
            }

            let val = (x as i32, y as i32);

            x += 1;
            if x >= self.width {
                x = 0;
                y += 1;
            }

            Some(val)
        })
    }

    fn search_xmas(&self) -> usize {
        self.points().map(|p| self.xmas_count(p)).sum()
    }

    fn search_mas(&self) -> usize {
        self.points().filter(|p| self.is_mas(*p)).count()
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn offset(self, (x, y): (i32, i32)) -> (i32, i32) {
        let (off_x, off_y) = match self {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        };

        (x + off_x, y + off_y)
    }

    fn all() -> &'static [Direction] {
        &[
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }
}

struct OffsetIter {
    origin: (i32, i32),
    offset: (i32, i32),
    dir: Direction,
}

impl OffsetIter {
    fn new(dir: Direction, origin: (i32, i32)) -> Self {
        Self {
            origin,
            offset: (0, 0),
            dir,
        }
    }
}

impl Iterator for OffsetIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let val = (self.offset.0 + self.origin.0, self.offset.1 + self.origin.1);
        self.offset = self.dir.offset(self.offset);

        Some(val)
    }
}

#[test]
fn test() {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    assert_eq!(18, part_one(input));
    assert_eq!(9, part_two(input));
}
