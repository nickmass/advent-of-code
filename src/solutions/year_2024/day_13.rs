pub fn part_one(input: &str) -> i64 {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .filter_map(|s| solve_one(&s))
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .filter_map(|s| solve_two(&s))
        .sum()
}

fn solve_one(scenario: &Scenario) -> Option<i64> {
    let max_b = scenario.target.min_div(scenario.button_b.offset);
    let max = if max_b > 100 { 100 } else { max_b };

    for b in (0..=max).rev() {
        let pos = Presses::new(0, b).position(scenario);
        let diff = scenario.target - pos;

        let a = diff.min_div(scenario.button_a.offset);
        if a > 100 {
            continue;
        }
        let m = Presses::new(a, b);
        if m.position(scenario) == scenario.target {
            return Some(m.cost());
        }
    }

    None
}

fn solve_two(scenario: &Scenario) -> Option<i64> {
    let offset = Point(10000000000000, 10000000000000);
    let target = scenario.target + offset;

    let de = (scenario.button_a.offset.1 * scenario.button_b.offset.0)
        - (scenario.button_a.offset.0 * scenario.button_b.offset.1);

    let x_num = (scenario.button_b.offset.0 * target.1) - (scenario.button_b.offset.1 * target.0);
    let y_num = (scenario.button_a.offset.1 * target.0) - (scenario.button_a.offset.0 * target.1);

    let x = x_num / de;
    let y = y_num / de;

    let presses = Presses::new(x, y);
    if presses.position(scenario) == target {
        Some(presses.cost())
    } else {
        None
    }
}

#[derive(Debug, Copy, Clone)]
struct Presses {
    a: i64,
    b: i64,
}

impl Presses {
    fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    fn position(&self, scenario: &Scenario) -> Point {
        let a = scenario.button_a.offset * self.a;
        let b = scenario.button_b.offset * self.b;

        a + b
    }

    fn cost(&self) -> i64 {
        3 * self.a + self.b
    }
}

#[derive(Debug, Copy, Clone)]
struct Scenario {
    button_a: Button,
    button_b: Button,
    target: Point,
}

#[derive(Debug, Copy, Clone)]
struct ParseErr;

impl std::str::FromStr for Scenario {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut lines = s.lines();
        let button = lines.next().ok_or(ParseErr)?;
        let button_a = button.parse()?;
        let button = lines.next().ok_or(ParseErr)?;
        let button_b = button.parse()?;
        let target = lines.next().ok_or(ParseErr)?;
        let target = target.get("Prize: X=".len()..).ok_or(ParseErr)?;
        let (x, y) = target.split_once(", Y=").ok_or(ParseErr)?;

        let x = x.parse().map_err(|_| ParseErr)?;
        let y = y.parse().map_err(|_| ParseErr)?;

        Ok(Scenario {
            button_a,
            button_b,
            target: Point(x, y),
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Button {
    offset: Point,
}

impl std::str::FromStr for Button {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = s.get("Button A: X+".len()..).ok_or(ParseErr)?;
        let (x, y) = s.split_once(", Y+").ok_or(ParseErr)?;

        let x = x.parse().map_err(|_| ParseErr)?;
        let y = y.parse().map_err(|_| ParseErr)?;

        Ok(Button {
            offset: Point(x, y),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point(i64, i64);

impl Point {
    fn min_div(self, other: Point) -> i64 {
        let x = self.0 / other.0;
        let y = self.1 / other.1;

        x.min(y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, Point(x, y): Self) -> Self::Output {
        Point(self.0 + x, self.1 + y)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, Point(x, y): Self) -> Self::Output {
        Point(self.0 - x, self.1 - y)
    }
}

impl std::ops::Mul for Point {
    type Output = Self;

    fn mul(self, Point(x, y): Self) -> Self::Output {
        Point(self.0 * x, self.1 * y)
    }
}

impl std::ops::Div for Point {
    type Output = Self;

    fn div(self, Point(x, y): Self) -> Self::Output {
        Point(self.0 / x, self.1 / y)
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        let Point(x, y) = self;
        Point(x * rhs, y * rhs)
    }
}

#[test]
fn test() {
    let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

    assert_eq!(480, part_one(input));
}
