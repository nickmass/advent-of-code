use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    solve::<2>(input)
}

pub fn part_two(input: &str) -> usize {
    solve::<10>(input)
}

fn solve<const N: usize>(input: &str) -> usize {
    let moves = input.trim().lines().filter_map(|l| l.parse::<Move>().ok());

    let mut knots = [Point::default(); N];
    let mut visited = HashSet::new();

    visited.insert(knots[N - 1]);
    for mov in moves {
        for _ in 0..mov.count() {
            knots[0] = knots[0] + mov;
            for idx in 1..N {
                knots[idx] = knots[idx].follow(knots[idx - 1]);
            }

            visited.insert(knots[N - 1]);
        }
    }

    visited.len()
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn follow(self, other: Self) -> Self {
        let (dx, dy) = self.diff(other);

        if dx.abs() > 1 || dy.abs() > 1 {
            self.adjust(dx.signum(), dy.signum())
        } else {
            self
        }
    }

    fn diff(self, other: Self) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
    }

    fn adjust(self, x: i32, y: i32) -> Self {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl std::ops::Add<Move> for Point {
    type Output = Self;

    fn add(self, rhs: Move) -> Self::Output {
        match rhs {
            Move::Left(_) => Point {
                x: self.x - 1,
                y: self.y,
            },
            Move::Right(_) => Point {
                x: self.x + 1,
                y: self.y,
            },
            Move::Up(_) => Point {
                x: self.x,
                y: self.y - 1,
            },
            Move::Down(_) => Point {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

impl Move {
    fn count(&self) -> i32 {
        match self {
            Move::Left(n) => *n,
            Move::Right(n) => *n,
            Move::Up(n) => *n,
            Move::Down(n) => *n,
        }
    }
}

struct MoveParseErr;

impl std::str::FromStr for Move {
    type Err = MoveParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s
            .split_once(" ")
            .and_then(|(d, v)| v.parse::<i32>().ok().map(|v| (d, v)))
            .ok_or(MoveParseErr)?
        {
            ("R", v) => Move::Right(v),
            ("L", v) => Move::Left(v),
            ("U", v) => Move::Up(v),
            ("D", v) => Move::Down(v),
            _ => return Err(MoveParseErr),
        };

        Ok(value)
    }
}

#[test]
fn test() {
    let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    assert_eq!(13, part_one(input));
    assert_eq!(1, part_two(input));

    let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    assert_eq!(36, part_two(input));
}
