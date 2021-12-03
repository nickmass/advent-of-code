use crate::HashMap;

use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Operand<W> {
    Immediate(W),
    Address(W),
    Relative(W),
}

#[derive(Debug, Copy, Clone)]
pub enum Interrupt<W> {
    Input,
    Output(W),
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum Instruction<W> {
    Add(Operand<W>, Operand<W>, Operand<W>),
    Mul(Operand<W>, Operand<W>, Operand<W>),
    In(Operand<W>),
    Out(Operand<W>),
    JmpTrue(Operand<W>, Operand<W>),
    JmpFalse(Operand<W>, Operand<W>),
    LessThan(Operand<W>, Operand<W>, Operand<W>),
    Equal(Operand<W>, Operand<W>, Operand<W>),
    AdjustRelativeBase(Operand<W>),
    Halt,
}

pub struct Opcode {
    opcode: u32,
    param_one_mode: u32,
    param_two_mode: u32,
    param_three_mode: u32,
}

pub trait Word:
    FromStr
    + Clone
    + Copy
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::cmp::PartialEq
    + std::cmp::PartialOrd
    + std::fmt::Debug
    + std::fmt::Display
{
    const ZERO: Self;
    const ONE: Self;
    fn as_opcode(&self) -> Opcode;
    fn as_address(&self) -> usize;
}

macro_rules! impl_word {
    ($($type:ty )+) => {
        $(
        impl Word for $type {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            fn as_opcode(&self) -> Opcode {
                let val = *self as u32;
                let opcode = (val as u32) % 100;
                let param_one_mode = (val as u32) / 100 % 10;
                let param_two_mode = (val as u32) / 1000 % 10;
                let param_three_mode = (val as u32) / 10000 % 10;

                Opcode {
                    opcode,
                    param_one_mode,
                    param_two_mode,
                    param_three_mode,
                }
            }

            fn as_address(&self) -> usize {
                *self as usize
            }
        }
            )+
    };
}

impl_word!(i16 i32 i64 i128);

pub trait Memory<W: Word> {
    fn new(capacity: usize) -> Self;
    fn read(&mut self, addr: usize) -> Option<W>;
    fn write(&mut self, addr: usize, value: W);
    fn reset(&mut self);
}

pub struct VecMem<W: Word> {
    capacity: usize,
    mem: Vec<Option<W>>,
}

impl<W: Word> VecMem<W> {
    fn resize_to_fit(&mut self, address: usize) {
        if address >= self.mem.len() {
            self.mem.resize(address + 1, Some(W::ZERO));
        }
    }
}

impl<W: Word> Memory<W> for VecMem<W> {
    fn new(capacity: usize) -> Self {
        let mem = vec![None; capacity];
        VecMem { capacity, mem }
    }
    fn read(&mut self, addr: usize) -> Option<W> {
        self.resize_to_fit(addr);
        self.mem.get(addr).and_then(|v| v.clone())
    }
    fn write(&mut self, addr: usize, value: W) {
        self.resize_to_fit(addr);
        self.mem[addr] = Some(value);
    }
    fn reset(&mut self) {
        self.mem.resize(self.capacity, None);
        for i in 0..self.mem.len() {
            self.mem[i] = None;
        }
    }
}

pub struct MapMem<W: Word> {
    capacity: usize,
    mem: HashMap<usize, W>,
}

impl<W: Word> Memory<W> for MapMem<W> {
    fn new(capacity: usize) -> Self {
        let mem = HashMap::new();
        MapMem { capacity, mem }
    }
    fn read(&mut self, addr: usize) -> Option<W> {
        self.mem.get(&addr).map(|v| v.clone()).or_else(|| {
            if addr > self.capacity {
                Some(W::ZERO)
            } else {
                None
            }
        })
    }
    fn write(&mut self, addr: usize, value: W) {
        self.mem.insert(addr, value);
    }
    fn reset(&mut self) {
        self.mem.clear();
    }
}

pub struct Machine<W: Word, M: Memory<W>> {
    static_mem: Vec<W>,
    mem: M,
    program_counter: usize,
    input: Option<W>,
    output: Option<W>,
    debug: bool,
    int_halt: bool,
    int_input: bool,
    int_output: bool,
    instruction: Instruction<W>,
    relative_base: W,
}

impl<W: Word> Machine<W, VecMem<W>> {
    pub fn new<S: AsRef<str>>(program: S) -> Machine<W, VecMem<W>> {
        Machine::<W, VecMem<W>>::with_mem(program)
    }
}

impl<W: Word, M: Memory<W>> Machine<W, M> {
    pub fn with_mem<S: AsRef<str>>(program: S) -> Self {
        let program = program.as_ref();
        let mem: Vec<_> = program
            .split(',')
            .map(str::trim)
            .map(W::from_str)
            .filter_map(Result::ok)
            .collect();

        let working_mem = M::new(mem.len());

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
            relative_base: W::ZERO,
        }
    }

    #[allow(dead_code)]
    pub fn debug<S: AsRef<str>>(program: S) -> Self {
        let mut machine = Self::with_mem(program);
        machine.debug = true;
        machine
    }

    pub fn reset(&mut self) {
        self.mem.reset();

        self.program_counter = 0;
        self.int_halt = false;
        self.int_input = false;
        self.int_output = false;
        self.input = None;
        self.output = None;
        self.relative_base = W::ZERO;

        if self.debug {
            println!("Reset");
        }
    }

    fn decode_operand(&mut self, address_mode: u32) -> Operand<W> {
        match address_mode {
            0 => Operand::Address(self.read_pc()),
            1 => Operand::Immediate(self.read_pc()),
            2 => Operand::Relative(self.read_pc()),
            _ => unreachable!("Unknown address mode: {}", address_mode),
        }
    }

    fn decode(&mut self) -> Instruction<W> {
        let instruction = self.read_pc();

        let Opcode {
            opcode,
            param_one_mode,
            param_two_mode,
            param_three_mode,
        } = instruction.as_opcode();

        match opcode {
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
            _ => unreachable!("Unknown opcode: {}", opcode),
        }
    }

    fn execute(&mut self, instruction: Instruction<W>) {
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

                if condition != W::ZERO {
                    self.program_counter = new_pc.as_address();
                }
            }
            Instruction::JmpFalse(a, b) => {
                let condition = self.read(a);
                let new_pc = self.read(b);

                if condition == W::ZERO {
                    self.program_counter = new_pc.as_address();
                }
            }
            Instruction::LessThan(a, b, c) => {
                let left = self.read(a);
                let right = self.read(b);
                let value = if left < right { W::ONE } else { W::ZERO };
                self.write(c, value);
            }
            Instruction::Equal(a, b, c) => {
                let left = self.read(a);
                let right = self.read(b);
                let value = if left == right { W::ONE } else { W::ZERO };
                self.write(c, value);
            }
            Instruction::AdjustRelativeBase(a) => {
                let val = self.read(a);
                self.relative_base = self.relative_base + val;
            }
            Instruction::Halt => {
                if self.debug {
                    println!("Halt")
                }
            }
        }
    }

    pub fn run(&mut self) -> Interrupt<W> {
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
            Interrupt::Output(self.output.take().unwrap_or(W::ZERO))
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

    pub fn poke(&mut self, addr: usize, value: W) {
        self.mem.write(addr, value);
    }

    pub fn peek(&mut self, addr: usize) -> W {
        self.mem.read(addr).unwrap_or_else(|| self.static_mem[addr])
    }

    pub fn set_input(&mut self, value: W) {
        if self.debug {
            println!("Input: {}", value);
        }
        self.input = Some(value);
    }

    fn read(&mut self, operand: Operand<W>) -> W {
        match operand {
            Operand::Immediate(val) => val,
            Operand::Address(addr) => self.peek(addr.as_address()),
            Operand::Relative(offset) => self.peek((self.relative_base + offset).as_address()),
        }
    }

    fn write(&mut self, operand: Operand<W>, value: W) {
        match operand {
            Operand::Immediate(_) => (),
            Operand::Address(addr) => self.poke(addr.as_address(), value),
            Operand::Relative(offset) => {
                self.poke((self.relative_base + offset).as_address(), value)
            }
        }
    }

    fn read_pc(&mut self) -> W {
        let result = self.peek(self.program_counter);
        self.program_counter += 1;

        result
    }
}
