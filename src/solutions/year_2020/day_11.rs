pub fn part_one(input: &str) -> i32 {
    solution(input, 4, false)
}

pub fn part_two(input: &str) -> i32 {
    solution(input, 5, true)
}

#[derive(Debug, Copy, Clone)]
enum GridCell {
    Empty,
    Seat,
    Occupied,
}

#[derive(Debug, Clone)]
struct DayElevenGrid {
    cells: Vec<GridCell>,
    width: usize,
    height: usize,
    fancy: bool,
}

impl DayElevenGrid {
    fn new(input: &str, fancy_grid: bool) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::with_capacity(input.len());
        for c in input.chars() {
            let cell = match c {
                '#' => Some(GridCell::Occupied),
                'L' => Some(GridCell::Seat),
                '.' => Some(GridCell::Empty),
                '\n' => {
                    height += 1;
                    None
                }
                _ => panic!("invalid grid"),
            };

            if let Some(cell) = cell {
                cells.push(cell);
            }

            if height == 0 {
                width += 1;
            }
        }

        Self {
            cells,
            width,
            height,
            fancy: fancy_grid,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<GridCell> {
        if y >= self.height || x >= self.width {
            None
        } else {
            Some(self.cells[y * self.width + x])
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: GridCell) {
        self.cells[y * self.width + x] = cell;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u32 {
        let mut sum = 0;
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                let mut x = x;
                let mut y = y;
                loop {
                    x = (x as isize + x_offset) as usize;
                    y = (y as isize + y_offset) as usize;

                    match self.get(x, y) {
                        Some(GridCell::Occupied) => sum += 1,
                        Some(GridCell::Empty) if self.fancy => continue,
                        _ => (),
                    }
                    break;
                }
            }
        }
        sum
    }
}

fn solution(input: &str, occupy_limit: u32, fancy: bool) -> i32 {
    let mut grid = DayElevenGrid::new(input, fancy);
    let mut next_grid = grid.clone();

    let mut changed = true;
    let mut occupied = 0;
    while changed {
        changed = false;
        for x in 0..grid.width {
            for y in 0..grid.height {
                let neighbors = grid.count_neighbors(x, y);
                match grid.get(x, y) {
                    Some(GridCell::Seat) if neighbors == 0 => {
                        changed = true;
                        next_grid.set(x, y, GridCell::Occupied);
                        occupied += 1;
                    }
                    Some(GridCell::Occupied) if neighbors >= occupy_limit => {
                        changed = true;
                        next_grid.set(x, y, GridCell::Seat);
                        occupied -= 1;
                    }
                    Some(cell) => {
                        next_grid.set(x, y, cell);
                    }
                    None => unreachable!("outside x y bounds"),
                }
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    occupied
}

#[test]
fn test() {
    let run_a = |inp, res| assert_eq!(part_one(inp), res);
    let run_b = |inp, res| assert_eq!(part_two(inp), res);

    let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;

    run_a(input, 37);
    run_b(input, 26);
}
