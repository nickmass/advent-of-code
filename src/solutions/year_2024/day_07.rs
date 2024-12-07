pub fn part_one(input: &str) -> u64 {
    solve(Evaluator::part_one(), input)
}

pub fn part_two(input: &str) -> u64 {
    solve(Evaluator::part_two(), input)
}

fn solve(mut eval: Evaluator, input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut all_nums = Vec::new();
    let mut valid = 0;
    for line in lines {
        let Some((test, nums)) = line.split_once(": ") else {
            continue;
        };

        let Some(test) = test.parse::<u64>().ok() else {
            continue;
        };

        all_nums.clear();
        all_nums.extend(nums.split(" ").filter_map(|n| n.parse::<u64>().ok()));

        if eval.evaluate(test, &all_nums) {
            valid += test;
        }
    }

    valid
}

struct Evaluator {
    ops: &'static [Operation],
    search: Vec<(usize, u64)>,
}

impl Evaluator {
    fn part_one() -> Self {
        Self {
            ops: &[Operation::Add, Operation::Mul],
            search: Vec::new(),
        }
    }

    fn part_two() -> Self {
        Self {
            ops: &[Operation::Add, Operation::Mul, Operation::Concat],
            search: Vec::new(),
        }
    }

    fn evaluate(&mut self, test: u64, nums: &[u64]) -> bool {
        self.search.clear();
        self.search.push((0, 0));

        while let Some((idx, n)) = self.search.pop() {
            let val = nums[idx];

            let idx = idx + 1;

            for op in self.ops {
                let new = op.apply(n, val);

                if new == test {
                    return true;
                }

                if idx < nums.len() && new < test {
                    self.search.push((idx, new));
                }
            }
        }

        false
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn apply(self, n: u64, val: u64) -> u64 {
        match self {
            Operation::Add => n + val,
            Operation::Mul => n * val,
            Operation::Concat => {
                let c = if val == 0 { 1 } else { val.ilog10() + 1 };
                (n * 10u64.pow(c)) + val
            }
        }
    }
}

#[test]
fn test() {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    assert_eq!(3749, part_one(input));
    assert_eq!(11387, part_two(input));
}
