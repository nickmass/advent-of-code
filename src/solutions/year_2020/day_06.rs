use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> u64 {
    let questions: usize = input
        .split("\n\n")
        .map(|ls| {
            let mut set = HashSet::new();
            for c in ls.chars() {
                if !c.is_whitespace() {
                    set.insert(c);
                }
            }
            set.len()
        })
        .sum();

    questions as u64
}

pub fn part_two(input: &str) -> u64 {
    let questions: usize = input
        .split("\n\n")
        .map(|ls| {
            let mut line_count = 0;
            let mut set = HashMap::new();
            for line in ls.lines() {
                for c in line.chars() {
                    set.entry(c).and_modify(|co| *co += 1).or_insert(1);
                }
                line_count += 1;
            }

            set.iter().filter(|(_k, v)| **v == line_count).count()
        })
        .sum();

    questions as u64
}

#[test]
fn test() {
    let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#;

    assert_eq!(11, part_one(input));
    assert_eq!(6, part_two(input));
}
