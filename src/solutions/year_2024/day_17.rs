use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> String {
    let (mut machine, program) = parse(input).unwrap();

    let mut output = String::new();
    while let Interrupt::Output(b) = machine.run(&program) {
        let c = (b + b'0') as char;
        if output.len() > 0 {
            output.push(',');
        }
        output.push(c);
    }

    output
}

pub fn part_two(input: &str) -> u64 {
    let (machine, program) = parse(input).unwrap();

    let mut output = Vec::with_capacity(program.len());
    let mut heap = BinaryHeap::new();
    heap.push(Search(0, 0));

    while let Some(Search(base_a, valid)) = heap.pop() {
        'outer: for n in 0..0x400 {
            let mut machine = machine.clone();
            output.clear();

            let a = n << (valid * 3) | base_a;
            machine.a = a;

            while let Interrupt::Output(b) = machine.run(&program) {
                if program.get(output.len()) == Some(&b) {
                    output.push(b);

                    if output.len() > valid {
                        let valid = output.len();
                        let mask = (1 << (valid * 3)) - 1;
                        heap.push(Search(a & mask, valid));
                    }
                } else {
                    continue 'outer;
                }
            }

            if output.len() == program.len() {
                return a;
            }
        }
    }

    0
}

#[derive(Debug, Clone)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
}

impl Machine {
    fn run(&mut self, mem: &[u8]) -> Interrupt {
        while let Some((opcode, operand)) = self.read_instruction(mem) {
            let combo = self.combo(operand);

            match opcode & 7 {
                0 => self.a = self.a >> combo,
                1 => self.b ^= operand as u64,
                2 => self.b = combo & 7,
                3 => {
                    if self.a != 0 {
                        self.pc = operand as usize;
                    }
                }
                4 => self.b ^= self.c,
                5 => return Interrupt::Output((combo & 7) as u8),
                6 => self.b = self.a >> combo,
                7 => self.c = self.a >> combo,
                _ => unreachable!(),
            }
        }

        Interrupt::Halt
    }

    fn combo(&self, n: u8) -> u64 {
        match n & 7 {
            n @ 0..=3 => n as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0,
        }
    }

    fn read_instruction(&mut self, mem: &[u8]) -> Option<(u8, u8)> {
        self.read(mem).zip(self.read(mem))
    }

    fn read(&mut self, mem: &[u8]) -> Option<u8> {
        if self.pc >= mem.len() {
            None
        } else {
            let v = mem[self.pc];
            self.pc += 1;
            Some(v)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Interrupt {
    Halt,
    Output(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Search(u64, usize);

impl std::cmp::Ord for Search {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.1.cmp(&other.1) {
            std::cmp::Ordering::Equal => other.0.cmp(&self.0),
            c => c,
        }
    }
}

impl std::cmp::PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Option<(Machine, Vec<u8>)> {
    let mut lines = input.lines();
    let a = take_token("Register A: ", lines.next()?)?.parse().ok()?;
    let b = take_token("Register B: ", lines.next()?)?.parse().ok()?;
    let c = take_token("Register C: ", lines.next()?)?.parse().ok()?;
    let _ = lines.next()?;
    let program = take_token("Program: ", lines.next()?)?
        .as_bytes()
        .iter()
        .copied()
        .filter(|&b| b != b',')
        .map(|b| b - b'0')
        .collect();

    let machine = Machine { a, b, c, pc: 0 };

    Some((machine, program))
}

fn take_token<'a>(token: &str, s: &'a str) -> Option<&'a str> {
    if s.starts_with(token) {
        Some(&s[token.len()..])
    } else {
        None
    }
}

#[test]
fn test() {
    let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    assert_eq!("4,6,3,5,6,3,5,2,1,0", part_one(input));

    let input = r#"Register A: 2024
Register B: 0
Register C: 0
 
Program: 0,3,5,4,3,0"#;

    assert_eq!(117440, part_two(input));
}
