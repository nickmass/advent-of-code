use crate::HashMap;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> String {
    let mut lines = input.lines();

    let mut stacks = Stacks::new(&mut lines);

    for move_crate in lines.filter_map(|l| l.parse::<Move>().ok()) {
        stacks.perform_move(move_crate);
    }

    stacks.top_crates()
}

pub fn part_two(input: &str) -> String {
    let mut lines = input.lines();

    let mut stacks = Stacks::new(&mut lines);

    for move_crate in lines.filter_map(|l| l.parse::<Move>().ok()) {
        stacks.perform_upgraded_move(move_crate);
    }

    stacks.top_crates()
}

struct Stacks {
    stacks: HashMap<usize, VecDeque<char>>,
    names: HashMap<char, usize>,
}

impl Stacks {
    fn new<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Stacks {
        let mut stacks = HashMap::new();
        let mut names = HashMap::new();
        for line in lines {
            if line.trim().is_empty() {
                break;
            }

            for (idx, c) in line
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_digit() || c.is_ascii_alphabetic())
            {
                if c.is_ascii_digit() {
                    let position = (idx - 1) / 4;
                    names.insert(c, position);
                } else {
                    let position = (idx - 1) / 4;
                    let stack = stacks.entry(position).or_insert(VecDeque::new());
                    stack.push_front(c);
                }
            }
        }

        Self { stacks, names }
    }

    fn perform_move(&mut self, move_crate: Move) {
        let Some((src, dst)) = self
            .names
            .get(&move_crate.source)
            .zip(self.names.get(&move_crate.destination))
        else {
            return;
        };

        for _ in 0..move_crate.count {
            let piece = self.stacks.get_mut(src).unwrap().pop_back().unwrap();

            self.stacks.get_mut(dst).unwrap().push_back(piece);
        }
    }

    fn perform_upgraded_move(&mut self, move_crate: Move) {
        let Some((src, dst)) = self
            .names
            .get(&move_crate.source)
            .zip(self.names.get(&move_crate.destination))
        else {
            return;
        };

        let mut tmp = VecDeque::with_capacity(move_crate.count);

        for _ in 0..move_crate.count {
            let piece = self.stacks.get_mut(src).unwrap().pop_back().unwrap();

            tmp.push_front(piece);
        }

        for piece in tmp {
            self.stacks.get_mut(dst).unwrap().push_back(piece);
        }
    }

    fn top_crates(mut self) -> String {
        let mut result = String::new();
        let mut names: Vec<_> = self.names.into_iter().collect();
        names.sort_by_key(|(k, _)| *k);

        for (_, v) in names {
            if let Some(top) = self.stacks.get_mut(&v).unwrap().pop_back() {
                result.push(top);
            }
        }

        result
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    source: char,
    destination: char,
    count: usize,
}

struct MoveParseErr;

impl std::str::FromStr for Move {
    type Err = MoveParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: [_; 3] = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| MoveParseErr)?;

        let count = parts[0].parse::<usize>().map_err(|_| MoveParseErr)?;
        let source = parts[1].chars().next().ok_or(MoveParseErr)?;
        let destination = parts[2].chars().next().ok_or(MoveParseErr)?;

        Ok(Move {
            count,
            source,
            destination,
        })
    }
}

#[test]
fn test() {
    let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    assert_eq!("CMZ".to_string(), part_one(input));
    assert_eq!("MCD", part_two(input));
}
