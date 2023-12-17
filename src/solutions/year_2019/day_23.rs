use super::intcode::{Interrupt, Machine, VecMem};
use std::collections::VecDeque;

pub fn part_one(input: &str) -> i64 {
    let mut machines = Vec::with_capacity(50);
    for id in 0..50 {
        machines.push(NetworkMachine::new(input, id));
    }

    for id in (0..).map(|i| i % 50) {
        if let Some(Packet { destination, x, y }) = machines[id].tick() {
            if destination == 255 {
                return y;
            } else {
                machines[destination].send(x, y);
            }
        }
    }

    unreachable!()
}

pub fn part_two(input: &str) -> i64 {
    let mut machines = Vec::with_capacity(50);
    for id in 0..50 {
        machines.push(NetworkMachine::new(input, id));
    }

    let mut nat = None;
    let mut nat_last_sent = None;

    for id in (0..).map(|i| i % 50) {
        if id == 0 {
            let all_idle = machines.iter().all(NetworkMachine::idle);
            if let Some((x, y)) = nat.filter(|_| all_idle) {
                if nat_last_sent == Some(y) {
                    return y;
                }
                machines[0].send(x, y);
                nat_last_sent = Some(y);
            }
        }

        if let Some(Packet { destination, x, y }) = machines[id].tick() {
            if destination == 255 {
                nat = Some((x, y));
            } else {
                machines[destination].send(x, y);
            }
        }
    }

    unreachable!()
}

#[derive(Debug, Copy, Clone)]
struct Packet {
    destination: usize,
    x: i64,
    y: i64,
}

struct NetworkMachine {
    machine: Machine<i64, VecMem<i64>>,
    buffer: VecDeque<i64>,
    output: (Option<i64>, Option<i64>),
    waiting: bool,
}

impl NetworkMachine {
    fn new(firmware: &str, id: i64) -> Self {
        let mut machine = Machine::new(firmware);
        let buffer = VecDeque::new();
        machine.set_input(id);

        let output = (None, None);

        Self {
            machine,
            buffer,
            output,
            waiting: false,
        }
    }

    fn send(&mut self, x: i64, y: i64) {
        self.buffer.push_back(x);
        self.buffer.push_back(y);
    }

    fn tick(&mut self) -> Option<Packet> {
        self.waiting = false;
        match self.machine.run() {
            Interrupt::Input => {
                let next = if let Some(next) = self.buffer.pop_front() {
                    next
                } else {
                    self.waiting = true;
                    -1
                };
                self.machine.set_input(next);
            }
            Interrupt::Output(n) => match self.output {
                (None, None) => self.output.0 = Some(n),
                (Some(_), None) => self.output.1 = Some(n),
                (Some(dst), Some(x)) => {
                    let packet = Packet {
                        destination: dst as usize,
                        x,
                        y: n,
                    };
                    self.output = (None, None);
                    return Some(packet);
                }
                _ => unreachable!(),
            },
            Interrupt::Halt => unreachable!(),
        }

        None
    }

    fn idle(&self) -> bool {
        self.buffer.is_empty() && self.waiting
    }
}
