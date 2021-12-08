pub fn part_one(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut sum = 0;
    for line in lines {
        if let Some((_, result)) = line.split_once(" | ") {
            for num in result.split(' ') {
                match num.len() {
                    2 | 3 | 7 | 4 => sum += 1,
                    _ => (),
                }
            }
        }
    }

    sum
}

//  000
// 1   2
// 1   2
//  333
// 4   5
// 4   5
//  666

// 0   6
// 1   2
// 2   5
// 3   5
// 4   4
// 5   5
// 6   6
// 7   3
// 8   7
// 9   6

pub fn part_two(input: &str) -> usize {
    let lines = input.trim().lines();

    let mut sum = 0;
    for line in lines {
        if let Some((sample, result)) = line.split_once(" | ") {
            let sample = sample.split(' ');
            let result = result.split(' ');

            let mut nums: [Option<&str>; 10] = [None; 10];
            let mut segments: [Option<char>; 7] = [None; 7];

            let single_diff = |big: &str, small: &str| {
                for c in big.chars() {
                    if !small.contains(c) {
                        return Some(c);
                    }
                }
                None
            };

            for s in sample.clone() {
                match s.len() {
                    2 => nums[1] = Some(s),
                    4 => nums[4] = Some(s),
                    3 => nums[7] = Some(s),
                    7 => nums[8] = Some(s),
                    _ => (),
                }
            }

            let c = single_diff(nums[7].unwrap(), nums[1].unwrap()).unwrap();
            segments[0] = Some(c);

            for s in sample.clone() {
                match s.len() {
                    6 => {
                        if single_diff(nums[4].unwrap(), s).is_none() {
                            nums[9] = Some(s);
                            let c = single_diff(nums[8].unwrap(), nums[9].unwrap()).unwrap();
                            segments[4] = Some(c);
                        }
                    }
                    _ => (),
                }
            }

            for s in sample.clone() {
                match s.len() {
                    5 if s.contains(segments[4].unwrap()) => {
                        nums[2] = Some(s);
                        let c = single_diff(nums[1].unwrap(), nums[2].unwrap()).unwrap();
                        segments[5] = Some(c);
                        for d in nums[1].unwrap().chars() {
                            if d != c {
                                segments[2] = Some(d);
                            }
                        }
                    }
                    _ => (),
                }
            }

            for s in sample.clone() {
                match s.len() {
                    6 if !s.contains(segments[2].unwrap()) => {
                        nums[6] = Some(s);
                    }
                    _ => (),
                }
            }

            for s in sample {
                match s.len() {
                    6 if s != nums[6].unwrap() && s != nums[9].unwrap() => {
                        nums[0] = Some(s);
                    }
                    5 if s.contains(segments[2].unwrap()) && !s.contains(segments[4].unwrap()) => {
                        nums[3] = Some(s)
                    }
                    5 if !s.contains(segments[2].unwrap()) => nums[5] = Some(s),
                    _ => (),
                }
            }

            let is_match = |left: &str, right: &str| {
                if left.len() == right.len() {
                    left.chars().all(|c| right.contains(c))
                } else {
                    false
                }
            };

            let mut number = 0;
            for n in result {
                for (idx, num) in nums.iter().enumerate() {
                    let num = num.unwrap();
                    if is_match(n, num) {
                        number *= 10;
                        number += idx;
                    }
                }
            }

            sum += number;
        }
    }

    sum
}

#[test]
fn test() {
    let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    assert_eq!(26, part_one(input));
    assert_eq!(61229, part_two(input));
}
