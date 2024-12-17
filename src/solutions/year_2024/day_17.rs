pub fn part_one(input: &str) -> String {
    Program::new(input).part_one()
}

pub fn part_two(input: &str) -> u64 {
    Program::new(input).part_two()
}

struct Program {
    init_a: u64,
    init_b: u64,
    init_c: u64,

    mask_a: u64,
    bits_a: u64,
    inc_a: u64,
    max_out: usize,

    a: u64,
    b: u64,
    c: u64,

    pc: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Program {
    fn new(input: &str) -> Self {
        let mut program = Vec::new();
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("Register A: ") {
                let (_, a_s) = line.split_once(": ").unwrap();
                a = a_s.parse().unwrap();
            } else if line.starts_with("Register B: ") {
                let (_, b_s) = line.split_once(": ").unwrap();
                b = b_s.parse().unwrap();
            } else if line.starts_with("Register C: ") {
                let (_, c_s) = line.split_once(": ").unwrap();
                c = c_s.parse().unwrap();
            } else if line.starts_with("Program: ") {
                let ops = line
                    .get("Program: ".len()..)
                    .into_iter()
                    .flat_map(|p| p.split(","))
                    .map(|n| n.parse::<u8>().unwrap());
                program.extend(ops);
            }
        }

        Self {
            a,
            b,
            c,

            init_a: a,
            init_b: b,
            init_c: c,

            inc_a: 0,
            mask_a: 0,
            bits_a: 0,
            max_out: 4,

            pc: 0,
            program,
            output: Vec::new(),
        }
    }

    fn run(&mut self, part_two: bool) {
        loop {
            let Some((opcode, operand)) = self.read().zip(self.read()) else {
                if part_two {
                    if self.output.len() == self.program.len() {
                        break;
                    } else {
                        self.next_a();
                        continue;
                    }
                } else {
                    break;
                }
            };
            let combo = self.combo(operand);
            let literal = operand as u64;

            match opcode & 7 {
                0 => self.a = self.a >> combo,
                1 => self.b ^= literal,
                2 => self.b = combo & 7,
                3 => {
                    if self.a != 0 {
                        self.pc = literal as usize;
                    }
                }
                4 => self.b ^= self.c,
                5 if part_two => {
                    let v = (combo & 7) as u8;
                    if self.program.get(self.output.len()).copied() == Some(v) {
                        self.output.push(v);
                    } else {
                        self.next_a();
                    }
                }
                5 => self.output.push((combo & 7) as u8),
                6 => self.b = self.a >> combo,
                7 => self.c = self.a >> combo,
                _ => unreachable!(),
            }
        }
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

    fn read(&mut self) -> Option<u8> {
        if self.pc >= self.program.len() {
            None
        } else {
            let v = self.program[self.pc];
            self.pc += 1;
            Some(v)
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.a = self.init_a;
        self.b = self.init_b;
        self.c = self.init_c;
        self.output.clear();
    }

    fn next_a(&mut self) {
        if self.output.len() > self.max_out + 1 {
            let v = self.inc_a << self.bits_a | self.mask_a;
            self.bits_a = self.output.len() as u64 * 3;
            let mask = (1 << self.bits_a) - 1;
            self.mask_a = v & mask;
            self.max_out = self.output.len();
        }

        self.reset();
        self.inc_a += 1;
        self.a = self.inc_a << self.bits_a | self.mask_a;
    }

    fn part_one(&mut self) -> String {
        self.run(false);

        let mut output: String = self
            .output
            .iter()
            .flat_map(|&n| [(n + b'0') as char, ','])
            .collect();
        output.pop();
        output
    }

    fn part_two(&mut self) -> u64 {
        self.a = 0;
        self.run(true);

        self.inc_a << self.bits_a | self.mask_a
    }
}

#[test]
fn test() {
    let mut program = Program::new("");
    program.c = 9;
    program.program.extend([2, 6]);
    program.run(false);
    assert_eq!(program.b, 1);

    let mut program = Program::new("");
    program.a = 10;
    program.program.extend([5, 0, 5, 1, 5, 4]);
    program.run(false);
    assert_eq!(program.part_one(), "0,1,2");

    let mut program = Program::new("");
    program.a = 2024;
    program.program.extend([0, 1, 5, 4, 3, 0]);
    program.run(false);
    assert_eq!(program.part_one(), "4,2,5,6,7,7,7,7,3,1,0");
    assert_eq!(program.a, 0);

    let mut program = Program::new("");
    program.b = 29;
    program.program.extend([1, 7]);
    program.run(false);
    assert_eq!(program.b, 26);

    let mut program = Program::new("");
    program.b = 2024;
    program.c = 43690;
    program.program.extend([4, 0]);
    program.run(false);
    assert_eq!(program.b, 44354);

    let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    assert_eq!("4,6,3,5,6,3,5,2,1,0", part_one(input));

    let input = r#"Register A: 2024
Register B: 0
Register C: 0
 
Program: 0,3,5,4,3,0
"#;
    assert_eq!(117440, part_two(input));
}
