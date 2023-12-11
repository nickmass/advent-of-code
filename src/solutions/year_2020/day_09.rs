use std::collections::VecDeque;

pub fn part_one(input: &str) -> u64 {
    solve_part_one::<25>(input)
}

fn solve_part_one<const N: usize>(input: &str) -> u64 {
    let mut lines = input.lines().filter_map(|p| p.parse::<u64>().ok());

    let mut preamble: VecDeque<u64> = lines.by_ref().take(N).collect();

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

pub fn part_two(input: &str) -> u64 {
    solve_part_two::<25>(input)
}
pub fn solve_part_two<const N: usize>(input: &str) -> u64 {
    let target = solve_part_one::<N>(input);
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

#[test]
fn test() {
    let input = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#;

    assert_eq!(127, solve_part_one::<5>(input));
    assert_eq!(62, solve_part_two::<5>(input));
}
