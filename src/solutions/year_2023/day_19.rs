use crate::HashMap;

pub fn part_one(input: &str) -> i64 {
    let (rules, items) = input.trim().split_once("\n\n").unwrap();

    let mut score = 0;

    let rule_map = parse_rule_map(rules);
    let inital_workflow = rule_map.get("in").unwrap();

    'items: for line in items.lines() {
        let item = parse_item(line);
        let mut workflow = inital_workflow;
        loop {
            for rule in workflow {
                if let Some(outcome) = rule.apply(&item) {
                    match outcome {
                        Outcome::Forward(w) => {
                            workflow = rule_map.get(w).unwrap();
                            break;
                        }
                        Outcome::Accept => {
                            score += item.sum();
                            continue 'items;
                        }
                        Outcome::Reject => {
                            continue 'items;
                        }
                    }
                }
            }
        }
    }

    score
}

pub fn part_two(input: &str) -> i64 {
    let (rules, _items) = input.trim().split_once("\n\n").unwrap();
    let rule_map = parse_rule_map(rules);
    let inital_workflow = rule_map.get("in").unwrap();

    let mut paths = Vec::new();
    paths.push((ItemRange::new(), inital_workflow));

    let mut count = 0;

    while let Some((mut range, workflow)) = paths.pop() {
        for rule in workflow {
            let (pass, fail) = rule.apply_range(&range);

            if let Some((pass, outcome)) = pass {
                match outcome {
                    Outcome::Forward(w) => {
                        let workflow = rule_map.get(w).unwrap();
                        paths.push((pass, workflow));
                    }
                    Outcome::Accept => {
                        count += pass.count();
                    }
                    Outcome::Reject => (),
                }
            }

            if let Some(fail) = fail {
                range = fail;
            } else {
                break;
            }
        }
    }

    count
}

fn parse_rule_map(input: &str) -> HashMap<&'_ str, Vec<Rule>> {
    let mut rule_map = HashMap::new();

    for line in input.trim().lines() {
        let (name, rest) = line.split_once("{").unwrap();
        let rules = rest.trim_end_matches("}").split(",");

        let mut rule_list = Vec::new();
        for rule in rules {
            let less = rule.split_once('<').map(|(p, r)| (p, r, Operation::Less));
            let greater = rule
                .split_once('>')
                .map(|(p, r)| (p, r, Operation::Greater));

            let rule = if let Some((param, rest, op)) = less.or(greater) {
                let (value, outcome) = rest.split_once(':').unwrap();
                let value = value.parse::<i64>().unwrap();
                let outcome = Outcome::from_str(outcome);
                let param = Param::from_str(param);
                let condition = Condition::Apply(param, op, value);

                Rule { condition, outcome }
            } else {
                let condition = Condition::Default;
                let outcome = Outcome::from_str(rule);

                Rule { condition, outcome }
            };
            rule_list.push(rule);
        }
        rule_map.insert(name, rule_list);
    }

    rule_map
}

fn parse_item(line: &str) -> Item {
    let line = line.trim_start_matches('{').trim_end_matches('}');

    let mut item = Item::builder();
    for param in line.split(",") {
        let (param, value) = param.split_once('=').unwrap();

        let param = Param::from_str(param);
        let value = value.parse::<i64>().unwrap();

        item.add(param, value);
    }

    item.build()
}

#[derive(Default)]
struct ItemBuilder {
    x: Option<i64>,
    m: Option<i64>,
    a: Option<i64>,
    s: Option<i64>,
}

impl ItemBuilder {
    fn add(&mut self, param: Param, value: i64) {
        match param {
            Param::X => self.x = Some(value),
            Param::M => self.m = Some(value),
            Param::A => self.a = Some(value),
            Param::S => self.s = Some(value),
        }
    }

    fn build(self) -> Item {
        Item {
            x: self.x.unwrap(),
            m: self.m.unwrap(),
            a: self.a.unwrap(),
            s: self.s.unwrap(),
        }
    }
}

struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Item {
    fn builder() -> ItemBuilder {
        ItemBuilder {
            ..Default::default()
        }
    }

    fn get(&self, param: Param) -> i64 {
        match param {
            Param::X => self.x,
            Param::M => self.m,
            Param::A => self.a,
            Param::S => self.s,
        }
    }

    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Param {
    X,
    M,
    A,
    S,
}

impl Param {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Param::X,
            "m" => Param::M,
            "a" => Param::A,
            "s" => Param::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Less,
    Greater,
}

impl Operation {
    fn apply(&self, lhs: i64, rhs: i64) -> bool {
        match self {
            Operation::Less => lhs < rhs,
            Operation::Greater => lhs > rhs,
        }
    }

    fn apply_range(&self, range: Range, rhs: i64) -> (Option<Range>, Option<Range>) {
        match self {
            Operation::Less => range.split_less(rhs),
            Operation::Greater => range.split_greater(rhs),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome<'a> {
    Forward(&'a str),
    Accept,
    Reject,
}

impl<'a> Outcome<'a> {
    fn from_str(s: &'a str) -> Outcome<'a> {
        match s {
            "A" => Outcome::Accept,
            "R" => Outcome::Reject,
            s => Outcome::Forward(s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Condition {
    Apply(Param, Operation, i64),
    Default,
}

#[derive(Debug, Copy, Clone)]
struct Rule<'a> {
    condition: Condition,
    outcome: Outcome<'a>,
}

impl<'a> Rule<'a> {
    fn apply(&self, item: &Item) -> Option<Outcome<'a>> {
        match self.condition {
            Condition::Apply(p, op, v) => {
                let lhs = item.get(p);
                if op.apply(lhs, v) {
                    Some(self.outcome)
                } else {
                    None
                }
            }
            Condition::Default => Some(self.outcome),
        }
    }

    fn apply_range(
        &self,
        item: &ItemRange,
    ) -> (Option<(ItemRange, Outcome<'a>)>, Option<ItemRange>) {
        match self.condition {
            Condition::Apply(p, op, v) => {
                let lhs = item.get(p);
                let (pass, fail) = op.apply_range(lhs, v);
                let pass = pass.map(|r| {
                    let mut item = item.clone();
                    item.set(p, r);
                    (item, self.outcome)
                });

                let fail = fail.map(|r| {
                    let mut item = item.clone();
                    item.set(p, r);
                    item
                });

                (pass, fail)
            }
            Condition::Default => (Some((*item, self.outcome)), None),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    lo: i64,
    hi: i64,
}

impl Range {
    fn new() -> Range {
        Range { lo: 1, hi: 4000 }
    }

    fn split_less(&self, n: i64) -> (Option<Range>, Option<Range>) {
        if n > self.lo && n <= self.hi {
            (
                Some(Range {
                    lo: self.lo,
                    hi: n - 1,
                }),
                Some(Range { lo: n, hi: self.hi }),
            )
        } else if n > self.hi {
            (Some(*self), None)
        } else if n <= self.lo {
            (None, Some(*self))
        } else {
            unreachable!()
        }
    }

    fn split_greater(&self, n: i64) -> (Option<Range>, Option<Range>) {
        if n >= self.lo && n < self.hi {
            (
                Some(Range {
                    lo: n + 1,
                    hi: self.hi,
                }),
                Some(Range { lo: self.lo, hi: n }),
            )
        } else if n >= self.hi {
            (None, Some(*self))
        } else if n < self.lo {
            (Some(*self), None)
        } else {
            unreachable!()
        }
    }

    fn len(&self) -> i64 {
        self.hi - (self.lo - 1)
    }
}

#[derive(Debug, Copy, Clone)]
struct ItemRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl ItemRange {
    fn new() -> Self {
        ItemRange {
            x: Range::new(),
            m: Range::new(),
            a: Range::new(),
            s: Range::new(),
        }
    }

    fn get(&self, param: Param) -> Range {
        match param {
            Param::X => self.x,
            Param::M => self.m,
            Param::A => self.a,
            Param::S => self.s,
        }
    }

    fn set(&mut self, param: Param, range: Range) {
        match param {
            Param::X => self.x = range,
            Param::M => self.m = range,
            Param::A => self.a = range,
            Param::S => self.s = range,
        }
    }

    fn count(&self) -> i64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[test]
fn test() {
    let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

    assert_eq!(19114, part_one(input));
    assert_eq!(167409079868000, part_two(input));
}
