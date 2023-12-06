use super::intcode::{Interrupt, Machine, VecMem};
use std::iter::once;

macro_rules! springcode {
    (RUN $($op:ident $r_reg:ident $w_reg:ident)*) => { springcode!(__BUILD, $(springcode!(__OP, $op)(RunReg::$r_reg, WriteReg::$w_reg)),*) };
    (WALK $($op:ident $r_reg:ident $w_reg:ident)*) => { springcode!(__BUILD, $(springcode!(__OP, $op)(WalkReg::$r_reg, WriteReg::$w_reg)),*) };
    (__BUILD, $($code:expr),*) => {{ let p = [$($code),*]; assert!(p.len() <= 15); p }};
    (__OP, AND) => {Opcode::And};
    (__OP, OR) => {Opcode::Or};
    (__OP, NOT) => {Opcode::Not};
    (__OP, $other:expr) => {compile_error!("Expected one of 'AND', 'OR', or 'NOT'")};
    ($other:ident $($op:ident $r_reg:ident $w_reg:ident)*) => {compile_error!("Programs must begin with 'WALK' or 'RUN'")};
}

pub fn part_one(input: &str) -> i32 {
    let mut machine = Machine::new(input);

    let program = springcode! {
            WALK
            OR D J
            NOT T T
            AND A T
            AND B T
            AND C T
            NOT T T
            AND T J
    };

    run_springcode(&mut machine, program, false).unwrap_or(0)
}

pub fn part_two(input: &str) -> i32 {
    let mut machine = Machine::new(input);

    let program = springcode! {
            RUN
            OR D J
            NOT T T
            AND A T
            AND B T
            AND C T
            NOT T T
            AND T J
            NOT A T
            AND A T
            OR H T
            OR E T
            AND T J
    };

    run_springcode(&mut machine, program, false).unwrap_or(0)
}

fn run_springcode<I: IntoIterator<Item = Opcode<P>>, P: ProgMode>(
    machine: &mut Machine<i32, VecMem<i32>>,
    program: I,
    debug: bool,
) -> Option<i32> {
    loop {
        match machine.run() {
            Interrupt::Output(c) if debug => print!("{}", c as u8 as char),
            Interrupt::Output(_) => (),
            Interrupt::Input => break,
            i => unreachable!("{i:?}"),
        }
    }

    for c in program
        .into_iter()
        .map(|op| op.bytes())
        .flatten()
        .chain(P::EXEC.iter().copied())
        .chain(once(b'\n'))
    {
        let c = c as i32;
        machine.set_input(c);
        match machine.run() {
            Interrupt::Output(c) if debug => print!("{}", c as u8 as char),
            Interrupt::Output(_) => (),
            Interrupt::Input => (),
            i => unreachable!("{i:?}"),
        }
    }

    loop {
        match machine.run() {
            Interrupt::Output(n) if n > 0x7f => return Some(n),
            Interrupt::Output(c) if debug => print!("{}", c as u8 as char),
            Interrupt::Output(_) => (),
            Interrupt::Halt => break,
            i => unreachable!("{i:?}"),
        }
    }

    None
}

#[derive(Debug, Copy, Clone)]
enum Opcode<T: ProgMode> {
    And(T, WriteReg),
    Or(T, WriteReg),
    Not(T, WriteReg),
}

trait ProgMode {
    const EXEC: &'static [u8];
    fn as_u8(&self) -> u8;
}

impl<T: ProgMode> Opcode<T> {
    fn bytes(&self) -> impl Iterator<Item = u8> {
        let (s, r, w) = match self {
            Opcode::And(r, w) => ("AND ", r, w),
            Opcode::Or(r, w) => ("OR ", r, w),
            Opcode::Not(r, w) => ("NOT ", r, w),
        };

        s.bytes()
            .chain(once(r.as_u8()))
            .chain(once(b' '))
            .chain(once(w.as_u8()))
            .chain(once(b'\n'))
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum WalkReg {
    J,
    T,
    A,
    B,
    C,
    D,
}

impl ProgMode for WalkReg {
    const EXEC: &'static [u8] = b"WALK";

    fn as_u8(&self) -> u8 {
        match self {
            WalkReg::J => b'J',
            WalkReg::T => b'T',
            WalkReg::A => b'A',
            WalkReg::B => b'B',
            WalkReg::C => b'C',
            WalkReg::D => b'D',
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum RunReg {
    J,
    T,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

impl ProgMode for RunReg {
    const EXEC: &'static [u8] = b"RUN";

    fn as_u8(&self) -> u8 {
        match self {
            RunReg::J => b'J',
            RunReg::T => b'T',
            RunReg::A => b'A',
            RunReg::B => b'B',
            RunReg::C => b'C',
            RunReg::D => b'D',
            RunReg::E => b'E',
            RunReg::F => b'F',
            RunReg::G => b'G',
            RunReg::H => b'H',
            RunReg::I => b'I',
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum WriteReg {
    J,
    T,
}

impl WriteReg {
    const fn as_u8(&self) -> u8 {
        match self {
            WriteReg::J => b'J',
            WriteReg::T => b'T',
        }
    }
}
