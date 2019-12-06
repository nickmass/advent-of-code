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
    pub fn move_iter<I: Iterator<Item = Movement>>(input: I) -> PointIterator<I> {
        PointIterator {
            inner: input,
            item: None,
            counter: 0,
            point: Point { x: 0, y: 0 },
        }
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

pub struct PointIterator<I: Iterator<Item = Movement>> {
    inner: I,
    item: Option<Movement>,
    counter: i32,
    point: Point,
}

impl<I: Iterator<Item = Movement>> PointIterator<I> {
    pub fn stepped(self) -> impl Iterator<Item = SteppedPoint> {
        self.enumerate().map(|(steps, inner)| SteppedPoint {
            inner,
            steps: steps + 1,
        })
    }
}

impl<I: Iterator<Item = Movement>> Iterator for PointIterator<I> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.item.take().or_else(|| self.inner.next());

        if let Some(item) = item {
            self.counter += 1;
            let offset = match item {
                Movement::Left(offset) => {
                    self.point.x -= 1;
                    offset
                }
                Movement::Right(offset) => {
                    self.point.x += 1;
                    offset
                }
                Movement::Up(offset) => {
                    self.point.y -= 1;
                    offset
                }
                Movement::Down(offset) => {
                    self.point.y += 1;
                    offset
                }
            };

            if offset == self.counter {
                self.item = None;
                self.counter = 0;
            } else {
                self.item = Some(item);
            }

            Some(self.point.clone())
        } else {
            None
        }
    }
}

pub struct SteppedPoint {
    inner: Point,
    pub steps: usize,
}

impl std::hash::Hash for SteppedPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl std::cmp::PartialEq for SteppedPoint {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl std::cmp::Eq for SteppedPoint {}
