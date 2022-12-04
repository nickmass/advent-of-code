pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter_map(parse_line)
        .filter(|(x1, x2, y1, y2)| x1 <= y1 && x2 >= y2 || y1 <= x1 && y2 >= x2)
        .count()
}

pub fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter_map(parse_line)
        .filter(|(x1, x2, y1, y2)| x1 <= y2 && y1 <= x2)
        .count()
}

fn parse_line(line: &str) -> Option<(u64, u64, u64, u64)> {
    let (left, right) = line.split_once(",")?;
    let (x1, x2) = left.split_once("-")?;
    let (y1, y2) = right.split_once("-")?;

    let x1 = x1.parse().ok()?;
    let x2 = x2.parse().ok()?;
    let y1 = y1.parse().ok()?;
    let y2 = y2.parse().ok()?;

    Some((x1, x2, y1, y2))
}

#[test]
fn test() {
    let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    assert_eq!(2, part_one(input));
    assert_eq!(4, part_two(input));
}
