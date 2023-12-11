pub fn part_one(input: &str) -> u64 {
    let grid = Grid::new(input);

    let mut count = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.visible(x as isize, y as isize) {
                count += 1;
            }
        }
    }

    count
}

pub fn part_two(input: &str) -> u64 {
    let grid = Grid::new(input);

    let mut max_score = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let score = grid.score(x as isize, y as isize);
            max_score = max_score.max(score);
        }
    }

    max_score
}

struct Grid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::with_capacity(input.len());
        for line in input.trim().lines() {
            width = width.max(line.len());
            height += 1;
            for c in line.chars().filter_map(|c| c.to_digit(10)) {
                cells.push(c as u8);
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            None
        } else {
            let x = x as usize;
            let y = y as usize;
            let idx = y * self.width + x;

            self.cells.get(idx).copied()
        }
    }

    fn visible(&self, x: isize, y: isize) -> bool {
        let me = self.get(x, y).unwrap();
        let mut blocked = [false; 4];
        for n in 1.. {
            let mut neighbors = [None; 4];

            neighbors[0] = self.get(x + n, y);
            neighbors[1] = self.get(x - n, y);
            neighbors[2] = self.get(x, y + n);
            neighbors[3] = self.get(x, y - n);

            blocked
                .iter_mut()
                .zip(neighbors)
                .for_each(|(b, v)| *b = *b || v.map(|v| v >= me).unwrap_or(false));

            if neighbors.iter().all(|v| v.is_none()) {
                break;
            }
        }

        blocked.iter().any(|v| !v)
    }

    fn score(&self, x: isize, y: isize) -> u64 {
        let me = self.get(x, y).unwrap();
        let mut blocked = [false; 4];
        let mut score = [0; 4];
        for n in 1.. {
            let mut neighbors = [None; 4];

            neighbors[0] = self.get(x, y - n);
            neighbors[1] = self.get(x - n, y);
            neighbors[2] = self.get(x, y + n);
            neighbors[3] = self.get(x + n, y);

            for s in score
                .iter_mut()
                .zip(neighbors)
                .zip(blocked)
                .filter(|(_, b)| !b)
                .filter_map(|((s, v), _)| v.map(|_| s))
            {
                *s = n as u64;
            }

            blocked
                .iter_mut()
                .zip(neighbors)
                .for_each(|(b, v)| *b = *b || v.map(|v| v >= me).unwrap_or(false));

            if neighbors.iter().all(|v| v.is_none()) {
                break;
            }
        }

        score.into_iter().product()
    }
}

#[test]
fn test() {
    let input = r#"30373
25512
65332
33549
35390"#;

    assert_eq!(21, part_one(input));
    assert_eq!(8, part_two(input));
}
