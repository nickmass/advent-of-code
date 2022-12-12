use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    let grid = Grid::new(input);

    grid.search()
}

pub fn part_two(input: &str) -> usize {
    let grid = Grid::new(input);

    grid.search_wide()
}

struct Grid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
    start: Point,
    minimums: Vec<Point>,
    end: Point,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut start = Point::default();
        let mut end = Point::default();
        let mut y = 0;
        let mut minimums = Vec::new();

        for line in input.lines() {
            height += 1;
            if width == 0 {
                width = line.len();
            }

            let mut x = 0;

            for c in line.bytes() {
                let value = match c {
                    b'a'..=b'z' => c - b'a',
                    b'S' => {
                        start = Point::new(x, y);
                        0
                    }
                    b'E' => {
                        end = Point::new(x, y);
                        b'z' - b'a'
                    }
                    _ => unreachable!(),
                };

                if value == 0 {
                    minimums.push(Point::new(x, y));
                }

                cells.push(value);
                x += 1;
            }
            y += 1;
        }

        Self {
            cells,
            width,
            height,
            start,
            end,
            minimums,
        }
    }

    fn get(&self, p: Point) -> Option<u8> {
        if p.x < 0 || p.x >= self.width as isize || p.y < 0 || p.y >= self.height as isize {
            return None;
        }

        let x = p.x as usize;
        let y = p.y as usize;

        self.cells.get(y * self.width + x).copied()
    }

    fn search(&self) -> usize {
        let mut visited = HashMap::new();
        self.do_search(self.start, &mut visited)
    }

    fn search_wide(&self) -> usize {
        let mut min_count = usize::MAX;
        let mut visited = HashMap::new();

        for &pos in self.minimums.iter() {
            let count = self.do_search(pos, &mut visited);
            min_count = min_count.min(count);
        }

        min_count
    }

    fn do_search(&self, position: Point, visited: &mut HashMap<Point, usize>) -> usize {
        let mut haystack = Vec::new();
        let mut min_count = usize::MAX;
        haystack.extend(self.neighbors(position, 0));

        while let Some((next, count)) = haystack.pop() {
            if next == self.end {
                min_count = min_count.min(count);
                continue;
            }

            for (neighbor, count) in self.neighbors(next, count) {
                if let Some(&old_count) = visited.get(&neighbor) {
                    if old_count <= count {
                        continue;
                    }
                }

                visited.insert(neighbor, count);
                haystack.push((neighbor, count))
            }
        }

        min_count
    }

    fn neighbors(&self, point: Point, count: usize) -> impl Iterator<Item = (Point, usize)> {
        let count = count + 1;

        let me = self.get(point).unwrap();

        let left = point.adjust(1, 0);
        let right = point.adjust(-1, 0);
        let up = point.adjust(0, 1);
        let down = point.adjust(0, -1);

        self.get(left)
            .filter(|v| *v <= me + 1)
            .map(|_| (left, count))
            .into_iter()
            .chain(
                self.get(right)
                    .filter(|v| *v <= me + 1)
                    .map(|_| (right, count)),
            )
            .chain(self.get(up).filter(|v| *v <= me + 1).map(|_| (up, count)))
            .chain(
                self.get(down)
                    .filter(|v| *v <= me + 1)
                    .map(|_| (down, count)),
            )
            .filter(move |_| me != 255)
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn adjust(&self, x: isize, y: isize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[test]
fn test() {
    let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    assert_eq!(31, part_one(input));
    assert_eq!(29, part_two(input));
}
