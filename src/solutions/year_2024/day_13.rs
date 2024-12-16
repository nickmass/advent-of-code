pub fn part_one(input: &str) -> i64 {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .filter_map(|s| solve::<0>(&s))
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .filter_map(|s| solve::<10000000000000>(&s))
        .sum()
}

fn solve<const OFFSET: i64>(scenario: &Scenario) -> Option<i64> {
    let offset = Point::new(OFFSET, OFFSET);
    let target = scenario.target + offset;

    let de = (scenario.button_a.offset.y * scenario.button_b.offset.x)
        - (scenario.button_a.offset.x * scenario.button_b.offset.y);

    if de == 0 {
        return None;
    }

    let a_num = (scenario.button_b.offset.x * target.y) - (scenario.button_b.offset.y * target.x);
    let b_num = (scenario.button_a.offset.y * target.x) - (scenario.button_a.offset.x * target.y);

    let a = a_num / de;
    let b = b_num / de;

    let presses = Presses::new(a, b);
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
            target: Point::new(x, y),
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
            offset: Point::new(x, y),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, Point { x, y }: Self) -> Self::Output {
        Point::new(self.x + x, self.y + y)
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        let Point { x, y } = self;
        Point::new(x * rhs, y * rhs)
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
