use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Immediate(i128),
    Address(i128),
    Relative(i128),
}

#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    Input,
    Output(i128),
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Add(Operand, Operand, Operand),
    Mul(Operand, Operand, Operand),
    In(Operand),
    Out(Operand),
    JmpTrue(Operand, Operand),
    JmpFalse(Operand, Operand),
    LessThan(Operand, Operand, Operand),
    Equal(Operand, Operand, Operand),
    AdjustRelativeBase(Operand),
    Halt,
}

pub struct Machine {
    static_mem: Vec<i128>,
    mem: Vec<Option<i128>>,
    program_counter: usize,
    input: Option<i128>,
    output: Option<i128>,
    debug: bool,
    int_halt: bool,
    int_input: bool,
    int_output: bool,
    instruction: Instruction,
    relative_base: i128,
}

impl Machine {
    pub fn new<S: AsRef<str>>(program: S) -> Self {
        let program = program.as_ref();
        let mem: Vec<_> = program
            .split(',')
            .map(str::trim)
            .map(i128::from_str)
            .filter_map(Result::ok)
            .collect();

        let working_mem = vec![None; mem.len()];

        Machine {
            static_mem: mem,
            mem: working_mem,
            program_counter: 0,
            input: None,
            output: None,
            debug: false,
            int_halt: false,
            int_input: false,
            int_output: false,
            instruction: Instruction::Halt,
            relative_base: 0,
        }
    }

    #[allow(dead_code)]
    pub fn debug<S: AsRef<str>>(program: S) -> Self {
        let mut machine = Self::new(program);
        machine.debug = true;
        machine
    }

    pub fn reset(&mut self) {
        for i in 0..self.mem.len() {
            self.mem[i] = None;
        }

        self.program_counter = 0;
        self.int_halt = false;
        self.int_input = false;
        self.int_output = false;
        self.input = None;
        self.output = None;

        if self.debug {
            println!("Reset");
        }
    }

    fn decode_operand(&mut self, address_mode: i128) -> Operand {
        match address_mode {
            0 => Operand::Address(self.read_pc()),
            1 => Operand::Immediate(self.read_pc()),
            2 => Operand::Relative(self.read_pc()),
            _ => unreachable!("Unknown address mode: {}", address_mode),
        }
    }

    fn decode(&mut self) -> Instruction {
        let instruction = self.read_pc();
        let op = instruction % 100;
        let param_one_mode = instruction / 100 % 10;
        let param_two_mode = instruction / 1000 % 10;
        let param_three_mode = instruction / 10000 % 10;

        match op {
            1 => {
                let op_one = self.decode_operand(param_one_mode);
                let op_two = self.decode_operand(param_two_mode);
                let op_three = self.decode_operand(param_three_mode);
                Instruction::Add(op_one, op_two, op_three)
            }
            2 => {
                let op_one = self.decode_operand(param_one_mode);
                let op_two = self.decode_operand(param_two_mode);
                let op_three = self.decode_operand(param_three_mode);
                Instruction::Mul(op_one, op_two, op_three)
            }
            3 => {
                self.int_input = true;
                let op_one = self.decode_operand(param_one_mode);
                Instruction::In(op_one)
            }
            4 => {
                self.int_output = true;
                let op_one = self.decode_operand(param_one_mode);
                Instruction::Out(op_one)
            }
            5 => {
                let op_one = self.decode_operand(param_one_mode);
                let op_two = self.decode_operand(param_two_mode);
                Instruction::JmpTrue(op_one, op_two)
            }
            6 => {
                let op_one = self.decode_operand(param_one_mode);
                let op_two = self.decode_operand(param_two_mode);
                Instruction::JmpFalse(op_one, op_two)
            }
            7 => {
                let op_one = self.decode_operand(param_one_mode);
                let op_two = self.decode_operand(param_two_mode);
                let op_three = self.decode_operand(param_three_mode);
                Instruction::LessThan(op_one, op_two, op_three)
            }
            8 => {
                let op_one = self.decode_operand(param_one_mode);
                let op_two = self.decode_operand(param_two_mode);
                let op_three = self.decode_operand(param_three_mode);
                Instruction::Equal(op_one, op_two, op_three)
            }
            9 => {
                let op_one = self.decode_operand(param_one_mode);
                Instruction::AdjustRelativeBase(op_one)
            }
            99 => {
                self.int_halt = true;
                Instruction::Halt
            }
            _ => unreachable!("Unknown opcode: {}", op),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(a, b, c) => {
                let left = self.read(a);
                let right = self.read(b);
                self.write(c, left + right)
            }
            Instruction::Mul(a, b, c) => {
                let left = self.read(a);
                let right = self.read(b);
                self.write(c, left * right)
            }
            Instruction::In(a) => {
                if let Some(input) = self.input.take() {
                    self.int_input = false;
                    self.write(a, input);
                }
            }
            Instruction::Out(a) => {
                self.output = Some(self.read(a));
            }
            Instruction::JmpTrue(a, b) => {
                let condition = self.read(a);
                let new_pc = self.read(b);

                if condition != 0 {
                    self.program_counter = new_pc as usize;
                }
            }
            Instruction::JmpFalse(a, b) => {
                let condition = self.read(a);
                let new_pc = self.read(b);

                if condition == 0 {
                    self.program_counter = new_pc as usize;
                }
            }
            Instruction::LessThan(a, b, c) => {
                let left = self.read(a);
                let right = self.read(b);
                let value = if left < right { 1 } else { 0 };
                self.write(c, value);
            }
            Instruction::Equal(a, b, c) => {
                let left = self.read(a);
                let right = self.read(b);
                let value = if left == right { 1 } else { 0 };
                self.write(c, value);
            }
            Instruction::AdjustRelativeBase(a) => {
                let val = self.read(a);
                self.relative_base += val;
            }
            Instruction::Halt => {
                if self.debug {
                    println!("Halt")
                }
            }
        }
    }

    pub fn run(&mut self) -> Interrupt {
        loop {
            self.tick();
            if self.pending_interrupt() {
                break;
            }
        }

        if self.int_halt {
            Interrupt::Halt
        } else if self.int_input {
            Interrupt::Input
        } else if self.int_output {
            self.int_output = false;
            if self.debug {
                println!("Output: {:?}", self.output);
            }
            Interrupt::Output(self.output.take().unwrap_or(0))
        } else {
            unreachable!("Unmatched interrupt");
        }
    }

    fn pending_interrupt(&self) -> bool {
        self.int_halt || self.int_input || self.int_output
    }

    fn tick(&mut self) {
        let pc = self.program_counter;
        let op = self.peek(pc);
        if !self.pending_interrupt() {
            self.instruction = self.decode();
        }
        if self.debug {
            println!("{}:{}: {:?}", pc, op, self.instruction);
        }
        self.execute(self.instruction);
    }

    pub fn resize_to_fit(&mut self, addr: usize) {
        if addr >= self.mem.len() {
            if self.debug {
                println!("Resized mem from {} to {}", self.mem.len(), addr + 1);
            }
            self.mem.resize(addr + 1, Some(0));
            self.static_mem.resize(addr + 1, 0);
        }
    }

    pub fn poke(&mut self, addr: usize, value: i128) {
        self.resize_to_fit(addr);
        self.mem[addr] = Some(value);
    }

    pub fn peek(&mut self, addr: usize) -> i128 {
        self.resize_to_fit(addr);
        self.mem[addr].unwrap_or_else(|| self.static_mem[addr])
    }

    pub fn set_input(&mut self, value: i128) {
        if self.debug {
            println!("Input: {}", value);
        }
        self.input = Some(value);
    }

    fn read(&mut self, operand: Operand) -> i128 {
        match operand {
            Operand::Immediate(val) => val,
            Operand::Address(addr) => self.peek(addr as usize),
            Operand::Relative(offset) => self.peek((self.relative_base + offset) as usize),
        }
    }

    fn write(&mut self, operand: Operand, value: i128) {
        match operand {
            Operand::Immediate(_) => (),
            Operand::Address(addr) => self.poke(addr as usize, value),
            Operand::Relative(offset) => self.poke((self.relative_base + offset) as usize, value),
        }
    }

    fn read_pc(&mut self) -> i128 {
        let result = self.peek(self.program_counter);
        self.program_counter += 1;

        result
    }
}
