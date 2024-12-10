use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|l| ConditionRecord::unfold(l, 1).arrangements())
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|l| ConditionRecord::unfold(l, 5).arrangements())
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

struct ConditionRecord {
    record: Vec<SpringState>,
    pattern: Vec<u32>,
    req_pattern_idx: Vec<usize>,
}

impl ConditionRecord {
    fn unfold(line: &str, count: usize) -> Self {
        let (record_str, pattern_str) = line.trim().split_once(' ').unwrap();

        let mut record = Vec::new();
        for i in 0..count {
            if i != 0 {
                record.push(SpringState::Unknown);
            }
            record.extend(record_str.chars().map(|c| match c {
                '?' => SpringState::Unknown,
                '#' => SpringState::Damaged,
                '.' => SpringState::Operational,
                _ => unreachable!(),
            }));
        }

        let mut pattern = Vec::new();

        for _ in 0..count {
            pattern.extend(pattern_str.split(',').map(|n| n.parse::<u32>().unwrap()))
        }

        let mut req_pattern_idx = Vec::new();
        let mut pattern_iter = pattern.iter().copied().rev();
        let mut next_pattern = None;

        let mut next_pattern_idx = pattern.len().saturating_sub(1);
        let mut bad_count = 0;
        let mut need_good = false;

        for state in record.iter().rev() {
            let pattern = next_pattern.or_else(|| pattern_iter.next()).unwrap_or(0);
            next_pattern = Some(pattern);

            req_pattern_idx.push(next_pattern_idx);

            match state {
                SpringState::Operational | SpringState::Unknown if need_good => {
                    need_good = false;
                    next_pattern_idx = next_pattern_idx.saturating_sub(1);
                    bad_count = 0;
                    next_pattern = None;
                }
                SpringState::Operational => {
                    bad_count = 0;
                }
                SpringState::Damaged => {
                    bad_count += 1;
                }
                SpringState::Unknown => {
                    bad_count += 1;
                }
            }

            if bad_count == pattern {
                need_good = true;
            }
        }
        req_pattern_idx.reverse();

        Self {
            record,
            pattern,
            req_pattern_idx,
        }
    }

    fn arrangements(&self) -> u64 {
        let mut starts = vec![StackEntry::from_position(0, 0, 1)];
        let mut result = 0;
        for _ in 0..self.pattern.len() {
            let results = self.paths_to_next_pattern(starts);

            starts = Vec::with_capacity(results.len());
            result = 0;
            for (mut entry, count) in results {
                entry.multiplier = count;
                starts.push(entry);
                result += count;
            }
        }
        result
    }

    fn paths_to_next_pattern(&self, mut stack: Vec<StackEntry>) -> HashMap<StackEntry, u64> {
        let mut result = HashMap::new();

        let Some(mut entry) = stack.pop() else {
            return result;
        };

        let initial_pattern_idx = entry.pattern_idx;

        loop {
            if let Some(next) = self.record.get(entry.record_idx) {
                match next {
                    SpringState::Operational => {
                        let mut invalid = false;
                        if entry.bad_count > 0 {
                            if let Some(&pattern) = self.pattern.get(entry.pattern_idx) {
                                invalid |= pattern != entry.bad_count;
                            }
                            entry.pattern_idx += 1;
                        }
                        entry.record_idx += 1;
                        entry.bad_count = 0;
                        if !invalid {
                            continue;
                        }
                    }
                    SpringState::Damaged => {
                        entry.bad_count += 1;
                        entry.record_idx += 1;
                        continue;
                    }
                    SpringState::Unknown => {
                        for mut new_entry in self.options(&entry) {
                            if new_entry.pattern_idx > initial_pattern_idx
                                && new_entry.pattern_idx < self.pattern.len()
                            {
                                new_entry.multiplier = 0;
                                let result_entry = result.entry(new_entry).or_insert(0);
                                *result_entry += entry.multiplier;
                            } else {
                                stack.push(new_entry);
                            }
                        }
                    }
                }
            } else if self.validate(entry) {
                let count = entry.multiplier;
                entry.multiplier = 0;
                let result_entry = result.entry(entry).or_insert(0);
                *result_entry += count;
            }

            if let Some(next_entry) = stack.pop() {
                entry = next_entry;
            } else {
                break;
            }
        }

        result
    }

    fn options(&self, entry: &StackEntry) -> impl Iterator<Item = StackEntry> + '_ {
        let mut allow_bad = true;
        let mut allow_good = true;

        if let Some(&pattern) = self.pattern.get(entry.pattern_idx) {
            if entry.bad_count == pattern {
                allow_bad = false;
            } else if entry.bad_count > 0 && entry.bad_count < pattern {
                allow_good = false;
            } else if entry.bad_count > pattern {
                allow_bad = false;
                allow_good = false;
            }
        } else if entry.bad_count > 0 {
            allow_bad = false;
            allow_good = false;
        } else {
            allow_bad = false;
        }

        let new_idx = if entry.bad_count > 0 {
            entry.pattern_idx + 1
        } else {
            entry.pattern_idx
        };

        if new_idx < self.req_pattern_idx[entry.record_idx] {
            allow_good = false;
            allow_bad = false;
        } else if entry.pattern_idx < self.req_pattern_idx[entry.record_idx] {
            allow_bad = false;
        }

        let record_idx = entry.record_idx + 1;

        [
            StackEntry {
                record_idx,
                pattern_idx: entry.pattern_idx,
                state: SpringState::Damaged,
                bad_count: entry.bad_count + 1,
                multiplier: entry.multiplier,
            },
            StackEntry {
                record_idx,
                pattern_idx: new_idx,
                state: SpringState::Operational,
                bad_count: 0,
                multiplier: entry.multiplier,
            },
        ]
        .into_iter()
        .filter(move |e| match e.state {
            SpringState::Operational if allow_good => true,
            SpringState::Damaged if allow_bad => true,
            SpringState::Unknown => unreachable!(),
            _ => false,
        })
    }

    fn validate(&self, mut entry: StackEntry) -> bool {
        if entry.bad_count > 0 {
            if let Some(&pattern) = self.pattern.get(entry.pattern_idx) {
                if entry.bad_count != pattern {
                    return false;
                }
            } else {
                return false;
            }
            entry.pattern_idx += 1;
        }

        entry.pattern_idx == self.pattern.len()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct StackEntry {
    record_idx: usize,
    pattern_idx: usize,
    state: SpringState,
    bad_count: u32,
    multiplier: u64,
}

impl StackEntry {
    fn from_position(record_idx: usize, pattern_idx: usize, multiplier: u64) -> Self {
        StackEntry {
            record_idx,
            pattern_idx,
            state: SpringState::Operational,
            bad_count: 0,
            multiplier,
        }
    }
}

#[allow(dead_code)]
fn display(record: &[SpringState]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(record.len());

    for r in record.iter() {
        let c = match r {
            SpringState::Operational => '.',
            SpringState::Damaged => '#',
            SpringState::Unknown => '?',
        };

        let _ = write!(s, "{c}");
    }

    s
}

#[test]
fn test() {
    let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

    assert_eq!(21, part_one(input));
    assert_eq!(525152, part_two(input));
}
