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

pub fn part_two(input: &str) -> usize {
    let lines = input.trim().lines();

    let mut sum = 0;
    let mut buffer = Vec::with_capacity(10);
    for line in lines {
        if let Some((sample, result)) = line.split_once(" | ") {
            buffer.clear();
            buffer.extend(sample.split(' '));
            buffer.sort_by_key(|s| s.len());

            let solver = Solver::<Unsolved>::new(&buffer);
            let n = solver.solve().result(result);

            sum += n;
        }
    }

    sum
}

trait Solved {}

struct Solver<'s, T: Solved> {
    samples: &'s [&'s str],
    solved: T,
}

struct Unsolved;

struct OneFourSevenEightNine<'s> {
    num_1: &'s str,
    num_4: &'s str,
    num_7: &'s str,
    num_8: &'s str,
    num_9: &'s str,

    segment_4: char,
}

struct Two<'s> {
    num_1: &'s str,
    num_2: &'s str,
    num_4: &'s str,
    num_7: &'s str,
    num_8: &'s str,
    num_9: &'s str,

    segment_2: char,
    segment_4: char,
}

struct Six<'s> {
    num_1: &'s str,
    num_2: &'s str,
    num_4: &'s str,
    num_6: &'s str,
    num_7: &'s str,
    num_8: &'s str,
    num_9: &'s str,

    segment_2: char,
    segment_4: char,
}

struct Complete<'s> {
    nums: [&'s str; 10],
}

impl Solved for Unsolved {}
impl<'a> Solved for OneFourSevenEightNine<'a> {}
impl<'a> Solved for Two<'a> {}
impl<'a> Solved for Six<'a> {}
impl<'a> Solved for Complete<'a> {}

impl<'s, T: Solved> Solver<'s, T> {
    fn new(samples: &'s [&'s str]) -> Solver<'s, Unsolved> {
        Solver {
            samples,
            solved: Unsolved,
        }
    }
}

impl<'s> Solver<'s, Unsolved> {
    fn solve(self) -> Solver<'s, Complete<'s>> {
        let samples = self.samples;

        let num_1 = samples[0];
        let num_7 = samples[1];
        let num_4 = samples[2];
        let num_8 = samples[9];

        for s in samples.get(6..9).unwrap() {
            if diff_num_segments(num_4, s).is_none() {
                let num_9 = s;
                let segment_4 =
                    diff_num_segments(num_8, num_9).expect("8 and 9 should differ by segment 4");

                let solved = OneFourSevenEightNine {
                    num_1,
                    num_4,
                    num_7,
                    num_8,
                    num_9,

                    segment_4,
                };

                return Solver { samples, solved }.solve().solve().solve();
            }
        }

        unreachable!("9 should of been found")
    }
}

impl<'s> Solver<'s, OneFourSevenEightNine<'s>> {
    fn solve(self) -> Solver<'s, Two<'s>> {
        let samples = self.samples;
        let OneFourSevenEightNine {
            num_1,
            num_4,
            num_7,
            num_8,
            num_9,

            segment_4,
            ..
        } = self.solved;

        for s in samples.get(3..6).unwrap() {
            if s.contains(segment_4) {
                let num_2 = s;
                let segment_5 =
                    diff_num_segments(num_1, num_2).expect("1 and 2 have a segment differing");
                let segment_2 = num_1
                    .chars()
                    .find(|c| *c != segment_5)
                    .expect("1 without segment 5 leavees segment 2");

                let solved = Two {
                    num_1,
                    num_2,
                    num_4,
                    num_7,
                    num_8,
                    num_9,

                    segment_2,
                    segment_4,
                };

                return Solver { samples, solved };
            }
        }

        unreachable!("2 should of been found")
    }
}

impl<'s> Solver<'s, Two<'s>> {
    fn solve(self) -> Solver<'s, Six<'s>> {
        let samples = self.samples;
        let Two {
            num_1,
            num_2,
            num_4,
            num_7,
            num_8,
            num_9,

            segment_2,
            segment_4,
            ..
        } = self.solved;

        for s in samples.get(6..9).unwrap() {
            if !s.contains(segment_2) {
                let num_6 = s;

                let solved = Six {
                    num_1,
                    num_2,
                    num_4,
                    num_6,
                    num_7,
                    num_8,
                    num_9,

                    segment_2,
                    segment_4,
                };

                return Solver { samples, solved };
            }
        }

        unreachable!("6 should of been found")
    }
}

impl<'s> Solver<'s, Six<'s>> {
    fn solve(self) -> Solver<'s, Complete<'s>> {
        let samples = self.samples;
        let Six {
            num_1,
            num_2,
            num_4,
            num_6,
            num_7,
            num_8,
            num_9,

            segment_2,
            segment_4,
            ..
        } = self.solved;

        let mut num_0 = None;
        let mut num_3 = None;
        let mut num_5 = None;

        for s in samples.get(3..9).unwrap() {
            match s.len() {
                6 if *s != num_6 && *s != num_9 => num_0 = Some(s),
                5 if s.contains(segment_2) && !s.contains(segment_4) => num_3 = Some(s),
                5 if !s.contains(segment_2) => num_5 = Some(s),
                _ => (),
            }
        }

        let ((num_0, num_3), num_5) = num_0
            .zip(num_3)
            .zip(num_5)
            .expect("0, 3, and 5 should of been found");

        let nums = [
            num_0, num_1, num_2, num_3, num_4, num_5, num_6, num_7, num_8, num_9,
        ];

        let solved = Complete { nums };

        return Solver { samples, solved };
    }
}

impl<'s> Solver<'s, Complete<'s>> {
    fn result(&self, result: &str) -> usize {
        let mut number = 0;

        for n in result.split(' ') {
            for (idx, num) in self.solved.nums.iter().enumerate() {
                if is_unordered_match(n, num) {
                    number *= 10;
                    number += idx;
                    break;
                }
            }
        }

        number
    }
}

fn diff_num_segments(left: &str, right: &str) -> Option<char> {
    for c in left.chars() {
        if !right.contains(c) {
            return Some(c);
        }
    }
    None
}

fn is_unordered_match(left: &str, right: &str) -> bool {
    if left.len() == right.len() {
        left.chars().all(|c| right.contains(c))
    } else {
        false
    }
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
