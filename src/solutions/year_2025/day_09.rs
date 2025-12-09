use std::cmp::Reverse;

pub fn part_one(input: &str) -> u64 {
    let points: Vec<_> = input.trim().lines().filter_map(Point::parse).collect();
    let mut max = 0;

    for (a_idx, a) in points.iter().enumerate() {
        for b in points.iter().skip(a_idx + 1) {
            max = a.area(b).max(max);
        }
    }

    max
}

pub fn part_two(input: &str) -> u64 {
    let points: Vec<_> = input.trim().lines().filter_map(Point::parse).collect();
    let mut lines = Vec::with_capacity(points.len());

    for pair in points.windows(2) {
        if let Some(line) = Line::new(pair[0], pair[1]) {
            lines.push(line);
        }
    }

    if let Some((&a, &b)) = points.first().zip(points.last()) {
        if let Some(line) = Line::new(a, b) {
            lines.push(line);
        }
    }

    lines.sort_unstable_by_key(|l| Reverse(l.len()));

    let mut max = 0;

    for (a_idx, &a) in points.iter().enumerate() {
        'next: for &b in points.iter().skip(a_idx + 1) {
            let area = a.area(&b);
            if area <= max {
                continue;
            }

            let rect = Rect::new(a, b);

            for line in lines.iter() {
                if rect.intersects(line) {
                    continue 'next;
                }
            }

            max = area;
        }
    }

    max
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn parse(line: &str) -> Option<Self> {
        let mut parts = [0; 2];
        let mut idx = 0;

        for &b in line.as_bytes() {
            match b {
                b',' => {
                    if idx == 1 {
                        return None;
                    }
                    idx += 1
                }
                b'0'..=b'9' => {
                    parts[idx] *= 10;
                    parts[idx] += (b - b'0') as i64;
                }
                _ => {
                    return None;
                }
            }
        }

        Some(Point::new(parts[0], parts[1]))
    }

    fn min(&self, other: &Self) -> Self {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(&self, other: &Self) -> Self {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[derive(Debug, Copy, Clone)]
enum Line {
    Vert { x: i64, y0: i64, y1: i64 },
    Horz { y: i64, x0: i64, x1: i64 },
}

impl Line {
    fn new(a: Point, b: Point) -> Option<Self> {
        let line = if a.x == b.x {
            Line::Vert {
                x: a.x,
                y0: a.y.min(b.y),
                y1: a.y.max(b.y),
            }
        } else if a.y == b.y {
            Line::Horz {
                y: a.y,
                x0: a.x.min(b.x),
                x1: a.x.max(b.x),
            }
        } else {
            return None;
        };

        Some(line)
    }

    fn len(&self) -> u64 {
        match self {
            Line::Vert { y0, y1, .. } => y0.abs_diff(*y1),
            Line::Horz { x0, x1, .. } => x0.abs_diff(*x1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    min: Point,
    max: Point,
}

impl Rect {
    fn new(a: Point, b: Point) -> Self {
        let min = a.min(&b);
        let max = a.max(&b);

        Self { min, max }
    }

    fn intersects(&self, line: &Line) -> bool {
        match line {
            &Line::Vert { x, y0, y1 } => {
                let in_x = x > self.min.x && x < self.max.x;
                let around_y = y0 < self.min.y && y1 > self.max.y;
                let inside_y = y0 > self.min.y && y1 < self.max.y;
                let in_min = self.min.y > y0 && self.min.y < y1;
                let in_max = self.max.y > y0 && self.max.y < y1;

                let edge = if y0 == self.min.y || y1 == self.max.y {
                    in_x
                } else {
                    false
                };

                edge || in_x && (around_y || inside_y || in_min || in_max)
            }
            &Line::Horz { y, x0, x1 } => {
                let in_y = y > self.min.y && y < self.max.y;
                let around_x = x0 < self.min.x && x1 > self.max.x;
                let inside_x = x0 > self.min.x && x1 < self.max.x;
                let in_min = self.min.x > x0 && self.min.x < x1;
                let in_max = self.max.x > x0 && self.max.x < x1;

                let edge = if x0 == self.min.x || x1 == self.max.x {
                    in_y
                } else {
                    false
                };

                edge || in_y && (around_x || inside_x || in_min || in_max)
            }
        }
    }
}

#[test]
fn test() {
    let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    assert_eq!(50, part_one(input));
    assert_eq!(24, part_two(input));
}
