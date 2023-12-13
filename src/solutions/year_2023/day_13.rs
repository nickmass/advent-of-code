pub fn part_one(input: &str) -> i32 {
    let (vert, horz) = input
        .trim()
        .split("\n\n")
        .map(|block| Map::new(block))
        .map(|m| m.find_mirroring::<0>())
        .fold((0, 0), |(vert, horz), next| match next {
            Mirroring::Vertical(v) => (vert + v, horz),
            Mirroring::Horizontal(h) => (vert, horz + h),
        });

    vert + (horz * 100)
}

pub fn part_two(input: &str) -> i32 {
    let (vert, horz) = input
        .trim()
        .split("\n\n")
        .map(|block| Map::new(block))
        .map(|m| m.find_mirroring::<1>())
        .fold((0, 0), |(vert, horz), next| match next {
            Mirroring::Vertical(v) => (vert + v, horz),
            Mirroring::Horizontal(h) => (vert, horz + h),
        });

    vert + (horz * 100)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Rock,
    Ash,
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut cells = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 0;

        for line in input.lines() {
            height += 1;
            width = line.len();

            for c in line.chars() {
                let cell = match c {
                    '#' => Cell::Rock,
                    '.' => Cell::Ash,
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

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = y * self.width + x;
            self.cells.get(idx).copied()
        }
    }

    fn find_mirroring<const ERRORS: i32>(&self) -> Mirroring {
        let width = self.width as i32;
        let height = self.height as i32;

        'outer: for x in 1..width {
            let mut error_count = 0;
            for x_off in 0..width {
                for y in 0..height {
                    let a = self.get((x - 1 - x_off, y));
                    let b = self.get((x + x_off, y));

                    if let Some((a, b)) = a.zip(b) {
                        if a != b {
                            error_count += 1;

                            if error_count > ERRORS {
                                continue 'outer;
                            }
                        }
                    } else if error_count == ERRORS {
                        return Mirroring::Vertical(x);
                    }
                }
            }
        }

        'outer: for y in 1..height {
            let mut error_count = 0;
            for y_off in 0..height {
                for x in 0..width {
                    let a = self.get((x, y - 1 - y_off));
                    let b = self.get((x, y + y_off));

                    if let Some((a, b)) = a.zip(b) {
                        if a != b {
                            error_count += 1;

                            if error_count > ERRORS {
                                continue 'outer;
                            }
                        }
                    } else if error_count == ERRORS {
                        return Mirroring::Horizontal(y);
                    }
                }
            }
        }

        unreachable!("no mirroring found");
    }
}

#[derive(Debug, Copy, Clone)]
enum Mirroring {
    Vertical(i32),
    Horizontal(i32),
}

#[test]
fn test() {
    let input = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;
    assert_eq!(405, part_one(input));
    assert_eq!(400, part_two(input));
}
