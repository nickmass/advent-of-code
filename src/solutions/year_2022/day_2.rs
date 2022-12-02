pub fn part_one(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|l| match l.split_once(" ").unwrap() {
            ("A", "X") => 4,
            ("A", "Y") => 8,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 7,
            ("C", "Y") => 2,
            ("C", "Z") => 6,
            _ => unreachable!(),
        })
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|l| match l.split_once(" ").unwrap() {
            ("A", "X") => 3,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            ("C", "Z") => 7,
            _ => unreachable!(),
        })
        .sum()
}

#[test]
fn test() {
    let input = r#"A Y
B X
C Z"#;

    assert_eq!(15, part_one(input));
    assert_eq!(12, part_two(input));
}
