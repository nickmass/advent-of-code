pub fn part_one(input: &str) -> u64 {
    let moves = input.trim().lines().filter_map(parse_rotation);

    let mut pos = 50;
    let mut count = 0;

    for m in moves {
        pos += m;
        pos %= 100;

        if pos == 0 {
            count += 1;
        }
    }

    count
}

pub fn part_two(input: &str) -> u64 {
    let moves = input.trim().lines().filter_map(parse_rotation);

    let mut pos = 50;
    let mut count = 0;

    for m in moves {
        for _ in 0..m.abs() {
            pos += m.signum();

            if pos >= 100 || pos <= -100 {
                pos %= 100;
            }

            if pos == 0 {
                count += 1;
            }
        }
    }

    count
}

fn parse_rotation(line: &str) -> Option<i64> {
    if line.len() < 2 {
        return None;
    }

    let direction = if line.starts_with('L') { 1 } else { -1 };
    let value = line[1..].parse::<i64>().ok()?;
    Some(value * direction)
}

#[test]
fn test() {
    let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    assert_eq!(3, part_one(input));
    assert_eq!(6, part_two(input));
}
