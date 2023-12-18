use std::collections::BTreeSet;

pub fn part_one(input: &str) -> i64 {
    solve(input, parse_part_one)
}
pub fn part_two(input: &str) -> i64 {
    solve(input, parse_part_two)
}

fn solve(input: &str, parse: fn(&str) -> (Direction, i64)) -> i64 {
    let mut segments = Vec::new();
    let mut horz_lines = BTreeSet::new();

    let mut moves = input
        .trim()
        .lines()
        .map(parse)
        .enumerate()
        .cycle()
        .peekable();

    let mut point = Point(0, 0);
    let mut min = Point(i64::MAX, i64::MAX);
    let mut max = Point(i64::MIN, i64::MIN);
    let mut prev_dir = None;

    while let Some((idx, (dir, dist))) = moves.next() {
        let Some(prev) = prev_dir else {
            prev_dir = Some(dir);
            continue;
        };

        let end = dir.jump(point, dist);

        min = min.min(point);
        max = max.max(point);

        let x = point.0;
        let y = point.1;

        let y1 = point.1.min(end.1);
        let y2 = point.1.max(end.1);
        let x1 = point.0.min(end.0);
        let x2 = point.0.max(end.0);

        let segment = match dir {
            Direction::Left | Direction::Right => {
                let barrier = Some(prev) == moves.peek().map(|(_, (d, _))| *d);
                horz_lines.insert(y);
                Segment::Horz(HorzSegment { y, x1, x2, barrier })
            }
            Direction::Up | Direction::Down => Segment::Vert(VertSegment { x, y1, y2 }),
        };

        segments.push(segment);

        prev_dir = Some(dir);
        point = end;

        if idx == 0 {
            break;
        }
    }

    segments.sort_unstable();

    let mut count = 0;
    let mut y = min.1;
    while y <= max.1 {
        let mut line_count = 0;
        let mut x = min.0 - 1;
        let mut inside = false;
        let mut skip_to = None;
        for seg in segments.iter() {
            match seg {
                Segment::Horz(h) => {
                    if y == h.y {
                        if !h.barrier && !inside {
                            line_count += (h.x2 - h.x1) - 1;
                        } else if h.barrier && inside {
                            inside = false;
                            line_count += h.x2 - h.x1;
                        } else if h.barrier && !inside {
                            inside = true;
                            x += 1;
                        }
                    }
                }
                Segment::Vert(v) => {
                    if v.intersects(y) {
                        if inside {
                            line_count += (v.x - x) + 1;
                        }
                        x = v.x;
                        inside = !inside;
                        skip_to = Some(skip_to.unwrap_or(v.y2).min(v.y2));
                    } else if y < v.y1 {
                        skip_to = Some(skip_to.unwrap_or(v.y1).min(v.y1));
                    }
                }
            }
        }

        if inside {
            panic!("still inside shape at end of line")
        }

        let horz_skip = horz_lines.range(y..).next();

        skip_to = match (skip_to, horz_skip) {
            (None, None) => None,
            (None, Some(&h)) => Some(h),
            (Some(s), None) => Some(s),
            (Some(s), Some(&h)) => Some(s.min(h)),
        };

        if let Some(skip) = skip_to.filter(|&skip| skip > y) {
            count += line_count * y.abs_diff(skip) as i64;
            y = skip;
        } else {
            count += line_count;
            y += 1;
        }
    }

    count
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Segment {
    Horz(HorzSegment),
    Vert(VertSegment),
}

impl Segment {
    fn min_x(&self) -> i64 {
        match self {
            Segment::Horz(h) => h.x1,
            Segment::Vert(v) => v.x,
        }
    }
}

impl std::cmp::Ord for Segment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ordering = self.min_x().cmp(&other.min_x());

        match (ordering, self, other) {
            (std::cmp::Ordering::Equal, Segment::Horz(_), Segment::Vert(_)) => {
                std::cmp::Ordering::Greater
            }
            (std::cmp::Ordering::Equal, Segment::Vert(_), Segment::Horz(_)) => {
                std::cmp::Ordering::Less
            }
            _ => ordering,
        }
    }
}

impl std::cmp::PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct HorzSegment {
    y: i64,
    x1: i64,
    x2: i64,
    barrier: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct VertSegment {
    x: i64,
    y1: i64,
    y2: i64,
}

impl VertSegment {
    fn intersects(&self, y: i64) -> bool {
        y >= self.y1 && y <= self.y2
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i64, i64);

impl Point {
    fn min(&self, Point(x2, y2): Point) -> Point {
        let Point(x1, y1) = *self;

        Point(x1.min(x2), y1.min(y2))
    }

    fn max(&self, Point(x2, y2): Point) -> Point {
        let Point(x1, y1) = *self;

        Point(x1.max(x2), y1.max(y2))
    }
}

fn parse_part_one(line: &str) -> (Direction, i64) {
    let (dir, rest) = line.split_once(" ").unwrap();
    let (dist, _color) = rest.split_once(" ").unwrap();

    let dir = match dir {
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "U" => Direction::Up,
        _ => unreachable!(),
    };

    let dist = dist.parse::<i64>().unwrap();
    (dir, dist)
}

fn parse_part_two(line: &str) -> (Direction, i64) {
    let (_, color) = line.split_once("#").unwrap();
    let dist = &color[0..5];

    let mut value = 0;
    for digit in dist.chars() {
        value *= 0x10;
        value += digit.to_digit(16).unwrap() as i64;
    }

    let dir = match &color[5..6] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => unreachable!(),
    };

    (dir, value)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn offset(&self) -> Point {
        match *self {
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
            Direction::Up => Point(0, -1),
            Direction::Down => Point(0, 1),
        }
    }

    fn jump(&self, Point(x, y): Point, dist: i64) -> Point {
        let Point(d_x, d_y) = self.offset();

        Point(x + (d_x * dist), y + (d_y * dist))
    }
}

#[test]
fn test() {
    let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

    assert_eq!(62, part_one(input));
    assert_eq!(952408144115, part_two(input));
}
