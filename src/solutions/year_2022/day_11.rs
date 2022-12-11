pub fn part_one(input: &str) -> u64 {
    solve::<20, 3>(input)
}

pub fn part_two(input: &str) -> u64 {
    solve::<10000, 1>(input)
}

fn solve<const ROUNDS: usize, const WORRY_FACTOR: u64>(input: &str) -> u64 {
    let mut monkeys: Vec<_> = input
        .trim()
        .split("\n\n")
        .filter_map(parse_monkey)
        .collect();

    let total_test: u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..ROUNDS {
        for idx in 0..monkeys.len() {
            assert_eq!(monkeys[idx].id, idx);

            while let Some(new) = monkeys[idx].inspect::<WORRY_FACTOR>() {
                let throw_idx = monkeys[idx].target(new);
                monkeys[throw_idx].push(new % total_test);
            }
        }
    }

    monkeys.sort_by_key(|m| std::cmp::Reverse(m.inspections));

    monkeys[0..2].iter().map(|m| m.inspections).product()
}

fn parse_monkey(input: &str) -> Option<Monkey> {
    let mut lines = input.lines();

    let parse_only_num = |line: Option<&str>| {
        line?
            .trim_start_matches(|c: char| !c.is_ascii_digit())
            .trim_end_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .ok()
    };

    let id = parse_only_num(lines.next())? as usize;
    let items = lines
        .next()?
        .trim()
        .trim_start_matches("Starting items: ")
        .split(",")
        .filter_map(|n| n.trim().parse().ok())
        .collect();

    let op = match lines
        .next()?
        .trim()
        .trim_start_matches("Operation: new = old ")
        .split_once(" ")?
    {
        ("*", "old") => Operation::Square,
        ("*", n) => Operation::Mul(n.parse().ok()?),
        ("+", n) => Operation::Add(n.parse().ok()?),
        _ => return None,
    };

    let test = parse_only_num(lines.next())?;
    let pass = parse_only_num(lines.next())? as usize;
    let fail = parse_only_num(lines.next())? as usize;

    Some(Monkey {
        id,
        items,
        op,
        test,
        pass,
        fail,
        inspections: 0,
    })
}

struct Monkey {
    id: usize,
    items: Vec<u64>,
    op: Operation,
    test: u64,
    pass: usize,
    fail: usize,
    inspections: u64,
}

impl Monkey {
    fn push(&mut self, item: u64) {
        self.items.push(item)
    }

    fn inspect<const WORRY_FACTOR: u64>(&mut self) -> Option<u64> {
        let item = self.items.pop()?;
        self.inspections += 1;
        Some(self.op.apply(item) / WORRY_FACTOR)
    }

    fn target(&self, new_item: u64) -> usize {
        if new_item % self.test == 0 {
            self.pass
        } else {
            self.fail
        }
    }
}

enum Operation {
    Mul(u64),
    Add(u64),
    Square,
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Mul(n) => n * old,
            Operation::Add(n) => n + old,
            Operation::Square => old * old,
        }
    }
}

#[test]
fn test() {
    let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    assert_eq!(10605, part_one(input));
    assert_eq!(2713310158, part_two(input));
}
