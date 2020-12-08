use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

fn main() {
    let downloader = InputDownloader::new();

    let event = 2020;

    let solutions: Vec<(u32, ProblemFunc, ProblemFunc)> = vec![
        (1, problem_one_a, problem_one_b),
        (2, problem_two_a, problem_two_b),
        (3, problem_three_a, problem_three_b),
        (4, problem_four_a, problem_four_b),
        (5, problem_five_a, problem_five_b),
        (6, problem_six_a, problem_six_b),
        (7, problem_seven_a, problem_seven_b),
    ];

    for solution in solutions {
        run_day(&downloader, event, solution.0, solution.1, solution.2);
    }
}

type ProblemFunc = fn(&str) -> u64;

fn run_day(
    downloader: &InputDownloader,
    event: u32,
    day: u32,
    part_a: ProblemFunc,
    part_b: ProblemFunc,
) {
    match downloader.download_input_if_absent(event, day) {
        Ok(input) => {
            let time_a = std::time::Instant::now();
            let a = part_a(&input);
            let time_a = time_a.elapsed();

            let time_b = std::time::Instant::now();
            let b = part_b(&input);
            let time_b = time_b.elapsed();

            println!("Day {} a:{:>25}{:>10}ms", day, a, time_a.as_millis());
            println!("Day {} b:{:>25}{:>10}ms", day, b, time_b.as_millis());
        }
        Err(error) => {
            eprintln!(
                "unable to get input for '{}' day '{}'. {:?}",
                event, day, error
            );
            std::process::exit(1)
        }
    }
}

struct InputDownloader {
    session_key: Option<String>,
    http_client: reqwest::blocking::Client,
}

impl InputDownloader {
    fn new() -> Self {
        let session_key = std::fs::read_to_string("./.session-key").ok();
        let http_client = reqwest::blocking::Client::new();

        Self {
            session_key,
            http_client,
        }
    }

    fn download_input_if_absent(
        &self,
        event: u32,
        day: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from(format!("problems/{}/day{}.txt", event, day));
        if path.exists() {
            let input = std::fs::read_to_string(&path)?;
            Ok(input)
        } else {
            eprintln!("downloading {} day {}.", event, day);
            let session_key = self.session_key.as_ref().ok_or(".session-key not found")?;
            let url = format!("https://adventofcode.com/{}/day/{}/input", event, day);
            let req = self.http_client.get(&url);
            let req = req.header("cookie", format!("session={}", session_key));
            let res = req.send()?;
            let res = res.error_for_status()?;
            let input = res.text()?;
            std::fs::create_dir_all(&path.parent().expect("problem path should have parent"))?;
            std::fs::write(&path, &input)?;

            Ok(input)
        }
    }
}

fn problem_one_a(input: &str) -> u64 {
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

fn problem_one_b(input: &str) -> u64 {
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

fn problem_two_a(input: &str) -> u64 {
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

fn problem_two_b(input: &str) -> u64 {
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

fn problem_three_a(input: &str) -> u64 {
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

fn problem_three_b(input: &str) -> u64 {
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

fn problem_four_a(input: &str) -> u64 {
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

fn problem_four_b(input: &str) -> u64 {
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

fn problem_five_a(input: &str) -> u64 {
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

fn problem_five_b(input: &str) -> u64 {
    let lines = input.lines();

    let mut map = HashSet::default();
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

fn problem_six_a(input: &str) -> u64 {
    let questions: usize = input
        .split("\n\n")
        .map(|ls| {
            let mut set = HashSet::default();
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

fn problem_six_b(input: &str) -> u64 {
    let questions: usize = input
        .split("\n\n")
        .map(|ls| {
            let mut line_count = 0;
            let mut set = HashMap::default();
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

fn problem_seven_a(input: &str) -> u64 {
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

    let mut matches = HashSet::default();
    let mut no_match = HashSet::default();

    matches.insert("shiny gold");

    let mut list = Vec::with_capacity(bags.len());
    'outer: for (color, children) in &bags {
        if matches.contains(color.as_str()) {
            continue;
        }
        list.clear();
        list.extend(children.iter().map(|(child, _num)| child));

        while let Some(child_color) = list.pop() {
            if matches.contains(child_color.as_str()) {
                matches.insert(color);
                continue 'outer;
            } else if !no_match.contains(child_color) {
                let next_parent = bags.get(child_color).unwrap();
                for (child, _num) in next_parent {
                    list.push(child);
                }
            }
        }

        no_match.insert(color);
    }

    //subtract one for gold bag
    (matches.len() - 1) as u64
}

fn problem_seven_b(input: &str) -> u64 {
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
