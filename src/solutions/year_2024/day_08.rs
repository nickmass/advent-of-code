use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    AntennaMap::new(input).part_one()
}

pub fn part_two(input: &str) -> usize {
    AntennaMap::new(input).part_two()
}

struct AntennaMap {
    width: i32,
    height: i32,
    antennas: Vec<Vec<Point>>,
}

impl AntennaMap {
    fn new(input: &str) -> Self {
        let input = input.trim().as_bytes();
        let mut antennas = vec![Vec::new(); 10 + 26 + 26];
        let mut width = 0;
        let mut height = 0;
        for &b in input {
            if b == b'\n' {
                height += 1;
                width = 0;
                continue;
            }

            if let Some(b) = antenna_idx(b) {
                antennas[b].push(Point(width, height));
            }

            width += 1;
        }
        height += 1;

        Self {
            width,
            height,
            antennas,
        }
    }

    fn antenna_pairs(&self) -> impl Iterator<Item = (Point, Point)> + '_ {
        self.antennas.iter().flat_map(|g| {
            g.iter()
                .copied()
                .enumerate()
                .flat_map(|(idx, a)| g.iter().copied().skip(idx + 1).map(move |b| (a, b)))
        })
    }

    fn part_one(&self) -> usize {
        let mut antinodes = HashSet::new();
        for (a, b) in self.antenna_pairs() {
            let a_diff = a - b;
            let b_diff = b - a;

            let a_node = a + a_diff;
            if a_node.in_bounds(self.width, self.height) {
                antinodes.insert(a_node);
            }

            let b_node = b + b_diff;
            if b_node.in_bounds(self.width, self.height) {
                antinodes.insert(b_node);
            }
        }

        antinodes.len()
    }

    fn part_two(&self) -> usize {
        let mut antinodes = HashSet::new();
        for (a, b) in self.antenna_pairs() {
            let a_diff = a - b;
            let b_diff = b - a;

            let mut a_node = a;
            while a_node.in_bounds(self.width, self.height) {
                antinodes.insert(a_node);
                a_node += a_diff;
            }

            let mut b_node = b;
            while b_node.in_bounds(self.width, self.height) {
                antinodes.insert(b_node);
                b_node += b_diff;
            }
        }

        antinodes.len()
    }
}

fn antenna_idx(b: u8) -> Option<usize> {
    let v = match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'z' => Some(b - b'a' + 10),
        b'A'..=b'Z' => Some(b - b'A' + 36),
        _ => None,
    };

    v.map(|b| b as usize)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn in_bounds(&self, width: i32, height: i32) -> bool {
        self.0 >= 0 && self.0 < width && self.1 >= 0 && self.1 < height
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, Point(b_x, b_y): Self) -> Self::Output {
        let Point(a_x, a_y) = self;
        Point(a_x + b_x, a_y + b_y)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, Point(b_x, b_y): Self) -> Self::Output {
        let Point(a_x, a_y) = self;
        Point(a_x - b_x, a_y - b_y)
    }
}

#[test]
fn test() {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    assert_eq!(14, part_one(input));
    assert_eq!(34, part_two(input));
}
