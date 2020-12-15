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
L.LLLLL.LL"#;

    run_a(input, 37);
    run_b(input, 26);
}

fn day_eleven_a(input: &str) -> i32 {
    let mut grid: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(move |(column, c)| ((row, column), c))
        })
        .collect();

    let mut next_grid = HashMap::new();

    let mut changed = true;
    let mut occupied = 0;
    while changed {
        changed = false;
        for (&(row, column), c) in grid.iter() {
            let point = (row, column);
            let neighbors = count_neighbors(&grid, point, false);
            match c {
                'L' if neighbors == 0 => {
                    changed = true;
                    next_grid.insert(point, '#');
                    occupied += 1;
                }
                '#' if neighbors >= 4 => {
                    changed = true;
                    next_grid.insert(point, 'L');
                    occupied -= 1;
                }
                c => {
                    next_grid.insert(point, *c);
                }
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    occupied
}

fn count_neighbors(
    grid: &HashMap<(usize, usize), char>,
    point: (usize, usize),
    fancy: bool,
) -> u32 {
    let mut sum = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            let mut point = (point.0, point.1);
            loop {
                point = (
                    (point.0 as isize + y) as usize,
                    (point.1 as isize + x) as usize,
                );
                match grid.get(&point) {
                    Some('#') => sum += 1,
                    Some('.') if fancy => continue,
                    _ => (),
                }
                break;
            }
        }
    }
    sum
}

fn day_eleven_b(input: &str) -> u64 {
    let mut grid: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(move |(column, c)| ((row, column), c))
        })
        .collect();

    let mut next_grid = HashMap::new();

    let mut changed = true;
    let mut occupied = 0;
    while changed {
        changed = false;
        for (&(row, column), c) in grid.iter() {
            let point = (row, column);
            let neighbors = count_neighbors(&grid, point, true);
            match c {
                'L' if neighbors == 0 => {
                    changed = true;
                    next_grid.insert(point, '#');
                    occupied += 1;
                }
                '#' if neighbors >= 5 => {
                    changed = true;
                    next_grid.insert(point, 'L');
                    occupied -= 1;
                }
                c => {
                    next_grid.insert(point, *c);
                }
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    occupied
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
