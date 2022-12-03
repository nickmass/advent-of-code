pub fn part_one(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut sum = 0;
    'outer: for line in lines {
        let mid = line.len() / 2;
        let (left, right) = line.split_at(mid);

        for c in left.chars() {
            if right.contains(c) {
                sum += score_char(c);
                continue 'outer;
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> u64 {
    let mut lines = input.trim().lines().into_iter();

    let mut sum = 0;
    'outer: while let Some(first) = lines.next() {
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                sum += score_char(c);
                continue 'outer;
            }
        }
    }

    sum
}

fn score_char(c: char) -> u64 {
    let c = c as u8;
    match c {
        b'a'..=b'z' => (c - b'a') as u64 + 1,
        b'A'..=b'Z' => (c - b'A') as u64 + 27,
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    assert_eq!(157, part_one(input));
    assert_eq!(70, part_two(input));
}
