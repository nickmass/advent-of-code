use ahash::{AHashMap as HashMap, AHashSet as HashSet};

use std::collections::VecDeque;

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
        solution!(9, day_nine_a, day_nine_b),
        solution!(10, day_ten_a, day_ten_b),
        solution!(11, day_eleven_a, day_eleven_b),
        solution!(12, day_twelve_a, day_twelve_b),
        solution!(13, day_thirteen_a, day_thirteen_b),
        solution!(14, day_fourteen_a, day_fourteen_b),
        solution!(15, day_fifteen_a, day_fifteen_b),
        solution!(16, day_sixteen_a, day_sixteen_b),
        solution!(17, day_seventeen_a, day_seventeen_b),
        solution!(18, day_eighteen_a, day_eighteen_b),
        solution!(19, day_nineteen_a, day_nineteen_b),
        solution!(20, day_twenty_a, day_twenty_b),
        solution!(21, day_twenty_one_a, day_twenty_one_b),
        solution!(22, day_twenty_two_a, day_twenty_two_b),
        solution!(23, day_twenty_three_a, day_twenty_three_b),
        solution!(24, day_twenty_four_a, day_twenty_four_b),
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
fn test_parse() {
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

fn day_nine_a(input: &str) -> u64 {
    let mut lines = input.lines().filter_map(|p| p.parse::<u64>().ok());

    let mut preamble: VecDeque<u64> = lines.by_ref().take(25).collect();

    for n in lines {
        let mut valid = false;
        'outer: for a in &preamble {
            for b in &preamble {
                if a == b {
                    continue;
                }
                if a + b == n {
                    valid = true;
                    break 'outer;
                }
            }
        }

        if !valid {
            return n;
        }
        preamble.pop_front();
        preamble.push_back(n);
    }

    panic!("invalid input")
}

fn day_nine_b(input: &str) -> u64 {
    let target = day_nine_a(input);
    let nums = input.lines().filter_map(|p| p.parse::<u64>().ok());

    let mut range = VecDeque::new();
    let mut sum = 0;
    for n in nums {
        sum += n;
        range.push_back(n);

        while sum > target {
            if let Some(n) = range.pop_front() {
                sum -= n;
            }
        }

        if sum == target {
            let min = range.iter().min();
            let max = range.iter().max();

            if let Some((min, max)) = min.zip(max) {
                return min + max;
            }
        }
    }

    panic!("invalid input")
}

fn day_ten_a(input: &str) -> u32 {
    let mut adapters: HashSet<u32> = input.lines().filter_map(|l| l.parse().ok()).collect();
    let built_in = adapters.iter().copied().max().unwrap_or(0) + 3;

    adapters.insert(built_in);

    let mut n = 1;
    let mut last_n = 0;
    let mut diffs = HashMap::new();
    while adapters.len() != 0 {
        if adapters.contains(&n) {
            diffs.entry(n - last_n).and_modify(|c| *c += 1).or_insert(1);
            last_n = n;
            adapters.remove(&n);
            n += 1;
        } else {
            n += 1;
            if n - last_n > 3 {
                panic!("wrong input");
            }
        }
    }

    let ones = diffs.get(&1).copied().unwrap_or(0);
    let threes = diffs.get(&3).copied().unwrap_or(0);

    ones * threes
}

#[test]
fn test_day_ten() {
    let run_a = |inp, res| assert_eq!(day_ten_a(inp), res);
    let run_b = |inp, res| assert_eq!(day_ten_b(inp), res);

    let inp = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    run_a(inp, 7 * 5);
    run_b(inp, 8);

    let inp = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    run_a(inp, 22 * 10);
    run_b(inp, 19208);
}

#[derive(Debug, Copy, Clone)]
struct Node {
    value: u32,
    paths_to_root: u64,
}

struct NodeCollection {
    map: HashMap<u32, Node>,
}

impl NodeCollection {
    fn with_capacity(capacity: usize) -> Self {
        let root_node = Node {
            value: 0,
            paths_to_root: 1,
        };
        let mut map = HashMap::with_capacity(capacity);
        map.insert(0, root_node);

        Self { map }
    }

    fn insert(&mut self, value: u32) -> u64 {
        let mut paths_to_root = 0;

        for n in 1..=3.min(value) {
            let parent_value = value - n;
            if let Some(parent) = self.map.get(&parent_value) {
                paths_to_root += parent.paths_to_root;
            }
        }

        if paths_to_root != 0 {
            let node = Node {
                value,
                paths_to_root,
            };
            self.map.insert(value, node);
        }

        paths_to_root
    }
}

fn day_ten_b(input: &str) -> u64 {
    let mut adapters: std::collections::BTreeSet<u32> =
        input.lines().filter_map(|l| l.parse().ok()).collect();
    let built_in = adapters.iter().copied().last().unwrap_or(0) + 3;
    adapters.insert(built_in);

    let mut nodes = NodeCollection::with_capacity(adapters.len());
    let mut paths = 0;
    for n in adapters.range(1..=built_in) {
        paths = nodes.insert(*n);
    }

    paths
}

#[test]
fn test_day_eleven() {
    let run_a = |inp, res| assert_eq!(day_eleven_a(inp), res);
    let run_b = |inp, res| assert_eq!(day_eleven_b(inp), res);

    let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;

    run_a(input, 37);
    run_b(input, 26);
}

#[derive(Debug, Copy, Clone)]
enum GridCell {
    Empty,
    Seat,
    Occupied,
}

#[derive(Debug, Clone)]
struct DayElevenGrid {
    cells: Vec<GridCell>,
    width: usize,
    height: usize,
    fancy: bool,
}

impl DayElevenGrid {
    fn new(input: &str, fancy_grid: bool) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::with_capacity(input.len());
        for c in input.chars() {
            let cell = match c {
                '#' => Some(GridCell::Occupied),
                'L' => Some(GridCell::Seat),
                '.' => Some(GridCell::Empty),
                '\n' => {
                    height += 1;
                    None
                }
                _ => panic!("invalid grid"),
            };

            if let Some(cell) = cell {
                cells.push(cell);
            }

            if height == 0 {
                width += 1;
            }
        }

        Self {
            cells,
            width,
            height,
            fancy: fancy_grid,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<GridCell> {
        if y >= self.height || x >= self.width {
            None
        } else {
            Some(self.cells[y * self.width + x])
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: GridCell) {
        self.cells[y * self.width + x] = cell;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u32 {
        let mut sum = 0;
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                let mut x = x;
                let mut y = y;
                loop {
                    x = (x as isize + x_offset) as usize;
                    y = (y as isize + y_offset) as usize;

                    match self.get(x, y) {
                        Some(GridCell::Occupied) => sum += 1,
                        Some(GridCell::Empty) if self.fancy => continue,
                        _ => (),
                    }
                    break;
                }
            }
        }
        sum
    }
}

fn day_eleven(input: &str, occupy_limit: u32, fancy: bool) -> i32 {
    let mut grid = DayElevenGrid::new(input, fancy);
    let mut next_grid = grid.clone();

    let mut changed = true;
    let mut occupied = 0;
    while changed {
        changed = false;
        for x in 0..grid.width {
            for y in 0..grid.height {
                let neighbors = grid.count_neighbors(x, y);
                match grid.get(x, y) {
                    Some(GridCell::Seat) if neighbors == 0 => {
                        changed = true;
                        next_grid.set(x, y, GridCell::Occupied);
                        occupied += 1;
                    }
                    Some(GridCell::Occupied) if neighbors >= occupy_limit => {
                        changed = true;
                        next_grid.set(x, y, GridCell::Seat);
                        occupied -= 1;
                    }
                    Some(cell) => {
                        next_grid.set(x, y, cell);
                    }
                    None => unreachable!("outside x y bounds"),
                }
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    occupied
}

fn day_eleven_a(input: &str) -> i32 {
    day_eleven(input, 4, false)
}

fn day_eleven_b(input: &str) -> i32 {
    day_eleven(input, 5, true)
}

#[derive(Debug, Copy, Clone)]
enum ShipActions {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Foreward(i32),
}

fn day_twelve_a(input: &str) -> u64 {
    let actions: Vec<_> = input
        .lines()
        .filter_map(|l| {
            let n = l[1..].trim().parse().ok();

            if let Some(n) = n {
                use ShipActions::*;
                let v = match &l[0..1] {
                    "N" => North(n),
                    "S" => South(n),
                    "E" => East(n),
                    "W" => West(n),
                    "L" => Left(n),
                    "R" => Right(n),
                    "F" => Foreward(n),
                    _ => return None,
                };
                Some(v)
            } else {
                None
            }
        })
        .collect();

    let mut x = 0;
    let mut y = 0;

    let mut facing = 90;

    for action in actions {
        use ShipActions::*;
        match action {
            North(n) => y -= n,
            South(n) => y += n,
            East(n) => x += n,
            West(n) => x -= n,
            Left(n) => facing -= n,
            Right(n) => facing += n,
            Foreward(n) => match (facing % 360) / 90 {
                0 => y -= n,
                1 => x += n,
                2 => y += n,
                3 => x -= n,
                _ => panic!("bad facing {}", facing),
            },
        }
    }

    (x.abs() + y.abs()) as u64
}

fn day_twelve_b(input: &str) -> u64 {
    let actions: Vec<_> = input
        .lines()
        .filter_map(|l| {
            let n = l[1..].trim().parse().ok();

            if let Some(n) = n {
                use ShipActions::*;
                let v = match &l[0..1] {
                    "N" => North(n),
                    "S" => South(n),
                    "E" => East(n),
                    "W" => West(n),
                    "L" => Left(n),
                    "R" => Right(n),
                    "F" => Foreward(n),
                    _ => return None,
                };
                Some(v)
            } else {
                None
            }
        })
        .collect();

    let mut x = 0;
    let mut y = 0;

    let mut way_x = 10;
    let mut way_y = -1;

    for action in actions {
        use ShipActions::*;
        match action {
            North(n) => way_y -= n,
            South(n) => way_y += n,
            East(n) => way_x += n,
            West(n) => way_x -= n,
            Left(n) => {
                let count = n / 90;

                for _ in 0..count {
                    let temp = way_x;
                    way_x = way_y;
                    way_y = -temp;
                }
            }
            Right(n) => {
                let count = n / 90;

                for _ in 0..count {
                    let temp = way_x;
                    way_x = -way_y;
                    way_y = temp;
                }
            }
            Foreward(n) => {
                x += way_x * n;
                y += way_y * n;
            }
        }
    }

    (x.abs() + y.abs()) as u64
}

#[test]
fn test_day_twelve() {
    let run_a = |input, res| assert_eq!(day_twelve_a(input), res);
    let run_b = |input, res| assert_eq!(day_twelve_b(input), res);

    let i = r#"F10
N3
F7
R90
F11"#;

    run_a(i, 25);
    run_b(i, 286);
}

fn day_thirteen_a(input: &str) -> u32 {
    let mut lines = input.lines();
    let depart: u32 = lines.next().unwrap().trim().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|c| *c != "x")
        .filter_map(|n| n.parse().ok());

    let mut closest_bus = 0;
    let mut closest_depart = u32::MAX;
    for bus in buses {
        for n in 0.. {
            let time = n * bus;
            if time > depart && time < closest_depart {
                closest_depart = time;
                closest_bus = bus;
            }

            if time > closest_depart {
                break;
            }
        }
    }

    (closest_depart - depart) * closest_bus
}

enum Bus {
    Id(u64),
    Any,
}

fn day_thirteen_b(input: &str) -> u64 {
    let mut lines = input.lines();
    let _depart = lines.next();
    let buses = lines.next().unwrap().split(',').filter_map(|n| {
        if n == "x" {
            Some(Bus::Any)
        } else {
            n.parse().ok().map(Bus::Id)
        }
    });

    let mut t_off = 0;
    let mut bus_reqs = Vec::new();
    let mut max_bus = 0;
    let mut max_bus_offset = 0;
    for bus in buses {
        match bus {
            Bus::Id(n) => {
                if n > max_bus {
                    max_bus = n;
                    max_bus_offset = t_off;
                }

                bus_reqs.push((n, t_off));
            }
            _ => (),
        }
        t_off += 1;
    }

    let mut step_by = max_bus;
    let mut n = max_bus - max_bus_offset;
    let mut last_n = n;
    let mut match_count = 1;
    let mut found_next_match = false;
    loop {
        let mut matches = 0;
        for (id, offset) in bus_reqs.iter() {
            if (n + offset) % id == 0 {
                matches += 1;
            } else {
                break;
            }
        }

        if matches == bus_reqs.len() {
            return n;
        }

        if matches > match_count {
            if found_next_match {
                match_count = matches;
                found_next_match = false;
                step_by = n - last_n;
            } else {
                last_n = n;
                found_next_match = true;
            }
        }

        n += step_by;
    }
}

#[test]
fn test_day_thirteen() {
    let run_a = |input, res| assert_eq!(day_thirteen_a(input), res);
    let run_b = |input, res| assert_eq!(day_thirteen_b(input), res);

    let i = r#"939
7,13,x,x,59,x,31,19"#;

    run_a(i, 295);
    run_b(i, 1068781);
}

fn day_fourteen_a(input: &str) -> u64 {
    let lines = input.lines();

    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut mem = HashMap::new();

    for line in lines {
        let mut split = line
            .split(&['[', ']', '=', ' '][..])
            .filter(|w| w.trim().len() > 0);
        let op = split.next().unwrap();

        match op {
            "mask" => {
                and_mask = 0;
                or_mask = 0;
                let mask = split.last().unwrap();
                for c in mask.chars() {
                    and_mask <<= 1;
                    or_mask <<= 1;
                    match c {
                        'X' => {
                            and_mask |= 1;
                            or_mask |= 0;
                        }
                        '1' => {
                            and_mask |= 1;
                            or_mask |= 1;
                        }
                        '0' => {
                            and_mask |= 0;
                            or_mask |= 0;
                        }
                        _ => {
                            unreachable!("bad input")
                        }
                    }
                }
            }
            "mem" => {
                let addr: usize = split.next().unwrap().parse().unwrap();
                let mut val: u64 = split.next().unwrap().parse().unwrap();

                val &= and_mask;
                val |= or_mask;

                mem.insert(addr, val);
            }
            _ => (),
        }
    }

    mem.values().sum()
}

fn day_fourteen_b(input: &str) -> u64 {
    let lines = input.lines();

    let mut mem = HashMap::new();

    let mut mask = None;
    for line in lines {
        let mut split = line
            .split(&['[', ']', '=', ' '][..])
            .filter(|w| w.trim().len() > 0);

        let op = split.next().unwrap();
        match op {
            "mask" => {
                let mask_str = split.last().unwrap();
                mask = Some(Mask::new(mask_str));
            }
            "mem" => {
                let addr: u64 = split.next().unwrap().parse().unwrap();
                let val: u64 = split.next().unwrap().parse().unwrap();

                if let Some(mask) = &mask {
                    for addr in mask.addrs(addr) {
                        mem.insert(addr, val);
                    }
                }
            }

            _ => (),
        }
    }

    mem.values().sum()
}

struct Mask {
    or_mask: u64,
    and_mask: u64,
    floating_masks: Vec<u64>,
}

impl Mask {
    fn new(mask: &str) -> Self {
        let mut or_mask = 0;
        let mut floating_mask: u64 = 0;
        for c in mask.chars() {
            or_mask <<= 1;
            floating_mask <<= 1;

            match c {
                'X' => {
                    floating_mask |= 1;
                }
                '1' => {
                    or_mask |= 1;
                }
                '0' => {
                    or_mask |= 0;
                }
                _ => {
                    unreachable!("bad input")
                }
            }
        }
        let permutations = 2u64.pow(floating_mask.count_ones() as u32);
        let limit = 36.min(64 - floating_mask.leading_zeros()) as u64;

        let mut floating_masks = Vec::with_capacity(permutations as usize);

        for n in 0..permutations {
            let mut mask = 0;
            let mut bit_count = 0;
            let mut floating_bits = floating_mask;
            for bit_position in 0..limit {
                if floating_bits & 1 == 1 {
                    let new_bit = ((n >> bit_count) & 1) << bit_position;
                    mask |= new_bit;
                    bit_count += 1;
                }
                floating_bits >>= 1;
            }

            floating_masks.push(mask);
        }

        Self {
            or_mask,
            and_mask: !floating_mask,
            floating_masks,
        }
    }

    fn addrs<'s>(&'s self, mut addr: u64) -> impl 's + Iterator<Item = u64> {
        addr &= self.and_mask;
        addr |= self.or_mask;
        self.floating_masks.iter().map(move |mask| addr | *mask)
    }
}

#[test]
fn test_day_fourteen() {
    let run_a = |input, res| assert_eq!(day_fourteen_a(input), res);
    let run_b = |input, res| assert_eq!(day_fourteen_b(input), res);

    let i = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

    run_a(i, 165);

    let i = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
    run_b(i, 208);
}

fn day_fifteen_a(input: &str) -> u32 {
    day_fifteen(input, 2020)
}

fn day_fifteen_b(input: &str) -> u32 {
    day_fifteen(input, 30000000)
}

fn day_fifteen(input: &str, nth_number: u32) -> u32 {
    let nums = input.trim().split(',').filter_map(|n| n.parse().ok());
    let mut turn = 0;
    let mut map = vec![u32::MAX; nth_number as usize];
    let mut last_num = None;
    for num in nums {
        if let Some(last_num) = last_num {
            map[last_num as usize] = turn - 1;
        }
        last_num = Some(num);
        turn += 1;
    }

    if let Some(mut last_num) = last_num {
        let start = turn - 1;
        let end = nth_number - 1;

        for turn in start..end {
            let val = map[last_num as usize];
            let num = if val != u32::MAX { turn - val } else { 0 };

            map[last_num as usize] = turn;
            last_num = num;
        }

        last_num
    } else {
        0
    }
}

#[test]
fn test_day_fifteen() {
    let run_a = |input, res| assert_eq!(day_fifteen_a(input), res);
    let run_b = |input, res| assert_eq!(day_fifteen_b(input), res);

    let input = r#"0,3,6"#;
    run_a(input, 436);
    run_b(input, 175594);

    let input = r#"1,3,2"#;
    run_a(input, 1);

    let input = r#"2,1,3"#;
    run_a(input, 10);
    let input = r#"1,2,3"#;
    run_a(input, 27);
}

fn day_sixteen_a(input: &str) -> u64 {
    let lines = input.lines();
    let mut mode = 0;

    let mut rules = Vec::new();
    let mut error_num = 0;
    for line in lines {
        match mode {
            0 => {
                if line == "" {
                    mode += 1;
                    continue;
                }
                let mut splits: Vec<_> = line.split_whitespace().collect();
                splits.reverse();
                let mut left_rules = splits[0].split('-');
                let left_a: u64 = left_rules.next().unwrap().parse().unwrap();
                let left_b: u64 = left_rules.next().unwrap().parse().unwrap();
                let left = left_a..=left_b;

                let mut right_rules = splits[2].split('-');
                let right_a: u64 = right_rules.next().unwrap().parse().unwrap();
                let right_b: u64 = right_rules.next().unwrap().parse().unwrap();
                let right = right_a..=right_b;

                rules.push((left, right));
            }
            1 => {
                mode += 1;
            }
            2 => {
                //my ticket
                mode += 1;
            }
            3 => {
                mode += 1;
            }
            4 => {
                mode += 1;
            }
            5 => {
                let values = line.trim().split(',').filter_map(|n| n.parse().ok());

                'outer: for value in values {
                    for r in rules.iter() {
                        if r.0.contains(&value) {
                            continue 'outer;
                        }
                        if r.1.contains(&value) {
                            continue 'outer;
                        }
                    }

                    error_num += value;
                }
            }
            _ => panic!(),
        }
    }

    error_num
}

fn day_sixteen_b(input: &str) -> u64 {
    let lines = input.lines();
    let mut mode = 0;

    let mut rules = Vec::new();
    let mut tickets = Vec::new();
    let mut my_ticket: Vec<u64> = Vec::new();
    for line in lines {
        match mode {
            0 => {
                if line == "" {
                    mode += 1;
                    continue;
                }
                let mut splits: Vec<_> = line.split_whitespace().collect();
                splits.reverse();
                let mut left_rules = splits[0].split('-');
                let left_a: u64 = left_rules.next().unwrap().parse().unwrap();
                let left_b: u64 = left_rules.next().unwrap().parse().unwrap();
                let left = left_a..=left_b;

                let mut right_rules = splits[2].split('-');
                let right_a: u64 = right_rules.next().unwrap().parse().unwrap();
                let right_b: u64 = right_rules.next().unwrap().parse().unwrap();
                let right = right_a..=right_b;

                let depature = splits.last().unwrap() == &"departure";

                rules.push((left, right, depature, false));
            }
            1 => {
                mode += 1;
            }
            2 => {
                my_ticket = line
                    .trim()
                    .split(',')
                    .filter_map(|n| n.parse().ok())
                    .collect();

                mode += 1;
            }
            3 => {
                mode += 1;
            }
            4 => {
                mode += 1;
            }
            5 => {
                let values: Vec<_> = line
                    .trim()
                    .split(',')
                    .filter_map(|n| n.parse().ok())
                    .collect();

                let mut valid = true;
                'outer: for value in values.iter() {
                    for r in rules.iter() {
                        if r.0.contains(value) {
                            continue 'outer;
                        }
                        if r.1.contains(value) {
                            continue 'outer;
                        }
                    }
                    valid = false;
                }

                if valid {
                    tickets.push(values);
                }
            }
            _ => panic!(),
        }
    }

    let mut rule_map = HashMap::new();
    let column_count = my_ticket.len();

    loop {
        for r in rules.iter_mut() {
            if r.3 {
                continue;
            }
            let mut possible_cols = Vec::new();
            'col: for col in 0..column_count {
                if rule_map.contains_key(&col) {
                    continue 'col;
                }
                for t in tickets.iter() {
                    let v = t[col];
                    if !r.0.contains(&v) && !r.1.contains(&v) {
                        continue 'col;
                    }
                }
                possible_cols.push(col);
            }

            if possible_cols.len() == 1 {
                let col = possible_cols.first().unwrap();
                r.3 = true;
                rule_map.insert(*col, r.clone());
            } else if possible_cols.len() == 0 {
                panic!("shiiit");
            }
        }
        if rule_map.len() == column_count {
            break;
        }
    }

    let mut total = 1;
    for (k, v) in rule_map {
        if v.2 {
            total *= my_ticket[k];
        }
    }

    total
}

#[test]
fn test_day_sixteen() {
    let run_a = |input, res| assert_eq!(day_sixteen_a(input), res);

    let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    run_a(input, 71);
}

fn day_seventeen_a(input: &str) -> u64 {
    let mut grid = HashSet::new();

    let lines = input.trim().lines();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_z = -1;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 1;

    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;

            let point = (col, row, 0);

            max_x = max_x.max(col);
            max_y = max_y.max(row);
            min_x = min_x.min(col);
            min_y = min_y.min(row);

            if c == '#' {
                grid.insert(point);
            }
        }
    }

    for _ in 0..6 {
        let mut next_grid = HashSet::new();
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    let point = (x, y, z);
                    let count = count_neighbors(&grid, point);
                    let me = grid.contains(&point);

                    if me {
                        if count == 2 || count == 3 {
                            next_grid.insert(point);
                        } else {
                            next_grid.remove(&point);
                        }
                    } else {
                        if count == 3 {
                            max_x = max_x.max(x);
                            max_y = max_y.max(y);
                            max_z = max_z.max(z);
                            min_x = min_x.min(x);
                            min_y = min_y.min(y);
                            min_z = min_z.min(z);
                            next_grid.insert(point);
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    grid.len() as u64
}

fn count_neighbors(grid: &HashSet<(i32, i32, i32)>, point: (i32, i32, i32)) -> i32 {
    let mut count = 0;
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            for z_off in -1..=1 {
                if x_off == 0 && y_off == 0 && z_off == 0 {
                    continue;
                }
                let x = point.0 + x_off;
                let y = point.1 + y_off;
                let z = point.2 + z_off;
                let p = (x, y, z);

                if grid.contains(&p) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_neighbors_4d(grid: &HashSet<(i32, i32, i32, i32)>, point: (i32, i32, i32, i32)) -> i32 {
    let mut count = 0;
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            for z_off in -1..=1 {
                for w_off in -1..=1 {
                    if x_off == 0 && y_off == 0 && z_off == 0 && w_off == 0 {
                        continue;
                    }
                    let x = point.0 + x_off;
                    let y = point.1 + y_off;
                    let z = point.2 + z_off;
                    let w = point.3 + w_off;
                    let p = (x, y, z, w);

                    if grid.contains(&p) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn day_seventeen_b(input: &str) -> u64 {
    let mut grid = HashSet::new();

    let lines = input.trim().lines();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_z = -1;
    let mut min_w = -1;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 1;
    let mut max_w = 1;

    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;

            let point = (col, row, 0, 0);

            max_x = max_x.max(col);
            max_y = max_y.max(row);
            min_x = min_x.min(col);
            min_y = min_y.min(row);

            if c == '#' {
                grid.insert(point);
            }
        }
    }

    for _ in 0..6 {
        let mut next_grid = HashSet::new();
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    for w in (min_w - 1)..=(max_w + 1) {
                        let point = (x, y, z, w);
                        let count = count_neighbors_4d(&grid, point);
                        let me = grid.contains(&point);

                        if me {
                            if count == 2 || count == 3 {
                                next_grid.insert(point);
                            } else {
                                next_grid.remove(&point);
                            }
                        } else {
                            if count == 3 {
                                max_x = max_x.max(x);
                                max_y = max_y.max(y);
                                max_z = max_z.max(z);
                                max_w = max_w.max(w);
                                min_x = min_x.min(x);
                                min_y = min_y.min(y);
                                min_z = min_z.min(z);
                                min_w = min_w.min(w);
                                next_grid.insert(point);
                            }
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    grid.len() as u64
}

#[test]
fn test_day_seventeen() {
    let run_a = |input, res| assert_eq!(day_seventeen_a(input), res);
    let run_b = |input, res| assert_eq!(day_seventeen_b(input), res);

    let i = r#".#.
..#
###"#;

    run_a(i, 112);
    run_b(i, 848);
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Value(u64),
    Op(Op),
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

impl Op {
    fn precedence(&self) -> i32 {
        match *self {
            Op::Add => 2,
            Op::Mul => 1,
            _ => 0,
        }
    }
}

fn day_eighteen_a(input: &str) -> u64 {
    let mut result = 0;
    for line in input.trim().lines() {
        result += eval_expression(line, true);
    }
    result
}

fn day_eighteen_b(input: &str) -> u64 {
    let mut result = 0;
    for line in input.trim().lines() {
        result += eval_expression(line, false);
    }
    result
}

fn eval_expression(expression: &str, equal_precedence: bool) -> u64 {
    let mut tokens = Vec::with_capacity(expression.len());
    let mut value_builder = None;
    for c in expression.chars() {
        if c.is_ascii_digit() {
            let v = (c as u8 - '0' as u8) as u64;
            value_builder = Some((value_builder.unwrap_or(0) * 10) + v);
        } else if let Some(val) = value_builder {
            tokens.push(Token::Value(val));
            value_builder = None;
        }
        match c {
            '+' => tokens.push(Token::Op(Op::Add)),
            '*' => tokens.push(Token::Op(Op::Mul)),
            '(' => tokens.push(Token::Op(Op::OpenParen)),
            ')' => tokens.push(Token::Op(Op::CloseParen)),
            _ => (),
        }
    }

    if let Some(val) = value_builder {
        tokens.push(Token::Value(val));
    }

    let mut output = Vec::with_capacity(tokens.len());
    let mut operators = Vec::with_capacity(tokens.len());
    for token in tokens {
        match token {
            Token::Value(n) => output.push(Token::Value(n)),
            Token::Op(Op::OpenParen) => operators.push(Op::OpenParen),
            Token::Op(Op::CloseParen) => {
                while let Some(op) = operators.pop() {
                    match op {
                        Op::OpenParen => {
                            break;
                        }
                        op => output.push(Token::Op(op)),
                    }
                }
            }
            Token::Op(op) => {
                while let Some(next_op) = operators.last() {
                    match next_op {
                        Op::OpenParen | Op::CloseParen => {
                            break;
                        }
                        next_op if equal_precedence || next_op.precedence() >= op.precedence() => {
                            let next_op = operators.pop().unwrap();
                            output.push(Token::Op(next_op));
                        }
                        _ => {
                            break;
                        }
                    }
                }
                operators.push(op)
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(Token::Op(op));
    }

    let mut eval_stack = Vec::with_capacity(output.len());
    for token in output {
        match token {
            Token::Value(n) => eval_stack.push(n),
            Token::Op(Op::Add) => {
                let lhs = eval_stack.pop().unwrap();
                let rhs = eval_stack.pop().unwrap();
                eval_stack.push(lhs + rhs);
            }
            Token::Op(Op::Mul) => {
                let lhs = eval_stack.pop().unwrap();
                let rhs = eval_stack.pop().unwrap();
                eval_stack.push(lhs * rhs);
            }
            _ => unreachable!("invalid rpn"),
        }
    }

    eval_stack.pop().unwrap()
}

#[test]
fn test_day_eighteen() {
    let run_a = |input, res| assert_eq!(day_eighteen_a(input), res);
    let run_b = |input, res| assert_eq!(day_eighteen_b(input), res);

    let i = r#"1 + 2 * 3 + 4 * 5 + 6"#;
    run_a(i, 71);
    run_b(i, 231);

    let i = r#"1 + (2 * 3) + (4 * (5 + 6))"#;
    run_a(i, 51);
    run_b(i, 51);

    let i = r#"2 * 3 + (4 * 5)"#;
    run_a(i, 26);
    run_b(i, 46);

    let i = r#"5 + (8 * 3 + 9 + 3 * 4 * 3)"#;
    run_a(i, 437);
    run_b(i, 1445);

    let i = r#"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"#;
    run_a(i, 12240);
    run_b(i, 669060);

    let i = r#"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#;
    run_a(i, 13632);
    run_b(i, 23340);
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
    rec_count: usize,
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
            let mut state = MatchState {
                idx: 0,
                rec_count: 0,
            };
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

fn day_nineteen_a(input: &str) -> usize {
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

fn day_nineteen_b(input: &str) -> usize {
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

#[test]
fn test_day_nineteen() {
    let run_a = |input, res| assert_eq!(day_nineteen_a(input), res);
    let run_b = |input, res| assert_eq!(day_nineteen_b(input), res);

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

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
struct Edges {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

impl Edges {
    fn new(top: u32, bottom: u32, left: u32, right: u32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    fn permutations(self) -> impl Iterator<Item = (Edges, usize)> {
        let mut count = 0;
        std::iter::from_fn(move || {
            count += 1;
            match count {
                1 => Some((self, count)),
                2 => Some((self.rotate(), count)),
                3 => Some((self.rotate().rotate(), count)),
                4 => Some((self.rotate().rotate().rotate(), count)),
                5 => Some((self.vert_flip(), count)),
                6 => Some((self.vert_flip().rotate(), count)),
                7 => Some((self.vert_flip().rotate().rotate(), count)),
                8 => Some((self.vert_flip().rotate().rotate().rotate(), count)),
                _ => None,
            }
        })
    }

    fn vert_flip(self) -> Self {
        let left = self.left.reverse_bits() >> 22;
        let right = self.right.reverse_bits() >> 22;
        let vert_flip = Edges::new(self.bottom, self.top, left, right);
        vert_flip
    }

    fn rotate(self) -> Self {
        let top = self.top.reverse_bits() >> 22;
        let bottom = self.bottom.reverse_bits() >> 22;
        Edges::new(self.right, self.left, top, bottom)
    }

    fn adjacent(self, other: Self) -> Option<RelativePosition> {
        if self.left == other.right {
            Some(RelativePosition::Left)
        } else if self.right == other.left {
            Some(RelativePosition::Right)
        } else if self.top == other.bottom {
            Some(RelativePosition::Above)
        } else if self.bottom == other.top {
            Some(RelativePosition::Below)
        } else {
            None
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        if (x != 0 && x != 9) && (y != 0 && y != 9) {
            panic!("invalid edge cell {} {}", x, y);
        }

        let mut top = self.top;
        let mut bottom = self.bottom;
        let mut left = self.left;
        let mut right = self.right;

        if y == 0 {
            for i_x in 0..10 {
                if i_x == x {
                    return top & 1 == 1;
                }
                top >>= 1;
            }
        } else if y == 9 {
            for i_x in 0..10 {
                if i_x == x {
                    return bottom & 1 == 1;
                }
                bottom >>= 1;
            }
        } else if x == 0 || x == 9 {
            left >>= 1;
            right >>= 1;
            for i_y in 1..9 {
                if x == 0 && i_y == y {
                    return left & 1 == 1;
                }
                left >>= 1;

                if x == 9 && i_y == y {
                    return right & 1 == 1;
                }
                right >>= 1;
            }
        }
        panic!("could not find edge cell")
    }
}

impl std::fmt::Display for Edges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut top = self.top;
        let mut bottom = self.bottom;
        let mut left = self.left;
        let mut right = self.right;

        for _ in 0..10 {
            if top & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            top >>= 1;
        }
        write!(f, "\n")?;

        left >>= 1;
        right >>= 1;
        for _ in 0..8 {
            if left & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            left >>= 1;
            write!(f, "        ")?;

            if right & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            right >>= 1;
            write!(f, "\n")?;
        }

        for _ in 0..10 {
            if bottom & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            bottom >>= 1;
        }

        write!(f, "\n")
    }
}

struct Tile {
    id: u64,
    img: Vec<bool>,
    edges: Edges,
    indexer: Option<Box<dyn TileIndex>>,
}

#[derive(Debug, Copy, Clone)]
enum RelativePosition {
    Left,
    Above,
    Below,
    Right,
}

impl Tile {
    fn new(id: u64) -> Self {
        Self {
            id,
            img: Vec::with_capacity(100),
            edges: Edges::new(0, 0, 0, 0),
            indexer: Some(Box::new(TileIndexIdentity)),
        }
    }

    fn add_row(&mut self, row: &str) {
        assert_eq!(row.len(), 10);
        assert!(row.len() < 100);

        for c in row.chars() {
            self.img.push(c == '#');
        }

        if self.img.len() == 100 {
            let mut bottom = 0;
            for &n in &self.img[0..10] {
                bottom <<= 1;
                bottom |= if n { 1 } else { 0 };
            }
            let mut top = 0;
            for &n in &self.img[90..100] {
                top <<= 1;
                top |= if n { 1 } else { 0 };
            }

            let mut right = 0;
            let right_range = (0..100).step_by(10);
            for n in right_range {
                right <<= 1;
                right |= if self.img[n] { 1 } else { 0 };
            }

            let mut left = 0;
            let left_range = (9..100).step_by(10);
            for n in left_range {
                left <<= 1;
                left |= if self.img[n] { 1 } else { 0 };
            }

            self.edges = Edges::new(top, bottom, left, right);
        }
    }

    fn aligns_with(&self, other: &Tile) -> bool {
        if self.id == other.id {
            false
        } else {
            for (e, _) in other.edges.permutations() {
                if self.edges.adjacent(e).is_some() {
                    return true;
                }
            }
            false
        }
    }

    fn aligns_orientation(&self, other: &Tile) -> Option<(RelativePosition, usize)> {
        if self.id == other.id {
            None
        } else {
            for (e, o) in other.edges.permutations() {
                if let Some(relative) = self.edges.adjacent(e) {
                    return Some((relative, o));
                }
            }
            None
        }
    }

    fn set_orientation(&mut self, code: usize) {
        match code {
            1 => (),
            2 => {
                self.rotate();
            }
            3 => {
                self.rotate().rotate();
            }
            4 => {
                self.rotate().rotate().rotate();
            }
            5 => {
                self.vert_flip();
            }
            6 => {
                self.vert_flip().rotate();
            }
            7 => {
                self.vert_flip().rotate().rotate();
            }
            8 => {
                self.vert_flip().rotate().rotate().rotate();
            }
            _ => panic!("invalid orientation code"),
        };
    }

    fn vert_flip(&mut self) -> &mut Self {
        self.edges = self.edges.vert_flip();
        let old = self.indexer.take().unwrap_or(Box::new(TileIndexIdentity));
        self.indexer = Some(Box::new(VertFlip(old)));
        self
    }

    fn rotate(&mut self) -> &mut Self {
        self.edges = self.edges.rotate();
        let old = self.indexer.take().unwrap_or(Box::new(TileIndexIdentity));
        self.indexer = Some(Box::new(Rotate(old)));
        self
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        let i = self.indexer.as_ref().unwrap();
        let (x, y) = i.coords(x, y);
        self.img[y * 10 + x]
    }

    #[allow(dead_code)]
    fn valid_edge(&self) -> bool {
        let mut is_match = true;
        for y in 0..10 {
            for x in 0..10 {
                if y == 0 || x == 0 || y == 9 || x == 9 {
                    let cell = self.get_cell(x, y);
                    let edge_cell = self.edges.get_cell(x, y);
                    if cell != edge_cell {
                        is_match = false;
                    }
                }
            }
        }
        is_match
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..10 {
            for x in 0..10 {
                let c = if self.get_cell(x, y) { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

trait TileIndex {
    fn coords(&self, x: usize, y: usize) -> (usize, usize);
}

struct TileIndexIdentity;

impl TileIndex for TileIndexIdentity {
    fn coords(&self, x: usize, y: usize) -> (usize, usize) {
        (9 - x, (y as isize - 9).abs() as usize)
    }
}

struct VertFlip(Box<dyn TileIndex>);
impl TileIndex for VertFlip {
    fn coords(&self, x: usize, y: usize) -> (usize, usize) {
        let y = (y as isize - 9).abs() as usize;
        self.0.coords(x, y)
    }
}

struct Rotate(Box<dyn TileIndex>);
impl TileIndex for Rotate {
    fn coords(&self, x: usize, y: usize) -> (usize, usize) {
        let temp = y;
        let y = x;
        let x = 10 - (temp + 1);
        self.0.coords(x, y)
    }
}

struct TilePattern {
    pub width: usize,
    pub height: usize,
    pub cell_count: u64,
    cells: Vec<bool>,
    working_buf: Vec<bool>,
}

impl TilePattern {
    fn new(data: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut cell_count = 0;
        let mut cells = Vec::new();
        for line in data.lines() {
            width = 0;
            for c in line.chars() {
                let active = c == '#';
                if active {
                    cell_count += 1;
                }
                cells.push(active);
                width += 1;
            }
            height += 1;
        }
        let working_buf = Vec::with_capacity(cells.len());

        Self {
            width,
            height,
            cell_count,
            cells,
            working_buf,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.cells[y * self.width + x])
        }
    }

    fn rotate(&mut self) -> &mut Self {
        self.working_buf.clear();

        for x in 0..self.width {
            for y in 0..self.height {
                self.working_buf
                    .push(self.get_cell(self.width - x - 1, y).unwrap());
            }
        }

        let height = self.height;
        self.height = self.width;
        self.width = height;

        std::mem::swap(&mut self.cells, &mut self.working_buf);

        self
    }

    fn vert_flip(&mut self) -> &mut Self {
        self.working_buf.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                self.working_buf
                    .push(self.get_cell(x, self.height - y - 1).unwrap())
            }
        }

        std::mem::swap(&mut self.cells, &mut self.working_buf);

        self
    }
}

impl std::fmt::Display for TilePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();
                if cell {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

struct TileImage {
    tiles: HashMap<u64, Tile>,
    position_to_tile: HashMap<(usize, usize), u64>,
    tile_to_position: HashMap<u64, (usize, usize)>,
    min: (usize, usize),
    max: (usize, usize),
}

const IMAGE_CENTER: usize = usize::MAX / 2;

impl TileImage {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            position_to_tile: HashMap::new(),
            tile_to_position: HashMap::new(),
            min: (IMAGE_CENTER, IMAGE_CENTER),
            max: (IMAGE_CENTER, IMAGE_CENTER),
        }
    }

    fn process_pattern(&self, pattern: &str) -> u64 {
        let mut pattern = TilePattern::new(pattern);
        let (image_x_dim, image_y_dim) = self.borderless_dims();
        let mut cell_count = 0;
        for x in 0..image_x_dim {
            for y in 0..image_y_dim {
                if self.borderless_get_cell(x, y).unwrap() {
                    cell_count += 1;
                }
            }
        }

        let count_matches = |pattern: &TilePattern| {
            let mut count = 0;
            for x in 0..image_x_dim {
                'offset_loop: for y in 0..image_y_dim {
                    for p_x in 0..pattern.width {
                        for p_y in 0..pattern.height {
                            if let Some(image_cell) = self.borderless_get_cell(x + p_x, y + p_y) {
                                let p_cell = pattern.get_cell(p_x, p_y).unwrap();
                                if p_cell && !image_cell {
                                    continue 'offset_loop;
                                }
                            } else {
                                continue 'offset_loop;
                            }
                        }
                    }

                    count += 1;
                }
            }

            count
        };

        let mut orientation = 0;
        while orientation < 8 {
            match orientation {
                0 => (),
                1 | 2 | 3 | 5 | 6 | 7 => {
                    pattern.rotate();
                }
                4 => {
                    pattern.rotate().vert_flip();
                }
                _ => unreachable!("ran past end of loop"),
            }
            let count = count_matches(&pattern);
            if count != 0 {
                return cell_count - (count * pattern.cell_count);
            }
            orientation += 1;
        }

        panic!("pattern never found")
    }

    fn attempt_insert(&mut self, mut tile: Tile) -> Result<(), Tile> {
        let id = tile.id;
        if self.tiles.len() == 0 {
            let pos = (IMAGE_CENTER, IMAGE_CENTER);
            self.tiles.insert(id, tile);
            self.tile_to_position.insert(id, pos);
            self.position_to_tile.insert(pos, id);
            Ok(())
        } else {
            let mut found_place = None;
            for existing_tile in self.tiles.values() {
                if let Some((relative, o_code)) = existing_tile.aligns_orientation(&tile) {
                    tile.set_orientation(o_code);

                    let mut pos = self
                        .tile_to_position
                        .get(&existing_tile.id)
                        .unwrap()
                        .clone();
                    match relative {
                        RelativePosition::Left => pos.0 -= 1,
                        RelativePosition::Above => pos.1 -= 1,
                        RelativePosition::Below => pos.1 += 1,
                        RelativePosition::Right => pos.0 += 1,
                    }

                    if self.position_to_tile.contains_key(&pos) {
                        continue;
                    } else {
                        found_place = Some(pos);
                        break;
                    }
                }
            }
            if let Some(pos) = found_place {
                self.min = (self.min.0.min(pos.0), self.min.1.min(pos.1));
                self.max = (self.max.0.max(pos.0), self.max.1.max(pos.1));
                self.tiles.insert(id, tile);
                self.tile_to_position.insert(id, pos);
                self.position_to_tile.insert(pos, id);
                Ok(())
            } else {
                Err(tile)
            }
        }
    }

    fn dims(&self) -> (usize, usize) {
        (
            (self.max.0 - self.min.0 + 1) * 10,
            (self.max.1 - self.min.1 + 1) * 10,
        )
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        let tile_x = x % 10;
        let tile_y = y % 10;
        let image_x = (x / 10) + self.min.0;
        let image_y = (y / 10) + self.min.1;
        let tile_id = self.position_to_tile.get(&(image_x, image_y))?;
        let tile = self.tiles.get(tile_id)?;
        Some(tile.get_cell(tile_x, tile_y))
    }

    fn borderless_dims(&self) -> (usize, usize) {
        (
            (self.max.0 - self.min.0 + 1) * 8,
            (self.max.1 - self.min.1 + 1) * 8,
        )
    }

    fn borderless_get_cell(&self, x: usize, y: usize) -> Option<bool> {
        let tile_x = x % 8;
        let tile_y = y % 8;
        let image_x = (x / 8) + self.min.0;
        let image_y = (y / 8) + self.min.1;
        let tile_id = self.position_to_tile.get(&(image_x, image_y))?;
        let tile = self.tiles.get(tile_id)?;
        Some(tile.get_cell(tile_x + 1, tile_y + 1))
    }
}

struct Borderless<'a>(&'a TileImage);

impl<'a> std::fmt::Display for Borderless<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_dim, y_dim) = self.0.borderless_dims();
        for y in 0..=y_dim {
            for x in 0..=x_dim {
                let c = if let Some(b) = self.0.borderless_get_cell(x, y) {
                    if b {
                        '#'
                    } else {
                        '.'
                    }
                } else {
                    ' '
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for TileImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_dim, y_dim) = self.dims();
        for y in 0..=y_dim {
            for x in 0..=x_dim {
                let c = if let Some(b) = self.get_cell(x, y) {
                    if b {
                        '#'
                    } else {
                        '.'
                    }
                } else {
                    ' '
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn day_twenty_a(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut count = 0;
    let mut all_tiles = Vec::new();
    let mut cur_tile = None;
    for line in lines {
        if line.len() == 0 {
            count = 0;
            continue;
        }
        if count == 0 {
            let id = line[5..9].parse().unwrap();
            if let Some(cur_tile) = cur_tile.take() {
                all_tiles.push(cur_tile);
            }
            cur_tile = Some(Tile::new(id));
        } else if let Some(cur_tile) = cur_tile.as_mut() {
            cur_tile.add_row(line);
        }

        count += 1;
    }
    if let Some(cur_tile) = cur_tile.take() {
        all_tiles.push(cur_tile);
    }

    let mut corners = 1;

    'outer: for tile in &all_tiles {
        let mut align_count = 0;
        for other_tile in &all_tiles {
            if tile.aligns_with(other_tile) {
                align_count += 1;
                if align_count > 2 {
                    continue 'outer;
                }
            }
        }

        if align_count == 2 {
            corners *= tile.id;
        }
    }

    corners
}

fn day_twenty_b(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut count = 0;
    let mut all_tiles = VecDeque::new();
    let mut cur_tile: Option<Tile> = None;
    for line in lines {
        if line.len() == 0 {
            count = 0;
            continue;
        }
        if count == 0 {
            let id = line[5..9].parse().unwrap();
            if let Some(cur_tile) = cur_tile.take() {
                all_tiles.push_back(cur_tile);
            }
            cur_tile = Some(Tile::new(id));
        } else if let Some(cur_tile) = cur_tile.as_mut() {
            cur_tile.add_row(line);
        }

        count += 1;
    }
    if let Some(cur_tile) = cur_tile.take() {
        all_tiles.push_back(cur_tile);
    }

    let mut image = TileImage::new();

    while let Some(next_tile) = all_tiles.pop_front() {
        if let Err(tile) = image.attempt_insert(next_tile) {
            all_tiles.push_back(tile);
        }
    }

    let pattern = r#"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "#;

    image.process_pattern(pattern)
}

#[test]
fn test_day_twenty() {
    let run_a = |input, res| assert_eq!(day_twenty_a(input), res);
    let run_b = |input, res| assert_eq!(day_twenty_b(input), res);

    let i = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

    run_a(i, 20899048083289);
    run_b(i, 273);
}

fn day_twenty_one_a(input: &str) -> u64 {
    let mut all_allergens: HashMap<_, Vec<_>> = HashMap::new();
    let mut all_ingredients = HashMap::new();
    for line in input.trim().lines() {
        let mut line = line.trim().trim_end_matches(')').split(" (contains ");

        let ingredients = line.next().unwrap();
        let allergens = line.next().unwrap();

        let ingredients: HashSet<_> = ingredients.split_whitespace().collect();

        let allergens = allergens.split(',').map(|a| a.trim());
        for allergen in allergens {
            all_allergens
                .entry(allergen)
                .and_modify(|potential_ingredients| {
                    let mut union: Vec<&str> = Vec::new();
                    for ingredient in potential_ingredients.into_iter() {
                        if ingredients.contains(*ingredient) {
                            union.push(*ingredient);
                        }
                    }

                    *potential_ingredients = union;
                })
                .or_insert_with(|| ingredients.iter().map(|s| s.clone()).collect());
        }

        for ingredient in ingredients {
            all_ingredients
                .entry(ingredient)
                .and_modify(|existing_count| *existing_count += 1)
                .or_insert(1);
        }
    }

    for (_allergen, potential_ingredients) in all_allergens {
        for ingredient in potential_ingredients {
            all_ingredients.remove(ingredient);
        }
    }

    all_ingredients.values().sum()
}

fn day_twenty_one_b(input: &str) -> String {
    let mut all_allergens: HashMap<_, Vec<_>> = HashMap::new();

    for line in input.trim().lines() {
        let mut line = line.trim().trim_end_matches(')').split(" (contains ");

        let ingredients = line.next().unwrap();
        let allergens = line.next().unwrap();

        let ingredients: HashSet<_> = ingredients.split_whitespace().collect();

        let allergens = allergens.split(',').map(|a| a.trim());
        for allergen in allergens {
            all_allergens
                .entry(allergen)
                .and_modify(|potential_ingredients| {
                    let mut union: Vec<&str> = Vec::new();
                    for ingredient in potential_ingredients.into_iter() {
                        if ingredients.contains(*ingredient) {
                            union.push(*ingredient);
                        }
                    }

                    *potential_ingredients = union;
                })
                .or_insert_with(|| ingredients.iter().map(|s| s.clone()).collect());
        }
    }

    let mut allergen_mapping: Vec<(&str, &str)> = Vec::new();

    while allergen_mapping.len() < all_allergens.len() {
        for (allergen, potential_ingredients) in all_allergens.iter_mut() {
            if potential_ingredients.len() == 0 {
                continue;
            }

            let mut i = 0;
            while i < potential_ingredients.len() {
                if allergen_mapping
                    .iter()
                    .any(|am| am.1 == potential_ingredients[i])
                {
                    potential_ingredients.remove(i);
                } else {
                    i += 1;
                }
            }

            if potential_ingredients.len() == 1 {
                let ingredient = potential_ingredients.remove(0);
                allergen_mapping.push((*allergen, ingredient));
            }
        }
    }

    allergen_mapping.sort_by_key(|am| am.0);

    let mut result = String::new();
    let mut first = true;
    for (_allergen, ingredient) in allergen_mapping {
        if !first {
            result.push(',');
        }
        result.push_str(ingredient);
        first = false;
    }

    result
}

#[test]
fn test_day_twenty_one() {
    let run_a = |input, res| assert_eq!(day_twenty_one_a(input), res);
    let run_b = |input, res| assert_eq!(day_twenty_one_b(input), res);

    let i = r#"
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"#;

    run_a(i, 5);
    run_b(i, "mxmxvkd,sqjhc,fvjkl");
}

fn day_twenty_two_parse(input: &str) -> Hands {
    let mut hands = Hands::new();
    let mut second_deck = false;

    for line in input.trim().lines() {
        if line.is_empty() {
            second_deck = true;
            continue;
        }

        if line.starts_with('P') {
            continue;
        }

        let n = line.parse().expect("valid card in range 0..256");

        if second_deck {
            hands.player_two_mut().push(n);
        } else {
            hands.player_one_mut().push(n);
        }
    }

    hands
}

fn day_twenty_two_a(input: &str) -> u64 {
    let mut hands = day_twenty_two_parse(input);

    while !hands.game_over() {
        let (one, two) = hands.pop();

        if one > two {
            hands.player_one_mut().push(one);
            hands.player_one_mut().push(two);
        } else {
            hands.player_two_mut().push(two);
            hands.player_two_mut().push(one);
        }
    }

    if hands.player_two().is_empty() {
        hands.player_one().score()
    } else {
        hands.player_two().score()
    }
}

fn day_twenty_two_b(input: &str) -> u64 {
    let mut hands = day_twenty_two_parse(input);

    let winner = combat_game(&mut hands);

    match winner {
        Winner::PlayerOne => hands.player_one().score(),
        Winner::PlayerTwo => hands.player_two().score(),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Winner {
    PlayerOne,
    PlayerTwo,
}

fn combat_game(hands: &mut Hands) -> Winner {
    let mut previous_game_states = HashSet::with_capacity(512);

    while !hands.game_over() {
        combat_round(hands);
        let state = HandState::from(&*hands);
        if !previous_game_states.insert(state) {
            return Winner::PlayerOne;
        }
    }

    if hands.player_two().is_empty() {
        Winner::PlayerOne
    } else {
        Winner::PlayerTwo
    }
}

fn combat_round(hands: &mut Hands) {
    let (one, two) = hands.pop();

    let winner = if hands.player_one().len() >= one && hands.player_two().len() >= two {
        let mut sub_p_one = hands.player_one().iter().take(one as usize);
        let mut sub_p_two = hands.player_two().iter().take(two as usize);

        let mut sub_hands =
            std::iter::from_fn(move || match (sub_p_one.next(), sub_p_two.next()) {
                (None, None) => None,
                h => Some(h),
            })
            .collect();

        combat_game(&mut sub_hands)
    } else if one > two {
        Winner::PlayerOne
    } else if two > one {
        Winner::PlayerTwo
    } else {
        panic!("someone has to have the higher card, invalid decks")
    };

    match winner {
        Winner::PlayerOne => {
            hands.player_one_mut().push(one);
            hands.player_one_mut().push(two);
        }
        Winner::PlayerTwo => {
            hands.player_two_mut().push(two);
            hands.player_two_mut().push(one);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct HandState {
    cards: [u8; 50],
    player_one_len: u8,
    player_two_len: u8,
}

impl From<&Hands> for HandState {
    fn from(hands: &Hands) -> Self {
        let mut cards = [0; 50];
        let player_one_len = hands.player_one().len();
        let player_two_len = hands.player_two().len();

        for (i, c) in hands
            .player_one()
            .iter()
            .chain(hands.player_two().iter())
            .enumerate()
        {
            cards[i] = c;
        }

        Self {
            cards,
            player_one_len,
            player_two_len,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Hand {
    cards: [u8; 64],
    len: u8,
    cursor: u8,
}

impl Hand {
    fn new() -> Self {
        Self {
            cards: [0; 64],
            len: 0,
            cursor: 0,
        }
    }

    fn push(&mut self, card: u8) {
        if self.len() == 50 {
            panic!("overflowed hand")
        }
        let idx = (self.cursor + self.len) & 0x3F;
        self.cards[idx as usize] = card;
        self.len += 1;
    }

    fn pop(&mut self) -> u8 {
        if self.len() == 0 {
            panic!("popped card from empty hand")
        } else {
            let res = self.cards[(self.cursor & 0x3f) as usize];
            self.cursor = self.cursor.wrapping_add(1);
            self.len -= 1;
            res
        }
    }

    fn len(&self) -> u8 {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        let start = self.cursor & 0x3f;
        let end = (start + self.len) & 0x3f;
        let mut idx = start;
        std::iter::from_fn(move || {
            if idx == end {
                None
            } else {
                let res = self.cards[idx as usize];
                idx = (idx + 1) & 0x3f;
                Some(res)
            }
        })
    }

    fn score(&self) -> u64 {
        self.iter()
            .enumerate()
            .map(|(i, c)| c as u64 * (self.len() as u64 - i as u64))
            .sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Hands {
    player_one: Hand,
    player_two: Hand,
}

impl Hands {
    fn new() -> Self {
        Self {
            player_one: Hand::new(),
            player_two: Hand::new(),
        }
    }

    fn pop(&mut self) -> (u8, u8) {
        (self.player_one_mut().pop(), self.player_two_mut().pop())
    }

    fn player_one_mut(&mut self) -> &mut Hand {
        &mut self.player_one
    }

    fn player_one(&self) -> &Hand {
        &self.player_one
    }

    fn player_two_mut(&mut self) -> &mut Hand {
        &mut self.player_two
    }

    fn player_two(&self) -> &Hand {
        &self.player_two
    }

    fn game_over(&self) -> bool {
        self.player_one().is_empty() || self.player_two().is_empty()
    }
}

impl std::iter::FromIterator<(Option<u8>, Option<u8>)> for Hands {
    fn from_iter<T: IntoIterator<Item = (Option<u8>, Option<u8>)>>(iter: T) -> Self {
        let iter = iter.into_iter();

        let mut player_one = [0; 64];
        let mut player_two = [0; 64];
        let mut player_one_len = 0;
        let mut player_two_len = 0;

        for (idx, (p1, p2)) in iter.enumerate() {
            if let Some(p1) = p1 {
                player_one[idx] = p1;
                player_one_len += 1;
            }
            if let Some(p2) = p2 {
                player_two[idx] = p2;
                player_two_len += 1;
            }
        }

        let player_one = Hand {
            cards: player_one,
            len: player_one_len,
            cursor: 0,
        };

        let player_two = Hand {
            cards: player_two,
            len: player_two_len,
            cursor: 0,
        };

        Self {
            player_one,
            player_two,
        }
    }
}

impl std::fmt::Display for Hands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P1: [")?;
        for c in self.player_one().iter() {
            write!(f, " {}", c)?;
        }
        write!(f, " ]  P2: [")?;
        for c in self.player_two().iter() {
            write!(f, " {}", c)?;
        }
        write!(f, " ]")
    }
}

#[test]
fn test_day_twenty_two() {
    let run_a = |input, res| assert_eq!(day_twenty_two_a(input), res);
    let run_b = |input, res| assert_eq!(day_twenty_two_b(input), res);

    let i = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

    run_a(i, 306);
    run_b(i, 291);
}

fn day_twenty_three_a(input: &str) -> u64 {
    let cups = day_twenty_three(input, 9, 100);
    let one = cups.iter().position(|n| *n == 1).unwrap();
    cups.iter()
        .cycle()
        .skip(one + 1)
        .take(8)
        .fold(0, |mut acc, &n| {
            acc *= 10;
            acc += n as u64;
            acc
        })
}

fn day_twenty_three_b(_input: &str) -> &'static str {
    "spent one hour brute-forcing the answer - needs a complete re-work"
    /*
        let cups = day_twenty_three(input, 1_000_000, 10_000_000);
        let one = cups.iter().position(|n| *n == 1).unwrap();
        let mut results = cups.iter().cycle().skip(one + 1).copied();

        let a = results.next().unwrap() as u64;
        let b = results.next().unwrap() as u64;

        a * b
    */
}

fn day_twenty_three(input: &str, max: u32, iterations: u32) -> Vec<u32> {
    let mut cups: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| (n as u8 - b'0') as u32)
        .chain(10..=max)
        .collect();

    let mut current_cup = 0;
    for i in 0..iterations {
        if i % 1_000_000 == 0 && i > 0 {
            println!("{}", i);
        }
        let mut pick_up = (current_cup + 1) % cups.len();
        let cup_val = cups[current_cup % cups.len()];
        let mut dest = cup_val - 1;
        if dest == 0 {
            dest = max;
        }
        let one = cups.remove(pick_up);
        if pick_up < current_cup {
            current_cup -= 1;
        }
        if pick_up >= cups.len() {
            pick_up = 0;
        }
        let two = cups.remove(pick_up);
        if pick_up < current_cup {
            current_cup -= 1;
        }
        if pick_up >= cups.len() {
            pick_up = 0;
        }
        let three = cups.remove(pick_up);
        if pick_up < current_cup {
            current_cup -= 1;
        }

        while dest == one || dest == two || dest == three {
            if dest == 1 {
                dest = max;
            } else {
                dest -= 1;
            }
        }

        let find = cups
            .iter()
            .enumerate()
            .find(|(_p, n)| **n == dest)
            .unwrap()
            .0;

        let find = find + 1;
        let find = if find >= (max - 3) as usize { 0 } else { find } as usize;
        cups.insert(find, three);
        cups.insert(find, two);
        cups.insert(find, one);

        if find <= current_cup {
            current_cup += 3;
        }

        current_cup = (current_cup + 1) % cups.len();
    }

    cups
}

#[test]
fn test_day_twenty_three() {
    let run_a = |input, res| assert_eq!(day_twenty_three_a(input), res);
    //let run_b = |input, res| assert_eq!(day_twenty_three_b(input), res);

    let i = r#"389125467"#;

    run_a(i, 67384529);
    //run_b(i, 149245887792);
}

fn day_twenty_four_a(input: &str) -> u64 {
    let tiles: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| {
            let mut tile = Vec::new();
            let mut idx = 0;
            while idx < line.len() {
                let dir = if idx == line.len() - 1 {
                    if &line[idx..] == "e" {
                        HexDir::East
                    } else {
                        HexDir::West
                    }
                } else {
                    match &line[idx..idx + 2] {
                        "sw" => HexDir::SouthWest,
                        "se" => HexDir::SouthEast,
                        "nw" => HexDir::NorthWest,
                        "ne" => HexDir::NorthEast,
                        d if d.starts_with("e") => HexDir::East,
                        d if d.starts_with("w") => HexDir::West,
                        _ => unreachable!(),
                    }
                };

                match dir {
                    HexDir::East | HexDir::West => idx += 1,
                    _ => idx += 2,
                }

                tile.push(dir);
            }

            tile
        })
        .collect();

    let mut flips: HashMap<_, bool> = HashMap::new();

    for tile in tiles.iter() {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        for movement in tile.iter() {
            let (o_x, o_y, o_z) = movement.offset();
            x += o_x;
            y += o_y;
            z += o_z;
        }

        flips
            .entry((x, y, z))
            .and_modify(|f| *f = !*f)
            .or_insert(true);
    }

    flips.values().filter(|f| **f).count() as u64
}

fn day_twenty_four_b(input: &str) -> u64 {
    let tiles: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| {
            let mut tile = Vec::new();
            let mut idx = 0;
            while idx < line.len() {
                let dir = if idx == line.len() - 1 {
                    if &line[idx..] == "e" {
                        HexDir::East
                    } else {
                        HexDir::West
                    }
                } else {
                    match &line[idx..idx + 2] {
                        "sw" => HexDir::SouthWest,
                        "se" => HexDir::SouthEast,
                        "nw" => HexDir::NorthWest,
                        "ne" => HexDir::NorthEast,
                        d if d.starts_with("e") => HexDir::East,
                        d if d.starts_with("w") => HexDir::West,
                        _ => unreachable!(),
                    }
                };

                match dir {
                    HexDir::East | HexDir::West => idx += 1,
                    _ => idx += 2,
                }

                tile.push(dir);
            }

            tile
        })
        .collect();

    let mut flips: HashMap<_, bool> = HashMap::new();
    let mut max = (0, 0, 0);
    let mut min = (0, 0, 0);

    for tile in tiles.iter() {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        for movement in tile.iter() {
            let (o_x, o_y, o_z) = movement.offset();
            x += o_x;
            y += o_y;
            z += o_z;
        }

        max.0 = max.0.max(x);
        max.1 = max.1.max(y);
        max.2 = max.2.max(z);

        min.0 = min.0.min(x);
        min.1 = min.1.min(y);
        min.2 = min.2.min(z);

        flips
            .entry((x, y, z))
            .and_modify(|f| *f = !*f)
            .or_insert(true);
    }

    max.0 = max.0.max(max.0 + 1);
    max.1 = max.1.max(max.1 + 1);
    max.2 = max.2.max(max.2 + 1);

    min.0 = min.0.min(min.0 - 1);
    min.1 = min.1.min(min.1 - 1);
    min.2 = min.2.min(min.2 - 1);

    let days = 100;

    let mut hex_grid = HexGrid::new(
        min.0 - days,
        max.0 + days,
        min.1 - days,
        max.1 + days,
        min.2 - days,
        max.2 + days,
    );
    let mut next_grid = hex_grid.clone();

    for (pos, v) in flips.iter() {
        hex_grid.set(*pos, *v);
    }

    for _day in 0..days {
        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                for z in min.2..=max.2 {
                    let neighbors = hex_grid.count_neighbors((x, y, z));
                    let t = match hex_grid.get((x, y, z)) {
                        true if neighbors == 0 || neighbors > 2 => false,
                        false if neighbors == 2 => {
                            max.0 = max.0.max(x + 1);
                            max.1 = max.1.max(y + 1);
                            max.2 = max.2.max(z + 1);

                            min.0 = min.0.min(x - 1);
                            min.1 = min.1.min(y - 1);
                            min.2 = min.2.min(z - 1);

                            true
                        }
                        o => o,
                    };

                    next_grid.set((x, y, z), t);
                }
            }
        }
        std::mem::swap(&mut hex_grid, &mut next_grid);
    }

    hex_grid.count()
}

#[derive(Clone)]
struct HexGrid {
    cells: Vec<bool>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    x_size: usize,
    y_size: usize,
    z_size: usize,
}

impl HexGrid {
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Self {
        let x_size = (x_min.abs() + x_max) as usize + 1;
        let y_size = (y_min.abs() + y_max) as usize + 1;
        let z_size = (z_min.abs() + z_max) as usize + 1;
        Self {
            cells: vec![false; x_size * y_size * z_size],
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            x_size,
            y_size,
            z_size,
        }
    }

    fn get(&self, (x, y, z): (i32, i32, i32)) -> bool {
        if x <= self.x_min
            || x >= self.x_max
            || y <= self.y_min
            || y >= self.y_max
            || z <= self.z_min
            || z >= self.z_max
        {
            panic!("hexgrid out of bounds")
        }

        let x = (x + self.x_min.abs()) as usize;
        let y = (y + self.y_min.abs()) as usize;
        let z = (z + self.z_min.abs()) as usize;

        self.cells[(z * self.y_size * self.x_size) + (y * self.x_size) + x]
    }

    fn set(&mut self, (x, y, z): (i32, i32, i32), val: bool) {
        if x <= self.x_min
            || x >= self.x_max
            || y <= self.y_min
            || y >= self.y_max
            || z <= self.z_min
            || z >= self.z_max
        {
            panic!("hexgrid out of bounds")
        }

        let x = (x + self.x_min.abs()) as usize;
        let y = (y + self.y_min.abs()) as usize;
        let z = (z + self.z_min.abs()) as usize;

        self.cells[(z * self.y_size * self.x_size) + (y * self.x_size) + x] = val
    }

    fn count_neighbors(&self, position: (i32, i32, i32)) -> u32 {
        let mut sum = 0;

        let mut do_sum = |dir: HexDir| {
            let new_dir = dir.add(position);
            if self.get(new_dir) {
                sum += 1;
            }
        };

        do_sum(HexDir::East);
        do_sum(HexDir::SouthEast);
        do_sum(HexDir::SouthWest);
        do_sum(HexDir::West);
        do_sum(HexDir::NorthWest);
        do_sum(HexDir::NorthEast);

        sum
    }

    fn count(&self) -> u64 {
        self.cells.iter().filter(|f| **f).count() as u64
    }
}

#[derive(Copy, Clone, Debug)]
enum HexDir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl HexDir {
    fn offset(&self) -> (i32, i32, i32) {
        match self {
            HexDir::West => (-1, 1, 0),
            HexDir::SouthWest => (-1, 0, 1),
            HexDir::SouthEast => (0, -1, 1),
            HexDir::East => (1, -1, 0),
            HexDir::NorthEast => (1, 0, -1),
            HexDir::NorthWest => (0, 1, -1),
        }
    }

    fn add(&self, other: (i32, i32, i32)) -> (i32, i32, i32) {
        let offset = self.offset();
        (other.0 + offset.0, other.1 + offset.1, other.2 + offset.2)
    }
}

#[test]
fn test_day_twenty_four() {
    let run_a = |input, res| assert_eq!(day_twenty_four_a(input), res);
    let run_b = |input, res| assert_eq!(day_twenty_four_b(input), res);

    let i = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

    run_a(i, 10);
    run_b(i, 2208);
}
