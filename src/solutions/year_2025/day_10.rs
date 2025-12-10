use std::{collections::VecDeque, hash::Hash};

use crate::HashSet;

pub fn part_one(input: &str) -> u32 {
    let mut machine = Machine::new();
    let mut search_stack = BfsStack::new();
    let mut sum = 0;

    for l in input.trim().lines() {
        machine = machine.parse(l).expect("parse machine");
        sum += machine.part_one(&mut search_stack);
    }

    sum
}

pub fn part_two(_input: &str) -> &'static str {
    "incomplete"
}

struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
    voltages: Voltages,
}

impl Machine {
    fn new() -> Self {
        Machine {
            lights: Lights::new(),
            buttons: Vec::new(),
            voltages: Voltages::new(),
        }
    }

    fn parse(mut self, input: &str) -> Option<Self> {
        let mut input = input.as_bytes();
        self.lights = Lights::parse(&mut input)?;
        self.buttons.clear();
        while let Some(button) = Button::parse(&mut input) {
            self.buttons.push(button);
        }
        self.voltages = Voltages::parse(input)?;

        Some(self)
    }

    fn part_one(&self, stack: &mut BfsStack<Lights>) -> u32 {
        stack.reset();

        let lights = Lights::new();
        if self.lights == lights {
            return 0;
        }

        stack.push((lights, 0));
        while let Some((lights, count)) = stack.pop() {
            let count = count + 1;
            for b in self.buttons.iter() {
                let lights = lights.toggle(b);
                if self.lights == lights {
                    return count;
                } else {
                    stack.push((lights, count));
                }
            }
        }

        panic!("no solution found");
    }
}

struct BfsStack<T> {
    stack: VecDeque<(T, u32)>,
    visited: HashSet<T>,
}

impl<T: Eq + Hash + Clone> BfsStack<T> {
    fn new() -> Self {
        BfsStack {
            stack: VecDeque::new(),
            visited: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.stack.clear();
        self.visited.clear();
    }

    fn insert(&mut self, item: T) -> bool {
        self.visited.insert(item)
    }

    fn push(&mut self, (item, count): (T, u32)) {
        if self.insert(item.clone()) {
            self.stack.push_back((item, count));
        }
    }

    fn pop(&mut self) -> Option<(T, u32)> {
        self.stack.pop_front()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Lights {
    state: u32,
}

impl Lights {
    fn new() -> Self {
        Self { state: 0 }
    }

    fn parse(line: &mut &[u8]) -> Option<Self> {
        let mut rev_state = 0;
        let mut len = 0;

        loop {
            match line.split_off_first()? {
                b'[' | b']' => (),
                b'.' => {
                    rev_state <<= 1;
                    len += 1;
                }
                b'#' => {
                    rev_state <<= 1;
                    rev_state |= 1;
                    len += 1;
                }
                b' ' => break,
                _ => return None,
            }
        }

        let mut state = 0;
        for _ in 0..len {
            let n = rev_state & 1;
            state <<= 1;
            state |= n;
            rev_state >>= 1;
        }

        Some(Lights { state })
    }

    fn toggle(&self, button: &Button) -> Self {
        Lights {
            state: self.state ^ button.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Button(u32);

impl Button {
    fn parse(line: &mut &[u8]) -> Option<Self> {
        let mut value = 0;

        loop {
            match line.split_off_first()? {
                b'(' | b',' | b')' => (),
                n @ b'0'..=b'9' => {
                    let n = n - b'0';
                    value |= 1 << n;
                }
                b' ' => break,
                _ => {
                    return None;
                }
            }
        }

        Some(Button(value))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Voltages {
    items: [u32; 10],
    len: u8,
}

impl Voltages {
    fn new() -> Self {
        Self {
            items: [0; 10],
            len: 0,
        }
    }

    fn parse(line: &[u8]) -> Option<Self> {
        let mut items = [0; 10];
        let mut idx = 0;
        let mut value = 0;

        for &b in line {
            match b {
                b' ' | b'{' => (),
                b'}' => {
                    items[idx] = value;
                    idx += 1;
                    break;
                }
                b'0'..=b'9' => {
                    let n = (b - b'0') as u32;
                    value *= 10;
                    value += n;
                }
                b',' => {
                    items[idx] = value;
                    idx += 1;
                    value = 0;
                }
                _ => return None,
            }
        }

        Some(Self {
            items,
            len: idx as u8,
        })
    }
}

#[test]
fn test() {
    let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    assert_eq!(7, part_one(input));
    //assert_eq!(33, part_two(input));
}
