use crate::HashSet;

pub fn part_one(input: &str) -> u64 {
    let map = Map::new(input);
    map.part_one()
}

pub fn part_two(input: &str) -> u64 {
    let map = Map::new(input);
    map.part_two()
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Symbol,
    Number(usize),
    Gear,
}

impl Cell {
    fn is_symbolic(self) -> bool {
        matches!(self, Cell::Symbol | Cell::Gear)
    }

    fn number(self) -> Option<usize> {
        match self {
            Cell::Number(n) => Some(n),
            _ => None,
        }
    }
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    numbers: Vec<u64>,
}

impl Map {
    fn new(input: &str) -> Self {
        let lines = input.trim();
        let mut cells = Vec::new();
        let mut width = None;
        let mut x = 0;
        let mut y = 0;

        let mut value = 0;

        let mut numbers = Vec::new();
        let mut was_in_num = false;

        for b in lines.as_bytes() {
            let mut in_num = false;
            let cell = match b {
                b'0'..=b'9' => {
                    value *= 10;
                    value += (b - b'0') as u64;
                    in_num = true;
                    Cell::Number(numbers.len())
                }
                b'.' => Cell::Empty,
                b'\n' => {
                    if width.is_none() {
                        width = Some(x);
                    }
                    x = 0;
                    y += 1;

                    if was_in_num {
                        numbers.push(value);
                    }

                    value = 0;
                    was_in_num = false;
                    continue;
                }
                b'*' => Cell::Gear,
                _ => Cell::Symbol,
            };

            cells.push(cell);
            if !in_num && was_in_num {
                numbers.push(value);
                value = 0;
            }
            was_in_num = in_num;
            x += 1;
        }
        let height = (y + 1) as usize;
        let width = width.unwrap() as usize;

        Self {
            cells,
            width,
            height,
            numbers,
        }
    }

    fn part_one(&self) -> u64 {
        let mut matches = HashSet::new();
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                if let Some(Cell::Number(n)) = self.get(x, y) {
                    if !matches.contains(&n) && self.neighbors(x, y).any(Cell::is_symbolic) {
                        matches.insert(n);
                    }
                }
            }
        }

        matches.into_iter().map(|n| self.numbers[n]).sum()
    }

    fn part_two(&self) -> u64 {
        let mut total = 0;
        let mut matches = HashSet::new();
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                if let Some(Cell::Gear) = self.get(x, y) {
                    matches.extend(self.neighbors(x, y).filter_map(Cell::number));

                    if matches.len() == 2 {
                        total += matches.iter().map(|n| self.numbers[*n]).product::<u64>();
                    }

                    matches.clear();
                }
            }
        }

        total
    }

    fn get(&self, x: i32, y: i32) -> Option<Cell> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            None
        } else {
            let idx = y as usize * self.width + x as usize;
            Some(self.cells[idx])
        }
    }

    fn neighbors(&self, x: i32, y: i32) -> impl Iterator<Item = Cell> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || loop {
            let (x, y) = match i {
                0 => (x - 1, y - 1),
                1 => (x - 1, y),
                2 => (x - 1, y + 1),
                3 => (x, y - 1),
                4 => (x, y + 1),
                5 => (x + 1, y - 1),
                6 => (x + 1, y),
                7 => (x + 1, y + 1),
                _ => return None,
            };
            i += 1;

            if let Some(cell) = self.get(x, y) {
                return Some(cell);
            }
        })
    }
}

#[test]
fn test() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    assert_eq!(4361, part_one(input));
    assert_eq!(467835, part_two(input));
}
