pub fn part_one(input: &str) -> u64 {
    let target = Target::new(input);

    let max_y_step = target.y_min.abs().max(target.y_max.abs());
    let mut result_y = 0;
    for y in target.y_min..=max_y_step {
        for x in 1..=target.x_max {
            let launcher = Launcher::new(x, y);
            let mut max_y = 0;
            for (x, y) in launcher.take_while(|(x, y)| !target.missed(*x, *y)) {
                max_y = max_y.max(y);
                if target.contains(x, y) {
                    result_y = result_y.max(max_y)
                }
            }
        }
    }

    result_y as u64
}

pub fn part_two(input: &str) -> u64 {
    let target = Target::new(input);

    let max_y_step = target.y_min.abs().max(target.y_max.abs());
    let mut solutions = 0;
    for y in target.y_min..=max_y_step {
        'x: for x in 1..=target.x_max {
            let launcher = Launcher::new(x, y);
            for (x, y) in launcher.take_while(|(x, y)| !target.missed(*x, *y)) {
                if target.contains(x, y) {
                    solutions += 1;
                    continue 'x;
                }
            }
        }
    }

    solutions
}

struct Target {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Target {
    fn new(input: &str) -> Self {
        let mut parts = input
            .trim()
            .split(|c| match c {
                '0'..='9' => false,
                '-' => false,
                _ => true,
            })
            .filter(|s| !s.is_empty())
            .filter_map(|n| n.parse::<i64>().ok());

        let x_min = parts.next().unwrap();
        let x_max = parts.next().unwrap();
        let y_min = parts.next().unwrap();
        let y_max = parts.next().unwrap();

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }

    fn missed(&self, x: i64, y: i64) -> bool {
        x > self.x_max || y < self.y_min
    }
}

struct Launcher {
    x_velocity: i64,
    y_velocity: i64,
    x: i64,
    y: i64,
}

impl Launcher {
    fn new(x_velocity: i64, y_velocity: i64) -> Self {
        Self {
            x_velocity,
            y_velocity,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for Launcher {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        self.x_velocity -= self.x_velocity.signum();
        self.y_velocity -= 1;

        Some((self.x, self.y))
    }
}

#[test]
fn test() {
    let input = r#"target area: x=20..30, y=-10..-5"#;

    assert_eq!(45, part_one(input));
    assert_eq!(112, part_two(input));
}
