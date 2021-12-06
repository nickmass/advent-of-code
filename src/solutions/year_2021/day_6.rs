pub fn part_one(input: &str) -> u64 {
    solver::<80>(input)
}

pub fn part_two(input: &str) -> u64 {
    solver::<256>(input)
}

fn solver<const DAYS: usize>(input: &str) -> u64 {
    let nums: Vec<_> = input
        .trim()
        .split(',')
        .filter_map(|n| n.parse::<u8>().ok())
        .map(|n| (n))
        .collect();

    let mut day_counts = vec![0u64; DAYS + 8];

    for num in nums {
        day_counts[num as usize - 1] += 1;
    }

    for n in 0..DAYS {
        let count = day_counts[n];
        if count > 0 {
            let mut cycle = n + 9;
            while cycle < DAYS + 8 {
                day_counts[cycle] += count;
                cycle += 7;
            }
        }
    }

    day_counts.into_iter().sum::<u64>()
}

#[test]
fn test() {
    let input = r#"3,4,3,1,2"#;

    assert_eq!(5934, part_one(input));
    assert_eq!(26984457539, part_two(input));
}
