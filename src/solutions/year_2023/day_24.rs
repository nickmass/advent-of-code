pub fn part_one(input: &str) -> u64 {
    solve_part_one::<200000000000000, 400000000000000>(input)
}

fn solve_part_one<const LOW: i128, const HIGH: i128>(input: &str) -> u64 {
    let stones: Vec<_> = input.trim().lines().map(Hailstone::new).collect();
    let mut count = 0;

    for (i, &stone) in stones.iter().enumerate() {
        for &other in stones.iter().skip(i + 1) {
            if stone.intersect::<LOW, HIGH>(other) {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    position: (i128, i128, i128),
    velocity: (i128, i128, i128),
}

impl Hailstone {
    fn new(line: &str) -> Self {
        let (pos, vel) = line.trim().split_once(" @ ").unwrap();
        let mut pos = pos.split(',').map(|n| n.trim().parse::<i128>().unwrap());
        let mut vel = vel.split(',').map(|n| n.trim().parse::<i128>().unwrap());

        let x = pos.next().unwrap();
        let y = pos.next().unwrap();
        let z = pos.next().unwrap();

        let a = vel.next().unwrap();
        let b = vel.next().unwrap();
        let c = vel.next().unwrap();

        let position = (x, y, z);
        let velocity = (a, b, c);

        Self { position, velocity }
    }

    fn intersect<const LOW: i128, const HIGH: i128>(self, other: Self) -> bool {
        let (v_x1, v_y1, _) = self.velocity;
        let (v_x3, v_y3, _) = other.velocity;

        let (x1, y1, _) = self.position;
        let (x2, y2) = (x1 + v_x1, y1 + v_y1);

        let (x3, y3, _) = other.position;
        let (x4, y4) = (x3 + v_x3, y3 + v_y3);

        let d = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));

        if d != 0 {
            let x = ((x1 * y2 - y1 * x2) * (x3 - x4)) - ((x1 - x2) * (x3 * y4 - y3 * x4));
            let y = ((x1 * y2 - y1 * x2) * (y3 - y4)) - ((y1 - y2) * (x3 * y4 - y3 * x4));

            let dd = d.abs();

            let p_x = x * d.signum();
            let p_y = y * d.signum();

            let x1 = x1 * dd;
            let x3 = x3 * dd;
            let y1 = y1 * dd;
            let y3 = y3 * dd;

            let low = LOW * dd;
            let high = HIGH * dd;

            let mut in_future = true;
            in_future &= v_x1 <= 0 || p_x >= x1;
            in_future &= v_x3 <= 0 || p_x >= x3;
            in_future &= v_x1 >= 0 || p_x <= x1;
            in_future &= v_x3 >= 0 || p_x <= x3;
            in_future &= v_y1 <= 0 || p_y >= y1;
            in_future &= v_y3 <= 0 || p_y >= y3;
            in_future &= v_y1 >= 0 || p_y <= y1;
            in_future &= v_y3 >= 0 || p_y <= y3;

            let in_bounds = p_x >= low && p_x <= high && p_y >= low && p_y <= high;

            in_future && in_bounds
        } else {
            false
        }
    }
}

pub fn part_two(_input: &str) -> &'static str {
    "incomplete"
}

#[test]
fn test() {
    let input = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;

    assert_eq!(2, solve_part_one::<7, 27>(input));
    //assert_eq!(47, part_two(input));
}
