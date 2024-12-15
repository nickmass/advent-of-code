pub fn part_one(input: &str) -> u64 {
    solve_part_one::<101, 103>(input)
}

pub fn part_two(input: &str) -> i32 {
    let robots: Vec<_> = input
        .trim()
        .lines()
        .map(|l| l.parse::<Robot>().unwrap())
        .collect();

    let mut set = crate::HashSet::new();
    'outer: for steps in 0.. {
        set.clear();
        for bot in &robots {
            if !set.insert(bot.step(101, 103, steps)) {
                continue 'outer;
            }
        }

        return steps;
    }

    unreachable!()
}

fn solve_part_one<const WIDTH: i32, const HEIGHT: i32>(input: &str) -> u64 {
    let quads = input
        .trim()
        .lines()
        .map(|l| l.parse::<Robot>().unwrap())
        .map(|r| r.step(WIDTH, HEIGHT, 100))
        .filter_map(|d| d.quadrant(WIDTH, HEIGHT));

    let mut counts = [0; 4];

    for quad in quads {
        counts[quad] += 1;
    }

    counts.into_iter().product()
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn step(&self, width: i32, height: i32, steps: i32) -> Point {
        let bounds = Point::new(width, height);
        let offset = self.velocity * steps;
        let Point { x, y } = (self.position + offset) % bounds;
        let x = if x < 0 { x + width } else { x };
        let y = if y < 0 { y + height } else { y };

        Point::new(x, y)
    }
}

#[derive(Debug, Copy, Clone)]
struct ParseErr;

impl std::str::FromStr for Robot {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = s.get("p=".len()..).ok_or(ParseErr)?;
        let (p, v) = s.split_once(" v=").ok_or(ParseErr)?;
        let position = p.parse().map_err(|_| ParseErr)?;
        let velocity = v.parse().map_err(|_| ParseErr)?;

        Ok(Robot { position, velocity })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn quadrant(&self, width: i32, height: i32) -> Option<usize> {
        if (self.x * 2) + 1 == width || (self.y * 2) + 1 == height {
            None
        } else {
            let v = match (self.x > width / 2, self.y > height / 2) {
                (false, false) => 0,
                (false, true) => 1,
                (true, false) => 2,
                (true, true) => 3,
            };

            Some(v)
        }
    }
}

impl std::str::FromStr for Point {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or(ParseErr)?;
        let x = x.parse().map_err(|_| ParseErr)?;
        let y = y.parse().map_err(|_| ParseErr)?;

        Ok(Point { x, y })
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, Point { x, y }: Self) -> Self::Output {
        Point::new(self.x + x, self.y + y)
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Rem for Point {
    type Output = Self;

    fn rem(self, Point { x, y }: Self) -> Self::Output {
        Point::new(self.x % x, self.y % y)
    }
}

#[test]
fn test() {
    let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

    assert_eq!(12, solve_part_one::<11, 7>(input));
}
