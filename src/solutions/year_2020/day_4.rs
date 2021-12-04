use super::{take_char, take_u64};

use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
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

pub fn part_two(input: &str) -> u64 {
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
