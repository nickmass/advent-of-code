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
                if ew.start.y >= ns.start.y
                    && ew.start.y <= ns.end.y
                    && ns.start.x >= ew.start.x
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
