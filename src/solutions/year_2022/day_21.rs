use crate::HashMap;

pub fn part_one(input: &str) -> i64 {
    let monkeys = MonkeyTroop::new(input, false);

    monkeys
        .get("root")
        .unwrap_or(PartialNum::Num(0))
        .unwrap_or(0)
}

pub fn part_two(input: &str) -> i64 {
    let monkeys = MonkeyTroop::new(input, true);

    monkeys
        .get("root")
        .unwrap_or(PartialNum::Num(0))
        .unwrap_or(0)
}

struct MonkeyTroop<'a> {
    monkeys: HashMap<&'a str, Monkey<'a>>,
}

impl<'a> MonkeyTroop<'a> {
    fn new(input: &'a str, part_2: bool) -> Self {
        let monkeys = input
            .lines()
            .filter_map(|l| Monkey::parse(l, part_2))
            .collect();

        Self { monkeys }
    }

    fn get(&self, name: &str) -> Option<PartialNum> {
        self.monkeys.get(name)?.value(self)
    }
}

#[derive(Debug, Copy, Clone)]
enum Monkey<'a> {
    Num(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Eq(&'a str, &'a str),
    Human,
}

impl<'a> Monkey<'a> {
    fn parse(line: &'a str, part_2: bool) -> Option<(&str, Self)> {
        let (name, line) = line.split_once(": ")?;

        let eq = line
            .split_once(['+', '-', '*', '/'])
            .filter(|_| part_2 && name == "root")
            .map(|(l, r)| (l.trim(), r.trim()));
        let human = (name == "humn" && part_2).then_some(());
        let add = line.split_once(" + ");
        let sub = line.split_once(" - ");
        let mul = line.split_once(" * ");
        let div = line.split_once(" / ");

        let monkey = if let Some((lhs, rhs)) = eq {
            Monkey::Eq(lhs, rhs)
        } else if human.is_some() {
            Monkey::Human
        } else if let Some((lhs, rhs)) = add {
            Monkey::Add(lhs, rhs)
        } else if let Some((lhs, rhs)) = sub {
            Monkey::Sub(lhs, rhs)
        } else if let Some((lhs, rhs)) = mul {
            Monkey::Mul(lhs, rhs)
        } else if let Some((lhs, rhs)) = div {
            Monkey::Div(lhs, rhs)
        } else {
            let n = line.parse().ok()?;
            Monkey::Num(n)
        };

        Some((name, monkey))
    }

    fn value(&self, monkeys: &MonkeyTroop<'a>) -> Option<PartialNum> {
        match self {
            Monkey::Num(n) => Some(PartialNum::Num(*n)),
            Monkey::Add(l, r) => Some(monkeys.get(l)? + monkeys.get(r)?),
            Monkey::Sub(l, r) => Some(monkeys.get(l)? - monkeys.get(r)?),
            Monkey::Mul(l, r) => Some(monkeys.get(l)? * monkeys.get(r)?),
            Monkey::Div(l, r) => Some(monkeys.get(l)? / monkeys.get(r)?),
            Monkey::Human => Some(PartialNum::Equation(vec![])),
            Monkey::Eq(l, r) => {
                let l = monkeys.get(l)?;
                let r = monkeys.get(r)?;

                match (l, r) {
                    (PartialNum::Num(l), PartialNum::Num(r)) => {
                        if l == r {
                            Some(PartialNum::Num(1))
                        } else {
                            Some(PartialNum::Num(0))
                        }
                    }
                    (PartialNum::Num(mut n), PartialNum::Equation(eq))
                    | (PartialNum::Equation(eq), PartialNum::Num(mut n)) => {
                        for op in eq.into_iter().rev() {
                            match op {
                                HumanOp::AddHuman(x) => n -= x,
                                HumanOp::MulHuman(x) => n /= x,
                                HumanOp::SubHuman(x) => n = x - n,
                                HumanOp::HumanSub(x) => n += x,
                                HumanOp::DivHuman(x) => n = x / n,
                                HumanOp::HumanDiv(x) => n *= x,
                            }
                        }

                        Some(PartialNum::Num(n))
                    }
                    (PartialNum::Equation(_), PartialNum::Equation(_)) => None,
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum HumanOp {
    AddHuman(i64),
    MulHuman(i64),
    SubHuman(i64),
    HumanSub(i64),
    DivHuman(i64),
    HumanDiv(i64),
}

enum PartialNum {
    Num(i64),
    Equation(Vec<HumanOp>),
}

impl PartialNum {
    fn unwrap_or(self, value: i64) -> i64 {
        match self {
            PartialNum::Num(n) => n,
            PartialNum::Equation(_) => value,
        }
    }
}

impl From<i64> for PartialNum {
    fn from(n: i64) -> Self {
        PartialNum::Num(n)
    }
}

impl std::ops::Add for PartialNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PartialNum::Num(n), PartialNum::Num(m)) => PartialNum::Num(n + m),
            (PartialNum::Num(n), PartialNum::Equation(mut eq))
            | (PartialNum::Equation(mut eq), PartialNum::Num(n)) => {
                eq.push(HumanOp::AddHuman(n));
                PartialNum::Equation(eq)
            }
            (PartialNum::Equation(_), PartialNum::Equation(_)) => unimplemented!(),
        }
    }
}

impl std::ops::Mul for PartialNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PartialNum::Num(n), PartialNum::Num(m)) => PartialNum::Num(n * m),
            (PartialNum::Num(n), PartialNum::Equation(mut eq))
            | (PartialNum::Equation(mut eq), PartialNum::Num(n)) => {
                eq.push(HumanOp::MulHuman(n));
                PartialNum::Equation(eq)
            }
            (PartialNum::Equation(_), PartialNum::Equation(_)) => unimplemented!(),
        }
    }
}

impl std::ops::Sub for PartialNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PartialNum::Num(n), PartialNum::Num(m)) => PartialNum::Num(n - m),
            (PartialNum::Num(n), PartialNum::Equation(mut eq)) => {
                eq.push(HumanOp::SubHuman(n));
                PartialNum::Equation(eq)
            }
            (PartialNum::Equation(mut eq), PartialNum::Num(n)) => {
                eq.push(HumanOp::HumanSub(n));
                PartialNum::Equation(eq)
            }
            (PartialNum::Equation(_), PartialNum::Equation(_)) => unimplemented!(),
        }
    }
}

impl std::ops::Div for PartialNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PartialNum::Num(n), PartialNum::Num(m)) => PartialNum::Num(n / m),
            (PartialNum::Num(n), PartialNum::Equation(mut eq)) => {
                eq.push(HumanOp::DivHuman(n));
                PartialNum::Equation(eq)
            }
            (PartialNum::Equation(mut eq), PartialNum::Num(n)) => {
                eq.push(HumanOp::HumanDiv(n));
                PartialNum::Equation(eq)
            }
            (PartialNum::Equation(_), PartialNum::Equation(_)) => unimplemented!(),
        }
    }
}

#[test]
fn test() {
    let input = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    assert_eq!(152, part_one(input));
    assert_eq!(301, part_two(input));
}
