use crate::HashMap;

pub fn part_one(input: &str) -> u32 {
    let mut lines = input.trim().lines();

    let rules = Rules::new(&mut lines);

    let mut sum = 0;
    let mut buf = Vec::new();

    for line in lines {
        let update = Update::new(line, &mut buf);

        if update.in_order(&rules) {
            sum += update.mid();
        }
    }

    sum
}

pub fn part_two(input: &str) -> u32 {
    let mut lines = input.trim().lines();

    let rules = Rules::new(&mut lines);

    let mut sum = 0;
    let mut buf = Vec::new();

    for line in lines {
        let mut update = Update::new(line, &mut buf);

        if !update.in_order(&rules) {
            update.sort(&rules);
            sum += update.mid();
        }
    }

    sum
}

struct Rules {
    map: HashMap<u32, Vec<u32>>,
}

impl Rules {
    fn new(lines: &mut std::str::Lines) -> Self {
        let mut rules = HashMap::new();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let Some((pre, post)) = line.trim().split_once("|") else {
                continue;
            };

            let pre = pre.parse::<u32>().ok();
            let post = post.parse::<u32>().ok();

            let Some((pre, post)) = pre.zip(post) else {
                continue;
            };

            let entry = rules.entry(post).or_insert(Vec::new());
            entry.push(pre);
        }

        Self { map: rules }
    }

    fn in_order(&self, first: u32, second: u32) -> bool {
        let Some(deps) = self.map.get(&first) else {
            return true;
        };

        !deps.contains(&second)
    }
}

struct Update<'a> {
    buf: &'a mut Vec<u32>,
}

impl<'a> Update<'a> {
    fn new(line: &str, buf: &'a mut Vec<u32>) -> Self {
        buf.clear();
        buf.extend(line.trim().split(',').filter_map(|n| n.parse::<u32>().ok()));

        Self { buf }
    }

    fn pairs(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        let mut first_idx = 0;
        let mut second_idx = first_idx + 1;

        std::iter::from_fn(move || {
            let first = self.buf.get(first_idx)?;
            let second = self.buf.get(second_idx)?;

            second_idx += 1;
            if second_idx >= self.buf.len() {
                first_idx += 1;
                second_idx = first_idx + 1;
            }

            Some((*first, *second))
        })
    }

    fn in_order(&self, rules: &Rules) -> bool {
        self.pairs().all(|(a, b)| rules.in_order(a, b))
    }

    fn sort(&mut self, rules: &Rules) {
        use std::cmp::Ordering;
        self.buf.sort_unstable_by(|a, b| {
            if rules.in_order(*a, *b) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
    }

    fn mid(&self) -> u32 {
        let mid_idx = self.buf.len() / 2;
        self.buf[mid_idx]
    }
}

#[test]
fn test() {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    assert_eq!(143, part_one(input));
    assert_eq!(123, part_two(input));
}
