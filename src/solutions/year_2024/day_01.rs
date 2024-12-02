use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .trim()
        .lines()
        .filter_map(|l| l.trim().split_once(" "))
        .map(|(l, r)| (l.trim(), r.trim()))
        .map(|(l, r)| (l.parse::<i32>().ok(), r.parse::<i32>().ok()))
        .filter_map(|(l, r)| l.zip(r))
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let lines = input
        .trim()
        .lines()
        .filter_map(|l| l.trim().split_once(" "))
        .map(|(l, r)| (l.trim(), r.trim()))
        .map(|(l, r)| (l.parse::<i32>().ok(), r.parse::<i32>().ok()))
        .filter_map(|(l, r)| l.zip(r));

    let mut left = Vec::new();
    let mut right = HashMap::new();
    for (l, r) in lines {
        left.push(l);
        *right.entry(r).or_insert(0) += 1;
    }

    left.into_iter()
        .map(|l| l * right.get(&l).copied().unwrap_or_default())
        .sum()
}

#[test]
fn test() {
    let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    assert_eq!(11, part_one(input));
    assert_eq!(31, part_two(input));
}
