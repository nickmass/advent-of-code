use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    solve_part_one::<64>(input)
}

fn solve_part_one<const STEPS: i64>(input: &str) -> usize {
    let map = Map::new(input);
    map.visited_squares(STEPS, false).len()
}

pub fn part_two(input: &str) -> i64 {
    // This is not a general solution, it requres a square map.
    // And some parts depend on how steps divides into the map width,
    // I can resolve that and get it working for the test cases with some
    // minor tweaks - but this is good for now.

    let steps = 26501365;

    let map = Map::new(input);
    let width = map.width as i64;
    let min_steps = (steps % width) + width + width;
    let grid_results = map.visited_squares_per_grid(min_steps, true);

    let (mut a, mut b) = (0, 0);
    for i in 1..(steps / width) {
        if i % 2 == 0 {
            a += 4 + ((i - 1) * 4);
        } else {
            b += 4 + ((i - 1) * 4);
        }
    }
    a += 1;

    let center = grid_results.get(&(0, 0)).copied().unwrap();
    let center_odd = grid_results.get(&(0, 1)).copied().unwrap();
    let north = grid_results.get(&(0, -2)).copied().unwrap();
    let south = grid_results.get(&(0, 2)).copied().unwrap();
    let west = grid_results.get(&(-2, 0)).copied().unwrap();
    let east = grid_results.get(&(2, 0)).copied().unwrap();
    let north_west = grid_results.get(&(-2, -1)).copied().unwrap();
    let north_west_odd = grid_results.get(&(-1, -1)).copied().unwrap();
    let north_east = grid_results.get(&(2, -1)).copied().unwrap();
    let north_east_odd = grid_results.get(&(1, -1)).copied().unwrap();
    let south_west = grid_results.get(&(-2, 1)).copied().unwrap();
    let south_west_odd = grid_results.get(&(-1, 1)).copied().unwrap();
    let south_east = grid_results.get(&(2, 1)).copied().unwrap();
    let south_east_odd = grid_results.get(&(1, 1)).copied().unwrap();

    let corners: i64 = [north, south, west, east].into_iter().sum();
    let long_edge: i64 = [north_west, north_east, south_west, south_east]
        .into_iter()
        .sum();

    let short_edge: i64 = [
        north_west_odd,
        north_east_odd,
        south_west_odd,
        south_east_odd,
    ]
    .into_iter()
    .sum();

    return corners
        + (steps / width * long_edge)
        + (((steps / width) - 1) * short_edge)
        + (center * a)
        + (center_odd * b);
}

#[allow(dead_code)]
fn solve_part_two<const STEPS: i64>(input: &str) -> usize {
    let map = Map::new(input);
    map.visited_squares(STEPS, true).len()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Rock,
    Open,
}

struct Map {
    cells: Vec<Cell>,
    start: (i64, i64),
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
                        start = Some((x as i64, y as i64));
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

    fn get(&self, (x, y): (i64, i64), infinite: bool) -> Option<Cell> {
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
        (x, y): (i64, i64),
        infinite: bool,
    ) -> impl Iterator<Item = (i64, i64)> + '_ {
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

    fn normalize(&self, (x, y): (i64, i64)) -> (i64, i64) {
        (
            x.rem_euclid(self.width as i64),
            y.rem_euclid(self.height as i64),
        )
    }

    fn grid_position(&self, (x, y): (i64, i64)) -> (i64, i64) {
        (
            x.div_euclid(self.width as i64),
            y.div_euclid(self.height as i64),
        )
    }

    fn visited_squares(&self, steps: i64, infinite: bool) -> HashSet<(i64, i64)> {
        let mut recent_new = HashSet::new();
        let mut recent_old = HashSet::new();
        let mut map_new = HashSet::new();
        let mut map_old = HashSet::new();

        recent_old.insert(self.start);
        map_old.insert(self.start);

        for _ in 0..steps {
            for p in recent_old.drain() {
                for n in self.neighbors(p, infinite) {
                    if map_new.insert(n) {
                        recent_new.insert(n);
                    }
                }
            }

            std::mem::swap(&mut recent_old, &mut recent_new);
            std::mem::swap(&mut map_old, &mut map_new);
        }

        map_old
    }

    fn visited_squares_per_grid(&self, steps: i64, infinite: bool) -> HashMap<(i64, i64), i64> {
        let visits = self.visited_squares(steps, infinite);
        let mut result = HashMap::new();

        for p in visits {
            let grid_pos = self.grid_position(p);
            let entry = result.entry(grid_pos).or_insert(0);
            *entry += 1;
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
    assert_eq!(167004, solve_part_two::<500>(input));

    //assert_eq!(668697, solve_part_two::<1000>(input));
    //assert_eq!(16733044, solve_part_two::<5000>(input));
}
