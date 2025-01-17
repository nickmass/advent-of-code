use crate::{HashMap, HashSet};
pub fn part_one(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|s| s.parse::<Line>().unwrap().winning_count)
        .filter(|c| *c > 0)
        .map(|c| 2u32.pow(c - 1))
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.trim().lines();
    let mut cards = HashMap::new();

    for line in lines.map(|s| s.parse::<Line>().unwrap()) {
        let count = line.winning_count + 1;
        let multiplier = *cards.entry(line.id).or_insert(1);

        for i in 1..count {
            let entry = cards.entry(line.id + i).or_insert(1);
            *entry += multiplier;
        }
    }

    cards.values().sum()
}

struct Line {
    id: u32,
    winning_count: u32,
}

#[derive(Debug, Copy, Clone)]
struct LineParseErr;

impl std::str::FromStr for Line {
    type Err = LineParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(" ").ok_or(LineParseErr)?;
        let (id, s) = s.split_once(": ").ok_or(LineParseErr)?;
        let (winners, mine) = s.split_once(" | ").ok_or(LineParseErr)?;

        let id = id.trim().parse().map_err(|_| LineParseErr)?;
        let winners = collect_numbers(winners).collect::<HashSet<_>>();
        let winning_count = collect_numbers(mine)
            .filter(|n| winners.contains(n))
            .count() as u32;

        Ok(Line { id, winning_count })
    }
}

fn collect_numbers(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.split(" ").filter_map(|s| s.parse().ok())
}

#[test]
fn test() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    assert_eq!(13, part_one(input));
    assert_eq!(30, part_two(input));
}
