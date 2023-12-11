enum SubCommand {
    Forward(u64),
    Up(u64),
    Down(u64),
}

struct SubCommandErr;

impl std::str::FromStr for SubCommand {
    type Err = SubCommandErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split_once(' ')
            .and_then(|(dir, dist)| Some(dir).zip(dist.parse().ok()));

        if let Some((direction, distance)) = parts {
            let res = match direction {
                "forward" => SubCommand::Forward(distance),
                "down" => SubCommand::Down(distance),
                "up" => SubCommand::Up(distance),
                _ => return Err(SubCommandErr),
            };

            Ok(res)
        } else {
            Err(SubCommandErr)
        }
    }
}

pub fn part_one(input: &str) -> u64 {
    let lines = input
        .trim()
        .lines()
        .map(|s| s.parse::<SubCommand>())
        .filter_map(Result::ok);

    let mut horizontal = 0;
    let mut depth = 0;

    for command in lines {
        match command {
            SubCommand::Forward(distance) => horizontal += distance,
            SubCommand::Up(distance) => depth -= distance,
            SubCommand::Down(distance) => depth += distance,
        }
    }

    horizontal * depth
}

pub fn part_two(input: &str) -> u64 {
    let lines = input
        .trim()
        .lines()
        .map(|s| s.parse::<SubCommand>())
        .filter_map(Result::ok);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in lines {
        match command {
            SubCommand::Forward(distance) => {
                horizontal += distance;
                depth += aim * distance
            }
            SubCommand::Up(distance) => aim -= distance,
            SubCommand::Down(distance) => aim += distance,
        }
    }

    horizontal * depth
}

#[test]
fn test() {
    let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    assert_eq!(150, part_one(input));
    assert_eq!(900, part_two(input));
}
