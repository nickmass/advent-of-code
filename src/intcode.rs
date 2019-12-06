use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Immediate(i32),
    Address(i32),
}

#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    Input,
    Output(i32),
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
    Halt,
}

pub struct Machine {
    static_mem: Box<[i32]>,
    mem: Box<[Option<i32>]>,
    program_counter: usize,
    input: Option<i32>,
    output: Option<i32>,
    debug: bool,
    int_halt: bool,
    int_input: bool,
    int_output: bool,
    instruction: Instruction,
}

impl Machine {
    pub fn new<S: AsRef<str>>(program: S) -> Self {
        let program = program.as_ref();
        let mem: Vec<_> = program
            .split(',')
            .map(str::trim)
            .map(i32::from_str)
            .filter_map(Result::ok)
            .collect();

        let working_mem = vec![None; mem.len()];

        Machine {
            static_mem: mem.into_boxed_slice(),
            mem: working_mem.into_boxed_slice(),
            program_counter: 0,
            input: None,
            output: None,
            debug: false,
            int_halt: false,
            int_input: false,
            int_output: false,
            instruction: Instruction::Halt,
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

    fn decode_operand(&mut self, immediate_mode: bool) -> Operand {
        if immediate_mode {
            Operand::Immediate(self.read_pc())
        } else {
            Operand::Address(self.read_pc())
        }
    }

    fn decode(&mut self) -> Instruction {
        let instruction = self.read_pc();
        let op = instruction % 100;
        let param_one_mode = (instruction / 100 % 10) != 0;
        let param_two_mode = (instruction / 1000 % 10) != 0;
        let param_three_mode = (instruction / 10000 % 10) != 0;

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
            99 => {
                self.int_halt = true;
                Instruction::Halt
            }
            _ => unreachable!(),
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
            unreachable!();
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

    pub fn poke(&mut self, addr: usize, value: i32) {
        self.mem[addr] = Some(value);
    }

    pub fn peek(&self, addr: usize) -> i32 {
        self.mem[addr].unwrap_or_else(|| self.static_mem[addr])
    }

    pub fn set_input(&mut self, value: i32) {
        if self.debug {
            println!("Input: {}", value);
        }
        self.input = Some(value);
    }

    fn read(&self, operand: Operand) -> i32 {
        match operand {
            Operand::Immediate(val) => val,
            Operand::Address(addr) => self.peek(addr as usize),
        }
    }

    fn write(&mut self, operand: Operand, value: i32) {
        match operand {
            Operand::Immediate(_) => (),
            Operand::Address(addr) => self.poke(addr as usize, value),
        }
    }

    fn read_pc(&mut self) -> i32 {
        let result = self.peek(self.program_counter);
        self.program_counter += 1;

        result
    }
}
