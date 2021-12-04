pub fn part_one(input: &str) -> u64 {
    let actions: Vec<_> = input
        .lines()
        .filter_map(|l| {
            let n = l[1..].trim().parse().ok();

            if let Some(n) = n {
                use ShipActions::*;
                let v = match &l[0..1] {
                    "N" => North(n),
                    "S" => South(n),
                    "E" => East(n),
                    "W" => West(n),
                    "L" => Left(n),
                    "R" => Right(n),
                    "F" => Foreward(n),
                    _ => return None,
                };
                Some(v)
            } else {
                None
            }
        })
        .collect();

    let mut x = 0;
    let mut y = 0;

    let mut facing = 90;

    for action in actions {
        use ShipActions::*;
        match action {
            North(n) => y -= n,
            South(n) => y += n,
            East(n) => x += n,
            West(n) => x -= n,
            Left(n) => facing -= n,
            Right(n) => facing += n,
            Foreward(n) => match (facing % 360) / 90 {
                0 => y -= n,
                1 => x += n,
                2 => y += n,
                3 => x -= n,
                _ => panic!("bad facing {}", facing),
            },
        }
    }

    (x.abs() + y.abs()) as u64
}

pub fn part_two(input: &str) -> u64 {
    let actions: Vec<_> = input
        .lines()
        .filter_map(|l| {
            let n = l[1..].trim().parse().ok();

            if let Some(n) = n {
                use ShipActions::*;
                let v = match &l[0..1] {
                    "N" => North(n),
                    "S" => South(n),
                    "E" => East(n),
                    "W" => West(n),
                    "L" => Left(n),
                    "R" => Right(n),
                    "F" => Foreward(n),
                    _ => return None,
                };
                Some(v)
            } else {
                None
            }
        })
        .collect();

    let mut x = 0;
    let mut y = 0;

    let mut way_x = 10;
    let mut way_y = -1;

    for action in actions {
        use ShipActions::*;
        match action {
            North(n) => way_y -= n,
            South(n) => way_y += n,
            East(n) => way_x += n,
            West(n) => way_x -= n,
            Left(n) => {
                let count = n / 90;

                for _ in 0..count {
                    let temp = way_x;
                    way_x = way_y;
                    way_y = -temp;
                }
            }
            Right(n) => {
                let count = n / 90;

                for _ in 0..count {
                    let temp = way_x;
                    way_x = -way_y;
                    way_y = temp;
                }
            }
            Foreward(n) => {
                x += way_x * n;
                y += way_y * n;
            }
        }
    }

    (x.abs() + y.abs()) as u64
}

#[derive(Debug, Copy, Clone)]
enum ShipActions {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Foreward(i32),
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"F10
N3
F7
R90
F11"#;

    run_a(i, 25);
    run_b(i, 286);
}
