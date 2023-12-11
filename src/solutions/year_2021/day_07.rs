pub fn part_one(input: &str) -> i64 {
    let crabs: Vec<_> = input
        .trim()
        .split(',')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    let mut min_dist = i64::MAX;
    let max_position = crabs.iter().max().copied().unwrap_or(0);
    for n in 0..max_position {
        let mut distance = 0;

        for crab in crabs.iter() {
            distance += (*crab - n as i64).abs()
        }

        min_dist = min_dist.min(distance);
    }

    min_dist
}

pub fn part_two(input: &str) -> i64 {
    let crabs: Vec<_> = input
        .trim()
        .split(',')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    let max_position = crabs.iter().max().copied().unwrap_or(0);
    let mut min_dist = i64::MAX;
    for n in 0..max_position {
        let mut distance = 0;

        for crab in crabs.iter() {
            let steps = (*crab - n as i64).abs();
            let sum = if steps & 1 == 0 {
                (steps / 2) * (steps + 1)
            } else {
                ((steps / 2) * (steps + 1)) + ((steps + 1) / 2)
            };
            distance += sum;
        }

        min_dist = min_dist.min(distance);
    }

    min_dist
}

#[test]
fn test() {
    let input = r#"16,1,2,0,4,2,7,1,2,14"#;

    assert_eq!(37, part_one(input));
    assert_eq!(168, part_two(input));
}
