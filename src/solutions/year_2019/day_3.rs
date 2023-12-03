use std::str::FromStr;

pub fn part_one(input: &str) -> i32 {
    let mut lines = input.trim().split('\n');

    let first_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(Movement::from_str)
        .filter_map(Result::ok);
    let second_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(Movement::from_str)
        .filter_map(Result::ok);

    let first_segments = LineIter::new(first_line);
    let second_segments: Vec<_> = LineIter::new(second_line).collect();

    let intersections =
        first_segments.filter_map(|(_, f)| second_segments.iter().find_map(|(_, s)| f.crosses(s)));

    intersections.map(|p| p.distance()).min().unwrap_or(0)
}

pub fn part_two(input: &str) -> i32 {
    let mut lines = input.trim().split('\n');

    let first_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(Movement::from_str)
        .filter_map(Result::ok);
    let second_line = lines
        .next()
        .unwrap()
        .split(',')
        .map(Movement::from_str)
        .filter_map(Result::ok);

    let first_segments = LineIter::new(first_line);
    let second_segments: Vec<_> = LineIter::new(second_line)
        .scan(0, |distance, (p, l)| {
            *distance += l.length();
            Some((*distance, p, l))
        })
        .collect();

    let mut min_distance = std::i32::MAX;

    let mut fl_distance = 0;
    for (fl_start, fl) in first_segments {
        for (sl_distance, sl_start, sl) in &second_segments {
            let total_distance = sl_distance + fl_distance;

            if let Some(cross) = fl.crosses(sl) {
                let distance = total_distance + fl_start.distance_to(&cross)
                    - (sl.length() - sl_start.distance_to(&cross));
                if distance < min_distance {
                    min_distance = distance;
                    break;
                }
            }

            if total_distance > min_distance {
                break;
            }
        }
        fl_distance += fl.length();
    }

    min_distance
}

#[derive(Debug, Clone)]
pub enum Line {
    EastWest(Segment),
    NorthSouth(Segment),
}

#[derive(Debug, Clone)]
pub struct Segment {
    start: Point,
    end: Point,
}

impl Line {
    pub fn crosses(&self, other: &Self) -> Option<Point> {
        match (self, other) {
            (Line::EastWest(ew), Line::NorthSouth(ns))
            | (Line::NorthSouth(ns), Line::EastWest(ew)) => {
                if ew.start.y > ns.start.y
                    && ew.start.y <= ns.end.y
                    && ns.start.x > ew.start.x
                    && ns.start.x <= ew.end.x
                {
                    Some(Point {
                        x: ns.start.x,
                        y: ew.start.y,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn new(location: &mut Point, movement: Movement) -> Self {
        let old_location = location.clone();
        match movement {
            Movement::Left(offset) => {
                location.x -= offset;
                Line::EastWest(Segment {
                    start: location.clone(),
                    end: old_location,
                })
            }
            Movement::Right(offset) => {
                location.x += offset;
                Line::EastWest(Segment {
                    start: old_location,
                    end: location.clone(),
                })
            }
            Movement::Up(offset) => {
                location.y += offset;
                Line::NorthSouth(Segment {
                    start: old_location,
                    end: location.clone(),
                })
            }
            Movement::Down(offset) => {
                location.y -= offset;
                Line::NorthSouth(Segment {
                    start: location.clone(),
                    end: old_location,
                })
            }
        }
    }

    pub fn length(&self) -> i32 {
        match self {
            Line::EastWest(segment) => segment.end.x - segment.start.x,
            Line::NorthSouth(segment) => segment.end.y - segment.start.y,
        }
    }
}

pub struct LineIter<I: Iterator<Item = Movement>> {
    moves: I,
    location: Point,
}

impl<I: Iterator<Item = Movement>> LineIter<I> {
    pub fn new(moves: I) -> Self {
        LineIter {
            moves,
            location: Point { x: 0, y: 0 },
        }
    }
}

impl<I: Iterator<Item = Movement>> Iterator for LineIter<I> {
    type Item = (Point, Line);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(movement) = self.moves.next() {
            let start = self.location.clone();
            Some((start, Line::new(&mut self.location, movement)))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum Movement {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

pub enum ParseMovementError {
    InvalidOffset,
    InvalidDirection,
}

impl std::str::FromStr for Movement {
    type Err = ParseMovementError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let offset = input
            .get(1..)
            .map(i32::from_str)
            .and_then(Result::ok)
            .ok_or(ParseMovementError::InvalidOffset)?;

        match input.chars().next() {
            Some('L') => Ok(Movement::Left(offset)),
            Some('R') => Ok(Movement::Right(offset)),
            Some('U') => Ok(Movement::Up(offset)),
            Some('D') => Ok(Movement::Down(offset)),
            _ => Err(ParseMovementError::InvalidDirection),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn distance_to(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[test]
fn test() {
    let input = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
"#;

    assert_eq!(159, part_one(input));
    assert_eq!(610, part_two(input));

    let input = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
"#;

    assert_eq!(135, part_one(input));
    assert_eq!(410, part_two(input));
}
