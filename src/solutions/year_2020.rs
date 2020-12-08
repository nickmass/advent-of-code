use ahash::{AHashMap as HashMap, AHashSet as HashSet};

use crate::{solution, Solution};

pub fn days() -> Vec<Solution> {
    vec![
        solution!(1, day_one_a, day_one_b),
        solution!(2, day_two_a, day_two_b),
        solution!(3, day_three_a, day_three_b),
        solution!(4, day_four_a, day_four_b),
        solution!(5, day_five_a, day_five_b),
        solution!(6, day_six_a, day_six_b),
        solution!(7, day_seven_a, day_seven_b),
        solution!(8, day_eight_a, day_eight_b),
    ]
}

fn day_one_a(input: &str) -> u64 {
    let values: HashSet<u64> = input
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    for n in &values {
        let x = 2020 - n;
        if values.contains(&x) {
            return x * n;
        }
    }

    0
}

fn day_one_b(input: &str) -> u64 {
    let values: HashSet<u64> = input
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    for a in &values {
        for b in &values {
            if a + b > 2020 {
                continue;
            }
            let x = 2020 - a - b;
            if values.contains(&x) {
                return x * a * b;
            }
        }
    }

    0
}

#[derive(Debug)]
struct PwLine<'a> {
    min: usize,
    max: usize,
    character: char,
    password: &'a str,
}

impl<'a> PwLine<'a> {
    fn from_str(s: &'a str) -> Option<Self> {
        let (s, min) = take_u64(s)?;
        let s = take_token(s, "-")?;
        let (s, max) = take_u64(s)?;
        let s = take_token(s, " ")?;
        let (s, character) = take_char(s)?;
        let s = take_token(s, ": ")?;
        let (_s, password) = take_until(s, |_| false)?;

        Some(PwLine {
            min: min as usize,
            max: max as usize,
            character,
            password,
        })
    }
}

fn day_two_a(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().flat_map(PwLine::from_str).collect();

    let mut valid = 0;
    for pw in lines {
        let count = pw.password.chars().fold(0, |state, next| {
            if next == pw.character {
                state + 1
            } else {
                state
            }
        });

        if count >= pw.min && count <= pw.max {
            valid += 1;
        }
    }

    valid
}

fn day_two_b(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().flat_map(PwLine::from_str).collect();

    let mut valid = 0;
    for pw in lines {
        let mut found = false;
        for (idx, is_match) in pw
            .password
            .char_indices()
            .map(|(idx, c)| (idx, c == pw.character))
        {
            if idx == pw.min - 1 && is_match {
                found = true;
            } else if idx == pw.max - 1 {
                if found && !is_match || !found && is_match {
                    valid += 1;
                }
                break;
            }
        }
    }

    valid
}

fn take_u64(s: &str) -> Option<(&str, u64)> {
    let mut index = 0;
    for c in s.chars() {
        if c.is_digit(10) {
            index += 1;
        } else {
            break;
        }
    }

    if index == 0 {
        None
    } else {
        let n = s[0..index].parse().ok();
        let remainder = &s[index..];
        n.map(|n| (remainder, n))
    }
}

fn take_token<'a, 'b>(s: &'a str, n: &'b str) -> Option<&'a str> {
    if n.len() > s.len() {
        None
    } else {
        let search = &s[0..n.len()];
        if search == n {
            Some(&s[n.len()..])
        } else {
            None
        }
    }
}

fn take_char(s: &str) -> Option<(&str, char)> {
    if s.len() > 0 {
        let c = s.chars().next().unwrap();
        Some((&s[1..], c))
    } else {
        None
    }
}

fn take_until<F: Fn(char) -> bool>(s: &str, f: F) -> Option<(&str, &str)> {
    let mut index = 0;
    for c in s.bytes() {
        if f(c as char) {
            break;
        }
        index += 1;
    }

    let (n, s) = s.split_at(index);
    Some((s, n))
}

#[test]
fn parse_test() {
    let s = "123abc";
    let (s, num) = take_u64(s).unwrap();
    assert_eq!(num, 123);
    assert_eq!(s, "abc");

    let s = "abc123";
    let n = take_u64(s);
    assert!(n.is_none());

    let s = "abc123";
    let s = take_token(s, "abc").unwrap();
    let (s, num) = take_u64(s).unwrap();
    assert_eq!(s, "");
    assert_eq!(num, 123);

    let s = "abc123";
    let s = take_token(s, "abc123efg");
    assert!(s.is_none());

    let s = "abc123";
    let (s, n) = take_until(s, |c| c.is_digit(10)).unwrap();
    assert_eq!(s, "123");
    assert_eq!(n, "abc");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Map {
    Empty,
    Tree,
}

fn day_three_a(input: &str) -> u64 {
    let lines: Vec<Vec<Map>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ll| match ll {
                    '.' => Map::Empty,
                    '#' => Map::Tree,
                    _ => Map::Empty,
                })
                .collect()
        })
        .collect();

    let offset_x = 3;
    let offset_y = 1;
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;

    while let Some(spot) = lines.get(y).and_then(|l| l.get(x % l.len())) {
        if spot == &Map::Tree {
            count += 1;
        }

        x += offset_x;
        y += offset_y;
    }

    count
}

fn day_three_b(input: &str) -> u64 {
    let lines: Vec<Vec<Map>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ll| match ll {
                    '.' => Map::Empty,
                    '#' => Map::Tree,
                    _ => Map::Empty,
                })
                .collect()
        })
        .collect();

    let mut total = 1;

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (offset_x, offset_y) in &slopes {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;

        while let Some(spot) = lines.get(y).and_then(|l| l.get(x % l.len())) {
            if spot == &Map::Tree {
                count += 1;
            }

            x += offset_x;
            y += offset_y;
        }

        total *= count;
    }

    total
}

fn day_four_a(input: &str) -> u64 {
    let records = input.split("\n\n");

    let passports: Vec<HashMap<_, _>> = records
        .map(|r| {
            r.split_whitespace()
                .flat_map(|kv| {
                    let mut splits = kv.split(':');
                    let key = splits.next();
                    let value = splits.next();

                    key.zip(value)
                })
                .collect()
        })
        .collect();

    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_count = 0;
    'outer: for passport in passports {
        for &field in &required_fields {
            if !passport.contains_key(field) {
                continue 'outer;
            }
        }
        valid_count += 1;
    }

    valid_count
}

struct PassportYearValidator {
    min: u64,
    max: u64,
}

impl PassportYearValidator {
    fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }
}

impl PassportValidator for PassportYearValidator {
    fn validate(&self, val: &str) -> bool {
        let n = val.parse::<u64>();
        if let Ok(n) = n {
            n >= self.min && n <= self.max
        } else {
            false
        }
    }
}

struct PassportHeightValidator;
impl PassportValidator for PassportHeightValidator {
    fn validate(&self, val: &str) -> bool {
        let parsed = take_u64(val);
        match parsed {
            Some(("in", num)) if num >= 59 && num <= 76 => true,
            Some(("cm", num)) if num >= 150 && num <= 193 => true,
            _ => false,
        }
    }
}

struct PassportHairColorValidator;
impl PassportValidator for PassportHairColorValidator {
    fn validate(&self, val: &str) -> bool {
        let parsed = take_char(val);
        if let Some((s, '#')) = parsed {
            s.len() == 6 && s.chars().all(|c| c.is_digit(16))
        } else {
            false
        }
    }
}

struct PassportEyeColorValidator;
impl PassportValidator for PassportEyeColorValidator {
    fn validate(&self, val: &str) -> bool {
        match val {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }
}

struct PassportIdValidator;
impl PassportValidator for PassportIdValidator {
    fn validate(&self, val: &str) -> bool {
        val.len() == 9 && val.chars().all(|c| c.is_digit(10))
    }
}

trait PassportValidator {
    fn validate(&self, val: &str) -> bool;
}

fn day_four_b(input: &str) -> u64 {
    let records = input.split("\n\n");

    let passports: Vec<HashMap<_, _>> = records
        .map(|r| {
            r.split_whitespace()
                .flat_map(|kv| {
                    let mut splits = kv.split(':');
                    let key = splits.next();
                    let value = splits.next();

                    key.zip(value)
                })
                .collect()
        })
        .collect();

    let birth_year = PassportYearValidator::new(1920, 2002);
    let issue_year = PassportYearValidator::new(2010, 2020);
    let expire_year = PassportYearValidator::new(2020, 2030);

    let required_fields: &[(&str, &dyn PassportValidator)] = &[
        ("byr", &birth_year),
        ("iyr", &issue_year),
        ("eyr", &expire_year),
        ("hgt", &PassportHeightValidator),
        ("hcl", &PassportHairColorValidator),
        ("ecl", &PassportEyeColorValidator),
        ("pid", &PassportIdValidator),
    ];

    let mut valid_count = 0;
    'outer: for passport in passports {
        for &(key, validator) in required_fields {
            if let Some(value) = passport.get(key) {
                if !validator.validate(value) {
                    continue 'outer;
                }
            } else {
                continue 'outer;
            }
        }

        valid_count += 1;
    }

    valid_count
}

fn day_five_a(input: &str) -> u64 {
    let lines = input.lines();

    let mut max = usize::MIN;

    for line in lines {
        let mut row_range = 0..128;
        let mut col_range = 0..8;

        for c in line.chars() {
            match c {
                'F' => row_range = row_range.start..row_range.end - (row_range.len() / 2),
                'B' => row_range = row_range.start + row_range.len() / 2..row_range.end,
                'L' => col_range = col_range.start..col_range.end - (col_range.len() / 2),
                'R' => col_range = col_range.start + col_range.len() / 2..col_range.end,
                _ => unreachable!(),
            }
        }

        let val = row_range.start * 8 + col_range.start;
        max = max.max(val);
    }

    max as u64
}

fn day_five_b(input: &str) -> u64 {
    let lines = input.lines();

    let mut map = HashSet::new();
    let mut max = usize::MIN;
    let mut min = usize::MAX;

    for line in lines {
        let mut row_range = 0..128;
        let mut col_range = 0..8;

        for c in line.chars() {
            match c {
                'F' => row_range = row_range.start..row_range.end - (row_range.len() / 2),
                'B' => row_range = row_range.start + row_range.len() / 2..row_range.end,
                'L' => col_range = col_range.start..col_range.end - (col_range.len() / 2),
                'R' => col_range = col_range.start + col_range.len() / 2..col_range.end,
                _ => unreachable!(),
            }
        }

        let val = row_range.start * 8 + col_range.start;

        map.insert(val);
        max = max.max(val);
        min = min.min(val);
    }

    for n in min..max {
        if !map.contains(&n) {
            return n as u64;
        }
    }

    0
}

fn day_six_a(input: &str) -> u64 {
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

fn day_six_b(input: &str) -> u64 {
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

fn day_seven_a(input: &str) -> u64 {
    let bags: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let splits: Vec<_> = l.split_whitespace().collect();
            let target_color = (splits[0], splits[1]);
            let children = if l.ends_with("no other bags.") {
                Vec::new()
            } else {
                let mut i = 4;
                let mut children = Vec::new();
                while i + 3 < splits.len() {
                    let n: u64 = splits[i].parse().unwrap();
                    let c = (splits[i + 1], splits[i + 2]);
                    i += 4;
                    children.push((c, n));
                }
                children
            };

            (target_color, children)
        })
        .collect();

    let mut matches = HashSet::new();
    let mut no_match = HashSet::new();

    matches.insert(("shiny", "gold"));

    let mut list = Vec::with_capacity(bags.len());
    'outer: for (color, children) in &bags {
        if matches.contains(color) || no_match.contains(color) {
            continue;
        }
        let mut checked = HashSet::new();
        list.clear();
        list.extend(children.iter().map(|(child, _num)| child));

        while let Some(child_color) = list.pop() {
            checked.insert(child_color);
            if matches.contains(child_color) {
                matches.insert(*color);
                continue 'outer;
            } else if !no_match.contains(child_color) {
                let next_parent = bags.get(child_color).unwrap();
                for (child, _num) in next_parent {
                    list.push(child);
                }
            }
        }

        no_match.insert(color);
        for child in checked {
            no_match.insert(child);
        }
    }

    //subtract one for gold bag
    (matches.len() - 1) as u64
}

fn day_seven_b(input: &str) -> u64 {
    let bags: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let splits: Vec<_> = l.split_whitespace().collect();
            let target_color = format!("{} {}", splits[0], splits[1]);
            let children = if l.ends_with("no other bags.") {
                Vec::new()
            } else {
                let mut i = 4;
                let mut children = Vec::new();
                while i + 3 < splits.len() {
                    let n: u64 = splits[i].parse().unwrap();
                    let c = format!("{} {}", splits[i + 1], splits[i + 2]);
                    i += 4;
                    children.push((c, n));
                }
                children
            };

            (target_color, children)
        })
        .collect();

    //subtract one for gold bag
    search_bags(&bags, "shiny gold", 1) - 1
}

fn search_bags<S: AsRef<str>>(
    bags: &HashMap<String, Vec<(String, u64)>>,
    bag: S,
    count: u64,
) -> u64 {
    let bag = bags.get(bag.as_ref()).unwrap();

    let mut sum = count;
    for (k, v) in bag {
        sum += search_bags(&bags, k, v * count);
    }

    sum
}

#[derive(Debug)]
struct EmuState {
    acc: i64,
    ip: usize,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

fn day_eight_a(input: &str) -> u64 {
    let program: Vec<_> = input
        .lines()
        .map(|l| {
            let inst = &l[0..3];
            let n = l[4..].parse().unwrap();
            match inst {
                "nop" => Instruction::Nop(n),
                "acc" => Instruction::Acc(n),
                "jmp" => Instruction::Jmp(n),
                _ => unreachable!("invalid program"),
            }
        })
        .collect();

    let mut hit_instrs = HashSet::new();
    let mut state = EmuState { acc: 0, ip: 0 };

    while !hit_instrs.contains(&state.ip) {
        hit_instrs.insert(state.ip);
        let inst = program[state.ip];
        match inst {
            Instruction::Nop(_) => {
                state.ip += 1;
            }
            Instruction::Acc(n) => {
                state.acc += n;
                state.ip += 1;
            }
            Instruction::Jmp(n) => state.ip = (state.ip as i64 + n) as usize,
        }
    }

    state.acc as u64
}

fn day_eight_b(input: &str) -> u64 {
    let program: Vec<_> = input
        .lines()
        .map(|l| {
            let inst = &l[0..3];
            let n = l[4..].parse().unwrap();
            match inst {
                "nop" => Instruction::Nop(n),
                "acc" => Instruction::Acc(n),
                "jmp" => Instruction::Jmp(n),
                _ => unreachable!("invalid program"),
            }
        })
        .collect();

    let mut changed_instrs = HashSet::new();
    let mut hit_instrs = HashSet::new();
    loop {
        let mut changed = false;
        let mut state = EmuState { acc: 0, ip: 0 };
        hit_instrs.clear();
        while !hit_instrs.contains(&state.ip) {
            if state.ip == program.len() {
                return state.acc as u64;
            }
            hit_instrs.insert(state.ip);
            let inst = program[state.ip];
            match inst {
                Instruction::Nop(n) => {
                    if !changed_instrs.contains(&state.ip) && !changed {
                        changed_instrs.insert(state.ip);
                        changed = true;
                        state.ip = (state.ip as i64 + n) as usize;
                    } else {
                        state.ip += 1;
                    }
                }
                Instruction::Acc(n) => {
                    state.acc += n;
                    state.ip += 1;
                }
                Instruction::Jmp(n) => {
                    if !changed_instrs.contains(&state.ip) && !changed {
                        changed_instrs.insert(state.ip);
                        changed = true;
                        state.ip += 1;
                    } else {
                        state.ip = (state.ip as i64 + n) as usize;
                    }
                }
            }
        }
    }
}
