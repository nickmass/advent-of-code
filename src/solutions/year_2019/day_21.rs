use super::intcode::{Interrupt, Machine, VecMem};
use std::iter::once;

macro_rules! springcode {
    (RUN $($token:ident)*) => { springcode!(@build, RunReg, $($token)*) };
    (WALK $($token:ident)*) => { springcode!(@build, WalkReg, $($token)*) };
    (@build, $kind:ident, $($op:ident $r_reg:ident $w_reg:ident)*) =>
    {{
        const P: &[Opcode<$kind>] = &[$(springcode!(@op, $op)($kind::$r_reg, WriteReg::$w_reg)),*];
        const _: () = if P.len() > 15 {
            panic!("exceeded 15 instructions")
        };
        P
    } };
    (@op, AND) => {Opcode::And};
    (@op, OR) => {Opcode::Or};
    (@op, NOT) => {Opcode::Not};
    (@op, $other:expr) => {compile_error!("expected one of 'AND', 'OR', or 'NOT'")};
    ($other:ident $($op:ident $r_reg:ident $w_reg:ident)*) => {compile_error!("must begin with 'WALK' or 'RUN'")};
}

pub fn part_one(input: &str) -> i32 {
    let mut machine = SpringcodeMachine::new(input);

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

    machine.run(program).unwrap_or(0)
}

pub fn part_two(input: &str) -> i32 {
    let mut machine = SpringcodeMachine::new(input);

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
            NOT A T
    };

    machine.run(program).unwrap_or(0)
}

struct SpringcodeMachine<const DEBUG: bool> {
    machine: Machine<i32, VecMem<i32>>,
}

impl SpringcodeMachine<false> {
    fn new(firmware: &str) -> Self {
        Self {
            machine: Machine::new(firmware),
        }
    }
}

impl<const DEBUG: bool> SpringcodeMachine<DEBUG> {
    #[allow(dead_code)]
    fn with_debug(firmware: &str) -> Self {
        Self {
            machine: Machine::new(firmware),
        }
    }

    fn run<'a, I: IntoIterator<Item = &'a Opcode<P>>, P: ProgMode + 'a>(
        &mut self,
        program: I,
    ) -> Option<i32> {
        self.machine.reset();
        loop {
            match self.machine.run() {
                Interrupt::Output(c) if DEBUG => print!("{}", c as u8 as char),
                Interrupt::Output(_) => (),
                Interrupt::Input => break,
                i => unreachable!("{i:?}"),
            }
        }

        for c in program
            .into_iter()
            .flat_map(|op| op.bytes())
            .chain(P::EXEC.iter().copied())
            .chain(once(b'\n'))
        {
            self.machine.set_input(c as i32);
            match self.machine.run() {
                Interrupt::Output(c) if DEBUG => print!("{}", c as u8 as char),
                Interrupt::Output(_) => (),
                Interrupt::Input => (),
                i => unreachable!("{i:?}"),
            }
        }

        loop {
            match self.machine.run() {
                Interrupt::Output(n) if n > 0x7f => return Some(n),
                Interrupt::Output(c) if DEBUG => print!("{}", c as u8 as char),
                Interrupt::Output(_) => (),
                Interrupt::Halt => break,
                i => unreachable!("{i:?}"),
            }
        }

        None
    }
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
            Opcode::And(r, w) => (&b"AND "[..], r, w),
            Opcode::Or(r, w) => (&b"OR "[..], r, w),
            Opcode::Not(r, w) => (&b"NOT "[..], r, w),
        };

        s.iter()
            .copied()
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
