pub fn part_one(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|l| ConditionRecord::unfold(l, 1).arrangements())
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    // 1,000,000x too slow, WORK IN PROGRESS
    // let folds = 5;
    let folds = 1;
    input
        .trim()
        .lines()
        .map(|l| ConditionRecord::unfold(l, folds).arrangements())
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
        //println!("-> {} ", display(&self.record));
        let mut searcher = Vec::with_capacity(self.record.len());
        let mut count = 0;

        let mut stack = Vec::new();
        let mut bad_count = 0;
        let mut pattern_idx = 0;

        loop {
            if let Some(next) = self.record.get(searcher.len()) {
                match next {
                    SpringState::Operational => {
                        let mut invalid = false;
                        if bad_count > 0 {
                            if let Some(&pattern) = self.pattern.get(pattern_idx) {
                                invalid |= pattern != bad_count;
                            }
                            pattern_idx += 1;
                            if !invalid {}
                        }
                        searcher.push(*next);
                        bad_count = 0;

                        if !invalid {
                            continue;
                        }
                    }
                    SpringState::Damaged => {
                        bad_count += 1;
                        searcher.push(*next);
                        continue;
                    }
                    SpringState::Unknown => {
                        for (option, bad_count, pattern_idx) in
                            self.options(&searcher, bad_count, pattern_idx)
                        {
                            stack.push((option, searcher.len(), bad_count, pattern_idx));
                        }
                    }
                }
            } else if self.validate(&searcher, bad_count, pattern_idx) {
                count += 1;
            }

            if let Some((opt, len, new_bad_count, new_pattern_idx)) = stack.pop() {
                searcher.truncate(len);
                searcher.push(opt);
                bad_count = new_bad_count;
                pattern_idx = new_pattern_idx;
            } else {
                break;
            }
        }

        count
    }

    fn options(
        &self,
        prefix: &[SpringState],
        bad_count: u32,
        pattern_idx: usize,
    ) -> impl Iterator<Item = (SpringState, u32, usize)> + '_ {
        let mut allow_bad = true;
        let mut allow_good = true;

        if let Some(&pattern) = self.pattern.get(pattern_idx) {
            if bad_count == pattern {
                allow_bad = false;
            } else if bad_count > 0 && bad_count < pattern {
                allow_good = false;
            } else if bad_count > pattern {
                allow_bad = false;
                allow_good = false;
            }
        } else if bad_count > 0 {
            allow_bad = false;
            allow_good = false;
        } else {
            allow_bad = false;
        }

        let new_idx = if bad_count > 0 {
            pattern_idx + 1
        } else {
            pattern_idx
        };

        if new_idx < self.req_pattern_idx[prefix.len()] {
            allow_good = false;
            allow_bad = false;
        } else if pattern_idx < self.req_pattern_idx[prefix.len()] {
            allow_bad = false;
        }

        [
            (SpringState::Damaged, bad_count + 1, pattern_idx),
            (SpringState::Operational, 0, new_idx),
        ]
        .into_iter()
        .filter(move |(s, _, _)| match s {
            SpringState::Operational if allow_good => true,
            SpringState::Damaged if allow_bad => true,
            SpringState::Unknown => unreachable!(),
            _ => false,
        })
    }

    fn validate(&self, _record: &[SpringState], bad_count: u32, mut pattern_idx: usize) -> bool {
        if bad_count > 0 {
            if let Some(&pattern) = self.pattern.get(pattern_idx) {
                if bad_count != pattern {
                    return false;
                }
            } else {
                return false;
            }
            pattern_idx += 1;
        }

        let result = pattern_idx == self.pattern.len();

        result
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
#[ignore = "part 2 incomplete"]
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
