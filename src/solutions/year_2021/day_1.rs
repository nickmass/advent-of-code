pub fn part_one(input: &str) -> usize {
    let lines: Vec<u64> = input
        .trim()
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    lines
        .windows(2)
        .filter(|ar| if let [l, r] = ar { r > l } else { false })
        .count()
}

pub fn part_two(input: &str) -> u64 {
    let lines: Vec<u64> = input
        .trim()
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    let mut count = 0;
    let mut last_sum = None;

    for sum in lines.windows(3).map(|win| win.iter().sum::<u64>()) {
        if sum > last_sum.unwrap_or(u64::MAX) {
            count += 1;
        }

        last_sum = Some(sum);
    }

    count
}

#[test]
fn test() {
    let input = r#"199
200
208
210
200
207
240
269
260
263
"#;

    assert_eq!(7, part_one(input));
    assert_eq!(5, part_two(input));
}
