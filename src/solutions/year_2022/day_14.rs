pub fn part_one(input: &str) -> u64 {
    let mut grid = Grid::new(input, false);

    let mut count = 0;
    while let Some(_) = grid.drop() {
        count += 1;
    }

    count
}

pub fn part_two(input: &str) -> u64 {
    let mut grid = Grid::new(input, true);

    let mut count = 0;
    while let Some(_) = grid.drop() {
        count += 1;
    }

    count + 1
}

struct Grid {
    cells: Vec<bool>,
    width: usize,
    height: usize,
    drop: (i32, i32),
    floor: bool,
}

impl Grid {
    fn new(input: &str, floor: bool) -> Self {
        let mut max_y = None;

        for line in input.lines() {
            for (_, y) in walk_line(line) {
                max_y = Some(max_y.unwrap_or(y).max(y));
            }
        }

        let height = max_y.unwrap() as usize + 2;
        let width = height * 2;
        let mut cells = vec![false; width * height];

        for line in input.lines() {
            for (x, y) in walk_line(line) {
                let x = ((x - 500) + height as i32) as usize;
                let y = y as usize;

                cells[(y * width) + x] = true;
            }
        }

        Self {
            cells,
            width,
            height,
            floor,
            drop: (height as i32, 0),
        }
    }

    fn drop(&mut self) -> Option<()> {
        let (mut x, mut y) = self.drop;

        loop {
            if !self.get(x, y + 1)? {
                y += 1;
            } else if !self.get(x - 1, y + 1)? {
                x -= 1;
                y += 1;
            } else if !self.get(x + 1, y + 1)? {
                x += 1;
                y += 1;
            } else {
                self.set(x, y);
                if (x, y) == self.drop {
                    return None;
                } else {
                    return Some(());
                }
            }
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<bool> {
        if self.floor && y as usize == self.height {
            Some(true)
        } else if !(0..self.width as i32).contains(&x) || !(0..self.height as i32).contains(&y) {
            None
        } else {
            let x = x as usize;
            let y = y as usize;

            self.cells.get(y * self.width + x).copied()
        }
    }

    fn set(&mut self, x: i32, y: i32) {
        if !(0..self.width as i32).contains(&x) || !(0..self.height as i32).contains(&y) {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        self.cells[y * self.width + x] = true;
    }
}

fn walk_line(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    let mut segments = input
        .split(" -> ")
        .filter_map(|pair| pair.split_once(','))
        .filter_map(|(x, y)| Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?)));

    let mut point = segments.next().unwrap();
    let mut target = point;

    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            return None;
        } else if point != target {
            point.0 += (target.0 - point.0).signum();
            point.1 += (target.1 - point.1).signum();
        } else {
            if let Some(next) = segments.next() {
                target = next;
            } else {
                done = true;
            }
        }

        Some(point)
    })
}

#[test]
fn test() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    assert_eq!(24, part_one(input));
    assert_eq!(93, part_two(input));
}
