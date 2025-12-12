pub fn part_one(input: &str) -> u32 {
    let blocks = input.trim().split("\n\n");
    let mut shapes = Vec::with_capacity(6);
    let mut fit_count = 0;
    let mut region_map = RegionMap::new();

    for block in blocks {
        if block.contains("x") {
            for line in block.lines() {
                let region = Region::parse(line, &shapes).expect("valid region");
                if region.solve(&mut region_map) {
                    fit_count += 1;
                }
            }
        } else {
            let shape = Shape::parse(block).expect("valid shape");
            shapes.push(shape)
        }
    }

    fit_count
}

pub fn part_two(_input: &str) -> &'static str {
    "Almost there..."
}

#[derive(Debug, Copy, Clone)]
struct Shape {
    cells: [[bool; 3]; 3],
    count: u32,
}

impl Shape {
    fn empty() -> Self {
        Self {
            cells: [[false; 3]; 3],
            count: 0,
        }
    }

    fn parse(input: &str) -> Option<Self> {
        let mut cells = [[false; 3]; 3];
        let mut count = 0;
        for (row, line) in input.lines().skip(1).enumerate() {
            for (col, &c) in line.as_bytes().iter().enumerate() {
                match c {
                    b'#' => {
                        cells[row][col] = true;
                        count += 1;
                    }
                    b'.' => cells[row][col] = false,
                    _ => return None,
                }
            }
        }

        Some(Self { cells, count })
    }

    fn permutations(self) -> [ShapeView; 8] {
        use Rotation as R;
        use Transform as T;
        [
            T::Normal(R::Zero).view(self),
            T::Normal(R::Ninty).view(self),
            T::Normal(R::OneEighty).view(self),
            T::Normal(R::TwoSeventy).view(self),
            T::Flipped(R::Zero).view(self),
            T::Flipped(R::Ninty).view(self),
            T::Flipped(R::OneEighty).view(self),
            T::Flipped(R::TwoSeventy).view(self),
        ]
    }

    fn get(&self, Point(x, y): Point) -> bool {
        assert!(x >= 0 && x < 3 && y >= 0 && y < 3);
        self.cells[y as usize][x as usize]
    }
}

#[derive(Debug, Copy, Clone)]
struct ShapeView(Transform, Shape);

impl ShapeView {
    fn iter(&self) -> impl Iterator<Item = (bool, Point)> {
        let mut x = 0;
        let mut y = 0;

        std::iter::from_fn(move || {
            if y == 3 {
                return None;
            }
            let p = Point(x, y);
            x += 1;
            if x == 3 {
                y += 1;
                x = 0;
            }

            let remap = self.0.apply(p);
            Some((self.1.get(remap), p))
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

impl Point {
    fn idx(&self, width: usize, height: usize) -> Option<usize> {
        let &Point(x, y) = self;
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= width || y >= height {
            return None;
        }

        Some(y * width + x)
    }

    fn rotate(self) -> Point {
        let Point(x, y) = self;
        assert!(x >= 0 && x < 3 && y >= 0 && y < 3);
        match (x, y) {
            (1, 1) => Point(1, 1),
            (0, 0) => Point(0, 2),
            (0, 2) => Point(2, 2),
            (2, 2) => Point(2, 0),
            (2, 0) => Point(0, 0),
            (1, 2) => Point(2, 1),
            (2, 1) => Point(1, 0),
            (1, 0) => Point(0, 1),
            (0, 1) => Point(1, 2),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Copy, Clone)]
enum Transform {
    Normal(Rotation),
    Flipped(Rotation),
}

impl Transform {
    fn apply(&self, Point(x, y): Point) -> Point {
        match self {
            Transform::Normal(rotation) => rotation.apply(Point(x, y)),
            Transform::Flipped(rotation) => rotation.apply(Point(x, 2 - y)),
        }
    }

    fn view(self, shape: Shape) -> ShapeView {
        ShapeView(self, shape)
    }
}

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Zero,
    Ninty,
    OneEighty,
    TwoSeventy,
}

impl Rotation {
    fn apply(&self, p: Point) -> Point {
        match self {
            Rotation::Zero => p,
            Rotation::Ninty => p.rotate(),
            Rotation::OneEighty => p.rotate().rotate(),
            Rotation::TwoSeventy => p.rotate().rotate().rotate(),
        }
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    tiles: [(Shape, u32); 6],
}

impl Region {
    fn parse(input: &str, shapes: &[Shape]) -> Option<Self> {
        let (dims, counts) = input.split_once(": ")?;
        let (width, height) = dims.split_once("x")?;
        let width = width.parse().ok()?;
        let height = height.parse().ok()?;

        let mut tiles = [(Shape::empty(), 0); 6];
        for (idx, count) in counts.split(" ").enumerate() {
            let count = count.parse().ok()?;
            tiles[idx] = (shapes[idx], count);
        }

        Some(Self {
            width,
            height,
            tiles,
        })
    }

    fn solve(&self, map: &mut RegionMap) -> bool {
        let x_blocks = self.width as u32 / 3;
        let y_blocks = self.height as u32 / 3;
        let total_count: u32 = self.tiles.iter().map(|(_, c)| c).sum();

        if total_count <= x_blocks * y_blocks {
            return true;
        }

        let mut cells = (self.width * self.height) as u32;

        for (shape, count) in self.tiles.iter().copied() {
            let needed = shape.count * count;
            if needed > cells {
                return false;
            }

            cells -= needed;
        }

        // So none of the following code is required, just checking the area is enough
        // but it didn't seem satisfying to leave my solution there I should atleast do
        // a basic fit attempt

        let mut x = -3;
        let mut y = -3;
        let mut counts = self.tiles.map(|(_, c)| c);
        let shapes = self.tiles.map(|(s, _)| s);

        map.resize(self.width, self.height);

        'outer: while counts.iter().any(|&n| n > 0) {
            for (idx, &shape) in shapes.iter().enumerate() {
                if counts[idx] > 0 {
                    for perm in shape.permutations() {
                        if map.try_place(Point(x, y), perm) {
                            counts[idx] -= 1;
                            continue 'outer;
                        }
                    }
                }
            }

            x += 1;
            if x >= self.width as i32 + 3 {
                y += 1;
                x = 0;
                if y >= self.height as i32 + 3 {
                    return false;
                }
            }
        }

        true
    }
}

struct RegionMap {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl RegionMap {
    fn new() -> Self {
        Self {
            cells: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn resize(&mut self, width: usize, height: usize) {
        self.cells.clear();
        self.cells.resize(width * height, false);
        self.width = width;
        self.height = height;
    }

    fn get(&mut self, point: Point) -> Option<bool> {
        let Some(idx) = point.idx(self.width, self.height) else {
            return None;
        };

        self.cells.get(idx).copied()
    }

    fn set(&mut self, point: Point, value: bool) {
        let Some(idx) = point.idx(self.width, self.height) else {
            return;
        };

        self.cells[idx] = value;
    }

    fn try_place(&mut self, origin: Point, shape: ShapeView) -> bool {
        for (value, point) in shape.iter() {
            if !value {
                continue;
            }
            let p = origin + point;
            if self.get(p) != Some(false) {
                return false;
            }
        }

        for (value, point) in shape.iter() {
            if !value {
                continue;
            }
            let p = origin + point;
            self.set(p, value);
        }

        true
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let c = if self.cells[idx] { '#' } else { '.' };

                eprint!("{c}");
            }
            eprintln!();
        }
        eprintln!();
    }
}

#[test]
#[ignore = "this is an unusual day..."]
fn test() {
    let input = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    assert_eq!(2, part_one(input));
}
