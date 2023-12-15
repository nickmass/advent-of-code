pub fn part_one(input: &str) -> usize {
    let lines = input.trim().lines();

    let mut min_heights = Vec::new();

    let mut count = 0;
    let mut sum = 0;
    let mut max_height = 0;

    for (height, line) in lines.enumerate() {
        if min_heights.len() < line.len() {
            min_heights.resize(line.len(), 0);
        }

        for (idx, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    let height = min_heights[idx];
                    min_heights[idx] = height + 1;
                    sum += height;
                    count += 1;
                }
                '#' => min_heights[idx] = height + 1,
                '.' => (),
                _ => unreachable!(),
            }
        }
        max_height += 1;
    }

    count * max_height - sum
}

pub fn part_two(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.run_to_cycle(1_000_000_000)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Rolling,
    Blocked,
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    min_heights: Vec<i32>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::with_capacity(input.len());

        for line in input.trim().lines() {
            if width < line.len() {
                width = line.len();
            }
            height += 1;

            for c in line.chars() {
                let cell = match c {
                    '.' => Cell::Empty,
                    'O' => Cell::Rolling,
                    '#' => Cell::Blocked,
                    _ => unreachable!(),
                };

                cells.push(cell);
            }
        }

        let min_heights = vec![0; width];

        Map {
            cells,
            width,
            height,
            min_heights,
        }
    }

    fn get<P: IntoPoint>(&self, point: P) -> Option<Cell> {
        let (x, y) = point.into(self.width, self.height);

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

    fn set<P: IntoPoint>(&mut self, point: P, cell: Cell) {
        let (x, y) = point.into(self.width, self.height);

        if x < 0 || y < 0 {
            return;
        }

        let (x, y) = (x as usize, y as usize);

        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.cells[idx] = cell;
        }
    }

    fn move_to<S: IntoPoint, D: IntoPoint>(&mut self, src: S, dst: D) {
        self.set(src, Cell::Empty);
        self.set(dst, Cell::Rolling);
    }

    fn cycle(&mut self) {
        for origin in Orientation::origins() {
            self.tilt(origin);
        }
    }

    fn tilt(&mut self, origin: Orientation) {
        let width = self.width as i32;
        let height = self.height as i32;
        self.min_heights.fill(0);

        for y in 0..height {
            for x in 0..width {
                let point = origin.set((x, y));

                match self.get(point) {
                    Some(Cell::Empty) => (),
                    Some(Cell::Rolling) => {
                        let height = self.min_heights[x as usize];
                        self.min_heights[x as usize] = height + 1;

                        let dst = point.set((x, height));
                        self.move_to(point, dst);
                    }
                    Some(Cell::Blocked) => {
                        self.min_heights[x as usize] = y + 1;
                    }
                    None => continue,
                }
            }
        }
    }

    fn run_to_cycle(&mut self, target: usize) -> i32 {
        let mut dupes = crate::HashMap::new();
        let mut dupe_found = false;
        let mut i = 0;

        while i < target {
            self.cycle();

            if !dupe_found {
                use std::hash::{Hash, Hasher};
                let mut h = crate::Hasher::default();
                self.cells.hash(&mut h);
                let hash = h.finish();

                if let Some(dupe) = dupes.insert(hash, i) {
                    let loop_len = i - dupe;
                    let remaining = target - i;

                    i = ((remaining / loop_len) * loop_len) + i;

                    dupe_found = true;
                }
            }

            i += 1;
        }

        self.north_load()
    }

    fn north_load(&self) -> i32 {
        let width = self.width as i32;
        let height = self.height as i32;

        let mut sum = 0;
        let mut count = 0;

        for y in 0..height {
            for x in 0..width {
                let point = Orientation::North(x, y);

                match self.get(point) {
                    Some(Cell::Rolling) => {
                        sum += y;
                        count += 1;
                    }
                    _ => (),
                }
            }
        }

        count * height - sum
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let (x, y) = (x as i32, y as i32);
                let cell = self.get(Orientation::North(x, y));

                let c = match cell {
                    Some(Cell::Empty) => '.',
                    Some(Cell::Rolling) => 'O',
                    Some(Cell::Blocked) => '#',
                    None => todo!(),
                };

                print!("{c}");
            }
            println!();
        }
    }
}

trait IntoPoint {
    fn into(self, width: usize, height: usize) -> (i32, i32);
}

#[derive(Debug, Copy, Clone)]
enum Orientation {
    North(i32, i32),
    West(i32, i32),
    South(i32, i32),
    East(i32, i32),
}

impl Orientation {
    fn origins() -> impl Iterator<Item = Orientation> {
        [
            Orientation::North(0, 0),
            Orientation::West(0, 0),
            Orientation::South(0, 0),
            Orientation::East(0, 0),
        ]
        .into_iter()
    }

    fn set(self, (x, y): (i32, i32)) -> Self {
        match self {
            Orientation::North(_, _) => Orientation::North(x, y),
            Orientation::West(_, _) => Orientation::West(x, y),
            Orientation::South(_, _) => Orientation::South(x, y),
            Orientation::East(_, _) => Orientation::East(x, y),
        }
    }
}

impl IntoPoint for Orientation {
    fn into(self, width: usize, height: usize) -> (i32, i32) {
        let width = width as i32;
        let height = height as i32;
        match self {
            Orientation::North(x, y) => (x, y),
            Orientation::East(x, y) => ((height - y) - 1, x),
            Orientation::South(x, y) => (x, (height - y) - 1),
            Orientation::West(x, y) => (y, (width - x) - 1),
        }
    }
}

#[test]
fn test() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;
    assert_eq!(136, part_one(input));
    assert_eq!(64, part_two(input));
}
