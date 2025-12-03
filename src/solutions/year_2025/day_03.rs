pub fn part_one(input: &str) -> u64 {
    solve::<2>(input)
}

pub fn part_two(input: &str) -> u64 {
    solve::<12>(input)
}

fn solve<const TARGET_LEN: usize>(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut total = 0;
    for line in lines {
        assert!(line.len() >= TARGET_LEN);
        let mut values = [None; TARGET_LEN];

        for (position, &b) in line.as_bytes().iter().enumerate() {
            let n = Some(b - b'0');

            let remaining = line.len() - position;
            let start_idx = TARGET_LEN.saturating_sub(remaining);

            for idx in start_idx..values.len() {
                if n > values[idx] {
                    values[idx] = n;
                    values[idx + 1..].fill(None);
                    break;
                }
            }
        }

        let mut value = 0;
        for v in values {
            value *= 10;
            value += v.expect("found all digits") as u64;
        }

        total += value;
    }

    total
}

#[test]
fn test() {
    let input = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

    assert_eq!(357, part_one(input));
    assert_eq!(3121910778619, part_two(input));
}
