use crate::HashSet;

pub fn part_one(input: &str) -> u64 {
    let mut grid = Grid::new(input);
    grid.find_dupicate()
}

pub fn part_two(input: &str) -> u64 {
    solve_part_two::<200>(input)
}

pub fn solve_part_two<const MINUTES: usize>(input: &str) -> u64 {
    let mut grid = RecursiveGrid::<5>::new(input);
    grid.count_bugs::<MINUTES>()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Bug,
}

struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 0;

        for line in input.trim().lines() {
            if width < line.len() {
                width = line.len();
            }

            height += 1;

            for c in line.chars() {
                let cell = match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Bug,
                    _ => unreachable!(),
                };

                cells.push(cell);
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<Cell> {
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

    fn count_neighbors(&self, (x, y): (i32, i32)) -> usize {
        [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter_map(|p| self.get(p))
            .filter(|&c| c == Cell::Bug)
            .count()
    }

    fn tick(&mut self) {
        let mut new_cells = vec![Cell::Empty; self.cells.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                let p = (x as i32, y as i32);

                let count = self.count_neighbors(p);
                let Some(cell) = self.get(p) else {
                    continue;
                };

                let new_cell = match (cell, count) {
                    (Cell::Empty, 1 | 2) => Cell::Bug,
                    (Cell::Bug, 1) => Cell::Bug,
                    (Cell::Bug, _) => Cell::Empty,
                    _ => cell,
                };

                let idx = y * self.width + x;
                new_cells[idx] = new_cell;
            }
        }

        self.cells = new_cells;
    }

    fn find_dupicate(&mut self) -> u64 {
        let mut states = HashSet::new();

        loop {
            use std::hash::{Hash, Hasher};
            let mut hasher = crate::Hasher::default();
            self.cells.hash(&mut hasher);

            if !states.insert(hasher.finish()) {
                return self.score();
            }

            self.tick();
        }
    }

    fn score(&self) -> u64 {
        let mut score = 0;
        for (idx, &cell) in self.cells.iter().enumerate() {
            let n = idx as u32;

            if cell == Cell::Bug {
                score += 2u64.pow(n);
            }
        }

        score
    }
}

struct RecursiveGrid<const DIM: usize> {
    layers: Vec<RecursiveLayer<DIM>>,
    neg_layers: Vec<RecursiveLayer<DIM>>,
    temp_layers: Vec<RecursiveLayer<DIM>>,
    temp_neg_layers: Vec<RecursiveLayer<DIM>>,
}

impl<const DIM: usize> RecursiveGrid<DIM> {
    fn new(input: &str) -> Self {
        let layer_zero = RecursiveLayer::new(input);
        let layers = vec![layer_zero];
        let neg_layers = Vec::new();

        Self {
            layers,
            neg_layers,
            temp_layers: Vec::new(),
            temp_neg_layers: Vec::new(),
        }
    }

    fn get(&self, (x, y, z): (i32, i32, isize)) -> RecursiveCell {
        let layer = if z >= 0 {
            let z = z as usize;
            if z >= self.layers.len() {
                return RecursiveLayer::<DIM>::empty().get((x, y));
            }
            &self.layers[z]
        } else {
            let z = (z.abs() - 1) as usize;
            if z >= self.neg_layers.len() {
                return RecursiveLayer::<DIM>::empty().get((x, y));
            }
            &self.neg_layers[z]
        };

        layer.get((x, y))
    }

    fn set(&mut self, (x, y, z): (i32, i32, isize), cell: Cell) {
        let layer = if z >= 0 {
            let z = z as usize;
            while z >= self.temp_layers.len() {
                self.temp_layers.push(RecursiveLayer::empty());
            }
            &mut self.temp_layers[z]
        } else {
            let z = (z.abs() - 1) as usize;
            while z >= self.temp_neg_layers.len() {
                self.temp_neg_layers.push(RecursiveLayer::empty());
            }
            &mut self.temp_neg_layers[z]
        };

        if x < 0 || y < 0 {
            return;
        }

        let (x, y) = (x as usize, y as usize);

        if x > DIM || y > DIM {
            return;
        } else if x == DIM / 2 && y == DIM / 2 {
            return;
        } else {
            layer.cells[y][x] = cell;
        }
    }

    fn z_range(&self) -> (isize, isize) {
        (
            -(self.neg_layers.len() as isize + 1),
            self.layers.len() as isize,
        )
    }

    fn tick(&mut self) {
        let (z_min, z_max) = self.z_range();

        for z in z_min..=z_max {
            for y in 0..DIM {
                for x in 0..DIM {
                    if y == DIM / 2 && x == DIM / 2 {
                        continue;
                    }
                    let (x, y) = (x as i32, y as i32);

                    let RecursiveCell::Cell(cell) = self.get((x, y, z)) else {
                        panic!("unexpected recursion");
                    };
                    let count = self.count_neighbors((x, y, z));

                    let new_cell = match (cell, count) {
                        (Cell::Empty, 1 | 2) => Some(Cell::Bug),
                        (Cell::Bug, 1) => Some(Cell::Bug),
                        (Cell::Bug, _) => Some(Cell::Empty),
                        _ => None,
                    };

                    if let Some(new_cell) = new_cell {
                        self.set((x, y, z), new_cell);
                    }
                }
            }
        }

        self.swap();
    }

    fn count_bugs<const MINUTES: usize>(&mut self) -> u64 {
        for _ in 0..MINUTES {
            self.tick();
        }

        let (z_min, z_max) = self.z_range();

        let mut count = 0;

        for z in z_min..=z_max {
            for y in 0..DIM {
                for x in 0..DIM {
                    if y == DIM / 2 && x == DIM / 2 {
                        continue;
                    }
                    let (x, y) = (x as i32, y as i32);
                    let RecursiveCell::Cell(cell) = self.get((x, y, z)) else {
                        panic!("unexpected recursion");
                    };

                    count += if cell == Cell::Bug { 1 } else { 0 };
                }
            }
        }

        count
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.temp_layers, &mut self.layers);
        std::mem::swap(&mut self.temp_neg_layers, &mut self.neg_layers);
        self.temp_layers.clear();
        self.temp_neg_layers.clear();
    }

    fn count_neighbors(&self, (x, y, z): (i32, i32, isize)) -> usize {
        let left = (x - 1, y, z);
        let right = (x + 1, y, z);
        let up = (x, y - 1, z);
        let down = (x, y + 1, z);

        let left_edge = 0;
        let right_edge = (DIM - 1) as i32;
        let up_edge = 0;
        let down_edge = (DIM - 1) as i32;

        let center = (DIM / 2) as i32;
        let left_inside = center - 1;
        let right_inside = center + 1;
        let up_inside = center - 1;
        let down_inside = center + 1;

        let mut count = 0;

        count += match self.get(left) {
            RecursiveCell::Cell(Cell::Empty) => 0,
            RecursiveCell::Cell(Cell::Bug) => 1,
            RecursiveCell::Inside => {
                let mut n = 0;
                for y in 0..DIM {
                    n += self.get((right_edge, y as i32, z + 1)).count()
                }
                n
            }
            RecursiveCell::Outside => self.get((left_inside, center, z - 1)).count(),
        };

        count += match self.get(right) {
            RecursiveCell::Cell(Cell::Empty) => 0,
            RecursiveCell::Cell(Cell::Bug) => 1,
            RecursiveCell::Inside => {
                let mut n = 0;
                for y in 0..DIM {
                    n += self.get((left_edge, y as i32, z + 1)).count()
                }
                n
            }
            RecursiveCell::Outside => self.get((right_inside, center, z - 1)).count(),
        };

        count += match self.get(up) {
            RecursiveCell::Cell(Cell::Empty) => 0,
            RecursiveCell::Cell(Cell::Bug) => 1,
            RecursiveCell::Inside => {
                let mut n = 0;
                for x in 0..DIM {
                    n += self.get((x as i32, down_edge, z + 1)).count()
                }
                n
            }
            RecursiveCell::Outside => self.get((center, up_inside, z - 1)).count(),
        };

        count += match self.get(down) {
            RecursiveCell::Cell(Cell::Empty) => 0,
            RecursiveCell::Cell(Cell::Bug) => 1,
            RecursiveCell::Inside => {
                let mut n = 0;
                for x in 0..DIM {
                    n += self.get((x as i32, up_edge, z + 1)).count()
                }
                n
            }
            RecursiveCell::Outside => self.get((center, down_inside, z - 1)).count(),
        };

        count
    }
}

struct RecursiveLayer<const DIM: usize> {
    cells: [[Cell; DIM]; DIM],
}

impl<const DIM: usize> RecursiveLayer<DIM> {
    fn new(input: &str) -> Self {
        let mut cells = [[Cell::Empty; DIM]; DIM];

        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cells[y][x] = match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Bug,
                    _ => unreachable!(),
                };
            }
        }

        Self { cells }
    }

    fn empty() -> Self {
        Self {
            cells: [[Cell::Empty; DIM]; DIM],
        }
    }

    fn get(&self, (x, y): (i32, i32)) -> RecursiveCell {
        if x < 0 || y < 0 {
            return RecursiveCell::Outside;
        }

        let (x, y) = (x as usize, y as usize);

        if x >= DIM || y >= DIM {
            RecursiveCell::Outside
        } else if x == DIM / 2 && y == DIM / 2 {
            RecursiveCell::Inside
        } else {
            RecursiveCell::Cell(self.cells[y][x])
        }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..DIM {
            for x in 0..DIM {
                if x == DIM / 2 && y == DIM / 2 {
                    print!("?");
                } else {
                    match self.cells[y][x] {
                        Cell::Empty => print!("."),
                        Cell::Bug => print!("#"),
                    }
                }
            }

            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RecursiveCell {
    Cell(Cell),
    Inside,
    Outside,
}

impl RecursiveCell {
    fn count(&self) -> usize {
        match self {
            RecursiveCell::Cell(Cell::Bug) => 1,
            _ => 0,
        }
    }
}

#[test]
fn test() {
    let input = r#"....#
#..#.
#..##
..#..
#....
"#;

    assert_eq!(2129920, part_one(input));
    assert_eq!(99, solve_part_two::<10>(input));
}
