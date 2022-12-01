use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    let mut lines = input.trim().lines();
    let mut rules = RuleCollection::new();
    for line in lines.by_ref() {
        if line.len() == 0 {
            break;
        }
        rules.insert(line);
    }

    lines.filter(|l| rules.is_non_recursive_match(l)).count()
}

pub fn part_two(input: &str) -> usize {
    let mut lines = input.trim().lines();
    let mut rules = RuleCollection::new();
    for line in lines.by_ref() {
        if line.len() == 0 {
            break;
        }
        rules.insert(line);
    }

    rules.insert("8: 42 | 42 8");
    rules.insert("11: 42 31 | 42 11 31");

    rules.compile();

    lines.filter(|l| rules.is_cyk_match(l)).count()
}

struct Rule {
    id: CnfRuleId,
    matcher: Match,
}

#[derive(Debug, Clone)]
enum Match {
    Simple(Vec<CnfRuleId>),
    Pair(Vec<CnfRuleId>, Vec<CnfRuleId>),
    Char(char),
}

impl std::str::FromStr for Rule {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(Eq, PartialEq)]
        enum Stage {
            Id,
            Char,
            MatchOne,
            MatchTwo,
        }

        let mut stage = Stage::Id;
        let mut num = None;
        let mut id = 0;
        let mut match_one = Vec::new();
        let mut match_two = Vec::new();
        let mut match_char = '0';
        for c in s.trim().chars() {
            match c {
                '|' => {
                    stage = Stage::MatchTwo;
                }
                ':' => {
                    id = num.unwrap();
                    num = None;
                    stage = Stage::MatchOne;
                }
                '"' => {
                    stage = Stage::Char;
                }
                c if c.is_ascii_digit() && stage != Stage::Char => {
                    num = Some((num.unwrap_or(0) * 10) + (c as u8 - '0' as u8) as CnfRuleId);
                }
                c if c.is_whitespace() => {
                    if let Some(next) = num {
                        match stage {
                            Stage::MatchOne => match_one.push(next),
                            Stage::MatchTwo => match_two.push(next),
                            _ => unreachable!("invalid rule"),
                        }
                        num = None;
                    }
                }
                c => {
                    match_char = c;
                }
            }
        }

        if let Some(next) = num {
            match stage {
                Stage::MatchOne => match_one.push(next),
                Stage::MatchTwo => match_two.push(next),
                _ => unreachable!("invalid rule"),
            }
        }

        let res = match (match_one.len(), match_two.len()) {
            (0, 0) => Rule {
                id,
                matcher: Match::Char(match_char),
            },
            (_, 0) => Rule {
                id,
                matcher: Match::Simple(match_one),
            },
            (_, _) => Rule {
                id,
                matcher: Match::Pair(match_one, match_two),
            },
        };
        Ok(res)
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.matcher {
            Match::Char(c) => write!(f, "{}: \"{}\"", self.id, c),
            Match::Simple(rules) => {
                let rules = rules
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "{}: {}", self.id, rules)
            }
            Match::Pair(left_rules, right_rules) => {
                let left = left_rules
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                let right = right_rules
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "{}: {} | {}", self.id, left, right)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum CnfRule {
    Terminator(char),
    Producer(CnfRuleId, CnfRuleId),
}

impl std::fmt::Display for CnfRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CnfRule::Terminator(c) => write!(f, "'{}'", c),
            CnfRule::Producer(l, r) => write!(f, "{} {}", l, r),
        }
    }
}

#[derive(Debug, Clone)]
struct MatchState {
    idx: usize,
}

struct RuleCollection {
    map: HashMap<CnfRuleId, Rule>,
    cnf_rules: Vec<(CnfRuleId, CnfRule)>,
    cnf_generator_idx: CnfRuleId,
}

type CnfRuleId = u8;

impl RuleCollection {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            cnf_rules: Vec::new(),
            cnf_generator_idx: CnfRuleId::MAX,
        }
    }

    fn insert<S: AsRef<str>>(&mut self, rule: S) {
        let rule: Rule = rule.as_ref().trim().parse().unwrap();
        self.map.insert(rule.id, rule);
    }

    fn compile(&mut self) {
        let rules: Vec<_> = self.map.drain().collect();

        let mut simplified_rules = Vec::new();
        let mut singular_rules = Vec::new();

        for (_id, rule) in rules {
            match rule.matcher {
                Match::Pair(l, r) => {
                    simplified_rules.push(Rule {
                        id: rule.id,
                        matcher: Match::Simple(l),
                    });
                    simplified_rules.push(Rule {
                        id: rule.id,
                        matcher: Match::Simple(r),
                    });
                }
                _ => simplified_rules.push(rule),
            }
        }

        for (idx, rule) in simplified_rules.iter().enumerate() {
            match rule.matcher {
                Match::Simple(ref l) if l.len() == 1 => singular_rules.push((rule.id, idx, l[0])),
                _ => {}
            }
        }

        for (id, single_idx, target) in singular_rules {
            let mut new_val = simplified_rules
                .iter()
                .filter(|r| r.id == target)
                .map(|r| &r.matcher)
                .cloned()
                .collect::<Vec<_>>()
                .into_iter();

            if let Some((single, new_val)) =
                simplified_rules.get_mut(single_idx).zip(new_val.next())
            {
                single.matcher = new_val;
            }

            for val in new_val {
                simplified_rules.push(Rule { id, matcher: val })
            }
        }

        let convert_list = |rule_id: CnfRuleId,
                            map: &mut Vec<(CnfRuleId, CnfRule)>,
                            list: &mut Vec<CnfRuleId>,
                            idx: &mut CnfRuleId| {
            if list.len() == 2 {
                map.push((rule_id, CnfRule::Producer(list[0], list[1])));
            } else if list.len() > 2 {
                list.reverse();
                let mut previous = None;
                while let Some(item) = list.pop() {
                    if let Some(prev) = previous {
                        let id = if list.len() == 0 {
                            rule_id
                        } else {
                            *idx -= 1;
                            *idx
                        };
                        map.push((id, CnfRule::Producer(prev, item)));
                        previous = Some(id);
                    } else {
                        previous = Some(item);
                    }
                }
            } else {
                panic!(
                    "single or empty rules not supported: {} {:?}",
                    rule_id, list
                )
            }
        };

        for rule in simplified_rules {
            match rule.matcher {
                Match::Simple(mut list) => {
                    convert_list(
                        rule.id,
                        &mut self.cnf_rules,
                        &mut list,
                        &mut self.cnf_generator_idx,
                    );
                }
                Match::Pair(mut left, mut right) => {
                    convert_list(
                        rule.id,
                        &mut self.cnf_rules,
                        &mut left,
                        &mut self.cnf_generator_idx,
                    );
                    convert_list(
                        rule.id,
                        &mut self.cnf_rules,
                        &mut right,
                        &mut self.cnf_generator_idx,
                    );
                }
                Match::Char(c) => {
                    self.cnf_rules.push((rule.id, CnfRule::Terminator(c)));
                }
            }
        }

        self.cnf_rules.sort_by_key(|r| r.0);
    }

    fn is_cyk_match(&self, s: &str) -> bool {
        let str_len = s.len();
        let mut cyk_table = vec![vec![Vec::new(); str_len]; str_len];

        for (c_idx, c) in s.chars().enumerate() {
            for (id, r) in self.cnf_rules.iter() {
                if let &CnfRule::Terminator(r_c) = r {
                    if r_c == c {
                        cyk_table[0][c_idx].push(*id);
                    }
                }
            }
        }

        for l in 2..=str_len {
            for s in 1..=str_len - l + 1 {
                for p in 1..=l - 1 {
                    let b_idx = (p - 1, s - 1);
                    let c_idx = (l - p - 1, s + p - 1);
                    if cyk_table[b_idx.0][b_idx.1].len() == 0
                        || cyk_table[c_idx.0][c_idx.1].len() == 0
                    {
                        continue;
                    }
                    for (a, r) in self.cnf_rules.iter() {
                        if let &CnfRule::Producer(b, c) = r {
                            if cyk_table[b_idx.0][b_idx.1].contains(&b)
                                && cyk_table[c_idx.0][c_idx.1].contains(&c)
                            {
                                cyk_table[l - 1][s - 1].push(*a);
                            }
                        }
                    }
                }
            }
        }

        cyk_table[str_len - 1][0].contains(&0)
    }

    fn is_non_recursive_match(&self, s: &str) -> bool {
        let r0 = self.map.get(&0);

        if let Some(r0) = r0 {
            let mut state = MatchState { idx: 0 };
            let chars: Vec<_> = s.chars().collect();
            let is_match = self.is_match(r0, &chars, &mut state);
            if is_match && state.idx == s.len() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_match(&self, rule: &Rule, s: &[char], state: &mut MatchState) -> bool {
        if state.idx >= s.len() {
            return false;
        }

        match &rule.matcher {
            Match::Char(c) => {
                if Some(c) == s.get(state.idx) {
                    state.idx += 1;
                    true
                } else {
                    false
                }
            }
            Match::Simple(rules) => self.is_list_match(rules, s, state),
            Match::Pair(left, right) => {
                self.is_list_match(left, s, state) || self.is_list_match(right, s, state)
            }
        }
    }

    fn is_list_match(&self, list: &[CnfRuleId], s: &[char], state: &mut MatchState) -> bool {
        let orig_idx = state.idx;
        let mut is_match = true;
        for r in list.iter().filter_map(|id| self.map.get(id)) {
            if !self.is_match(r, s, state) {
                is_match = false;
                state.idx = orig_idx;
                break;
            }
        }
        is_match
    }
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
    run_a(i, 002);

    let i = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    run_a(i, 003);
    run_b(i, 012);
}
