use crate::HashSet;

pub fn part_one(input: &str) -> u64 {
    let program: Vec<_> = input
        .lines()
        .map(|l| {
            let inst = &l[0..3];
            let n = l[4..].parse().unwrap();
            match inst {
                "nop" => Instruction::Nop(n),
                "acc" => Instruction::Acc(n),
                "jmp" => Instruction::Jmp(n),
                _ => unreachable!("invalid program"),
            }
        })
        .collect();

    let mut hit_instrs = HashSet::new();
    let mut state = EmuState { acc: 0, ip: 0 };

    while !hit_instrs.contains(&state.ip) {
        hit_instrs.insert(state.ip);
        let inst = program[state.ip];
        match inst {
            Instruction::Nop(_) => {
                state.ip += 1;
            }
            Instruction::Acc(n) => {
                state.acc += n;
                state.ip += 1;
            }
            Instruction::Jmp(n) => state.ip = (state.ip as i64 + n) as usize,
        }
    }

    state.acc as u64
}

pub fn part_two(input: &str) -> u64 {
    let program: Vec<_> = input
        .lines()
        .map(|l| {
            let inst = &l[0..3];
            let n = l[4..].parse().unwrap();
            match inst {
                "nop" => Instruction::Nop(n),
                "acc" => Instruction::Acc(n),
                "jmp" => Instruction::Jmp(n),
                _ => unreachable!("invalid program"),
            }
        })
        .collect();

    let mut changed_instrs = HashSet::new();
    let mut hit_instrs = HashSet::new();
    loop {
        let mut changed = false;
        let mut state = EmuState { acc: 0, ip: 0 };
        hit_instrs.clear();
        while !hit_instrs.contains(&state.ip) {
            if state.ip == program.len() {
                return state.acc as u64;
            }
            hit_instrs.insert(state.ip);
            let inst = program[state.ip];
            match inst {
                Instruction::Nop(n) => {
                    if !changed_instrs.contains(&state.ip) && !changed {
                        changed_instrs.insert(state.ip);
                        changed = true;
                        state.ip = (state.ip as i64 + n) as usize;
                    } else {
                        state.ip += 1;
                    }
                }
                Instruction::Acc(n) => {
                    state.acc += n;
                    state.ip += 1;
                }
                Instruction::Jmp(n) => {
                    if !changed_instrs.contains(&state.ip) && !changed {
                        changed_instrs.insert(state.ip);
                        changed = true;
                        state.ip += 1;
                    } else {
                        state.ip = (state.ip as i64 + n) as usize;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct EmuState {
    acc: i64,
    ip: usize,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

#[test]
fn test() {
    let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#;

    assert_eq!(5, part_one(input));
    assert_eq!(8, part_two(input));
}
