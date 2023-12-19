use crate::HashMap;

pub fn part_one(input: &str) -> i64 {
    let (rules, items) = input.trim().split_once("\n\n").unwrap();

    let mut score = 0;

    let workflows = WorkflowMap::new(rules);

    'items: for item in items.lines().map(|l| l.parse::<Item>().unwrap()) {
        let mut workflow = "in";
        loop {
            for rule in workflows.get(workflow) {
                if let Some(outcome) = rule.apply(&item) {
                    match outcome {
                        Outcome::Forward(w) => {
                            workflow = w;
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
    let workflows = WorkflowMap::new(rules);

    let mut paths = Vec::new();
    paths.push((ItemRange::new(), "in"));

    let mut count = 0;

    while let Some((mut range, workflow)) = paths.pop() {
        for rule in workflows.get(workflow) {
            let (pass, fail) = rule.apply_range(&range);

            if let Some((pass, outcome)) = pass {
                match outcome {
                    Outcome::Forward(w) => {
                        paths.push((pass, w));
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

struct WorkflowMap<'a> {
    workflows: HashMap<&'a str, (usize, usize)>,
    rules: Vec<Rule<'a>>,
}

impl<'a> WorkflowMap<'a> {
    fn new(input: &'a str) -> Self {
        let mut workflows = HashMap::new();
        let mut rules = Vec::new();

        for line in input.trim().lines() {
            let (name, rest) = line.trim_end_matches('}').split_once('{').unwrap();

            let start_idx = rules.len();
            rules.extend(rest.split(",").map(|r| Rule::try_from(r).unwrap()));
            workflows.insert(name, (start_idx, rules.len()));
        }

        WorkflowMap { workflows, rules }
    }

    fn get(&self, workflow: &str) -> impl Iterator<Item = Rule> + '_ {
        let workflow = self.workflows.get(workflow).copied();
        let mut i = 0;

        std::iter::from_fn(move || {
            let Some((start_idx, end_idx)) = workflow else {
                return None;
            };

            if i + start_idx == end_idx {
                return None;
            }

            let rule = self.rules.get(start_idx + i).copied();

            i += 1;

            rule
        })
    }
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

#[derive(Debug, Copy, Clone)]
struct ItemParseErr;

impl std::str::FromStr for Item {
    type Err = ItemParseErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.trim_start_matches('{').trim_end_matches('}');

        let mut item = Item::builder();
        for param in line.split(",") {
            let (param, value) = param.split_once('=').ok_or(ItemParseErr)?;

            let param = param.parse().map_err(|_| ItemParseErr)?;
            let value = value.parse().map_err(|_| ItemParseErr)?;

            item.add(param, value);
        }

        Ok(item.build())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Param {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone)]
struct ParamParseErr;

impl std::str::FromStr for Param {
    type Err = ParamParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let param = match s {
            "x" => Param::X,
            "m" => Param::M,
            "a" => Param::A,
            "s" => Param::S,
            _ => return Err(ParamParseErr),
        };

        Ok(param)
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

impl<'a> From<&'a str> for Outcome<'a> {
    fn from(value: &'a str) -> Self {
        match value {
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
struct RuleParseErr;

impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = RuleParseErr;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let less = value.split_once('<').map(|(p, r)| (p, r, Operation::Less));
        let greater = value
            .split_once('>')
            .map(|(p, r)| (p, r, Operation::Greater));

        if let Some((param, rest, op)) = less.or(greater) {
            let (value, outcome) = rest.split_once(':').ok_or(RuleParseErr)?;
            let value = value.parse().map_err(|_| RuleParseErr)?;
            let outcome = outcome.try_into().map_err(|_| RuleParseErr)?;
            let param = param.parse().map_err(|_| RuleParseErr)?;
            let condition = Condition::Apply(param, op, value);

            Ok(Rule { condition, outcome })
        } else {
            let condition = Condition::Default;
            let outcome = value.try_into().map_err(|_| RuleParseErr)?;

            Ok(Rule { condition, outcome })
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
