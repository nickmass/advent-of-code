use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    solve_part_one::<64>(input)
}

fn solve_part_one<const STEPS: usize>(input: &str) -> usize {
    let map = Map::new(input);
    map.track_steps::<STEPS>(false)
}

pub fn part_two(input: &str) -> usize {
    solve_part_two::<26_501_365>(input)
}

fn solve_part_two<const STEPS: usize>(input: &str) -> usize {
    let map = Map::new(input);
    map.track_steps::<STEPS>(true)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Rock,
    Open,
}

struct Map {
    cells: Vec<Cell>,
    start: (i32, i32),
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut cells = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 0;

        let mut start = None;

        for (y, line) in input.trim().lines().enumerate() {
            if width < line.len() {
                width = line.len();
            }
            height += 1;

            for (x, c) in line.chars().enumerate() {
                let cell = match c {
                    '.' => Cell::Open,
                    '#' => Cell::Rock,
                    'S' => {
                        start = Some((x as i32, y as i32));
                        Cell::Open
                    }
                    _ => unreachable!(),
                };

                cells.push(cell);
            }
        }

        let start = start.unwrap();

        Self {
            cells,
            start,
            width,
            height,
        }
    }

    fn get(&self, (x, y): (i32, i32), infinite: bool) -> Option<Cell> {
        let (x, y) = if infinite {
            self.normalize((x, y))
        } else {
            (x, y)
        };

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

    fn neighbors(
        &self,
        (x, y): (i32, i32),
        infinite: bool,
    ) -> impl Iterator<Item = (i32, i32)> + '_ {
        let mut i = 0;

        std::iter::from_fn(move || loop {
            let p = match i {
                0 => (x + 1, y),
                1 => (x - 1, y),
                2 => (x, y + 1),
                3 => (x, y - 1),
                _ => return None,
            };

            i += 1;

            if let Some(cell) = self.get(p, infinite) {
                if cell == Cell::Open {
                    return Some(p);
                }
            }
        })
    }

    fn normalize(&self, (x, y): (i32, i32)) -> (i32, i32) {
        (
            x.rem_euclid(self.width as i32),
            y.rem_euclid(self.height as i32),
        )
    }

    fn track_steps<const STEPS: usize>(&self, infinite: bool) -> usize {
        // TODO: Figure out what the hell is going on with part 2
        if infinite && STEPS > 5000 {
            return 0;
        }

        let mut haystack = Vec::new();
        haystack.push((0, self.start));

        let mut visited = HashSet::new();
        let mut result = 0;

        while let Some((steps, point)) = haystack.pop() {
            if !visited.insert((steps, point)) {
                continue;
            }

            if steps == STEPS {
                result += 1;
                continue;
            }

            haystack.extend(self.neighbors(point, infinite).map(|p| (steps + 1, p)));
        }

        result
    }
}

#[test]
fn test() {
    let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;

    assert_eq!(16, solve_part_one::<6>(input));
    assert_eq!(16, solve_part_two::<6>(input));
    assert_eq!(50, solve_part_two::<10>(input));
    assert_eq!(1594, solve_part_two::<50>(input));
    assert_eq!(6536, solve_part_two::<100>(input));

    //assert_eq!(167004, solve_part_two::<500>(input));
    //assert_eq!(668697, solve_part_two::<1000>(input));
    //assert_eq!(16733044, solve_part_two::<5000>(input));
}
