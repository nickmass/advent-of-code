use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    let map = Map::new(input);
    map.solve(Ray::East(0, 0))
}

pub fn part_two(input: &str) -> usize {
    let map = Map::new(input);
    map.max()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    UpwardMirror,
    DownwardMirror,
    VertSplitter,
    HorzSplitter,
}

impl Cell {
    fn interact(&self, ray: Ray) -> Interation {
        match (*self, ray) {
            (Cell::Empty, ray) => Interation::Ignore(ray),
            (Cell::UpwardMirror, Ray::North(x, y)) => Interation::Reflect(Ray::East(x, y)),
            (Cell::UpwardMirror, Ray::South(x, y)) => Interation::Reflect(Ray::West(x, y)),
            (Cell::UpwardMirror, Ray::East(x, y)) => Interation::Reflect(Ray::North(x, y)),
            (Cell::UpwardMirror, Ray::West(x, y)) => Interation::Reflect(Ray::South(x, y)),
            (Cell::DownwardMirror, Ray::North(x, y)) => Interation::Reflect(Ray::West(x, y)),
            (Cell::DownwardMirror, Ray::South(x, y)) => Interation::Reflect(Ray::East(x, y)),
            (Cell::DownwardMirror, Ray::East(x, y)) => Interation::Reflect(Ray::South(x, y)),
            (Cell::DownwardMirror, Ray::West(x, y)) => Interation::Reflect(Ray::North(x, y)),
            (Cell::VertSplitter, Ray::East(x, y) | Ray::West(x, y)) => {
                Interation::Split(Ray::North(x, y), Ray::South(x, y))
            }
            (Cell::VertSplitter, ray) => Interation::Ignore(ray),
            (Cell::HorzSplitter, Ray::North(x, y) | Ray::South(x, y)) => {
                Interation::Split(Ray::East(x, y), Ray::West(x, y))
            }
            (Cell::HorzSplitter, ray) => Interation::Ignore(ray),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Ray {
    North(i32, i32),
    South(i32, i32),
    East(i32, i32),
    West(i32, i32),
}

impl Ray {
    fn step(&self) -> Ray {
        match *self {
            Ray::North(x, y) => Ray::North(x, y - 1),
            Ray::South(x, y) => Ray::South(x, y + 1),
            Ray::East(x, y) => Ray::East(x + 1, y),
            Ray::West(x, y) => Ray::West(x - 1, y),
        }
    }

    fn point(&self) -> (i32, i32) {
        match *self {
            Ray::North(x, y) => (x, y),
            Ray::South(x, y) => (x, y),
            Ray::East(x, y) => (x, y),
            Ray::West(x, y) => (x, y),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Interation {
    Split(Ray, Ray),
    Reflect(Ray),
    Ignore(Ray),
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
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
                    '/' => Cell::UpwardMirror,
                    '\\' => Cell::DownwardMirror,
                    '|' => Cell::VertSplitter,
                    '-' => Cell::HorzSplitter,
                    _ => unreachable!(),
                };

                cells.push(cell)
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

    fn solve(&self, initial: Ray) -> usize {
        let mut rays = vec![initial];
        let mut visited = HashSet::new();

        while let Some(ray) = rays.pop() {
            if let Some(cell) = self.get(ray.point()) {
                if !visited.insert(ray) {
                    continue;
                }

                match cell.interact(ray) {
                    Interation::Split(a, b) => {
                        rays.push(a.step());
                        rays.push(b.step());
                    }
                    Interation::Reflect(ray) => rays.push(ray.step()),
                    Interation::Ignore(ray) => rays.push(ray.step()),
                }
            }
        }

        let unique_points: HashSet<_> = visited.into_iter().map(|r| r.point()).collect();

        unique_points.len()
    }

    fn max(&self) -> usize {
        let width = self.width as i32 - 1;
        let height = self.height as i32 - 1;

        (0..=width)
            .map(|x| Ray::South(x, 0))
            .chain((0..=width).map(|x| Ray::North(x, height)))
            .chain((0..=height).map(|y| Ray::East(0, y)))
            .chain((0..=height).map(|y| Ray::West(width, y)))
            .map(|ray| self.solve(ray))
            .max()
            .unwrap()
    }
}

#[test]
fn test() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
    assert_eq!(46, part_one(input));
    assert_eq!(51, part_two(input));
}
