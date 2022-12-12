pub fn part_one(input: &str) -> i32 {
    let instructions = input
        .trim()
        .lines()
        .filter_map(|l| l.parse::<Instruction>().ok());

    let mut x = 1;
    let mut cycles = 0;

    let mut next_signal = 20;

    let mut signal_strength = 0;

    for instruction in instructions {
        cycles += instruction.cycles();

        if cycles >= next_signal {
            signal_strength += next_signal * x;
            next_signal += 40;
        }

        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(n) => x += n,
        }
    }

    signal_strength
}

pub fn part_two(input: &str) -> String {
    let instructions = input
        .trim()
        .lines()
        .filter_map(|l| l.parse::<Instruction>().ok());

    let mut screen = String::with_capacity(41 * 6 * 2);
    let mut x = 1;
    let mut cycles = 0;

    for instruction in instructions {
        let next_cycles = cycles + instruction.cycles();

        for n in cycles..next_cycles {
            let v = n % 40;
            if v == 0 && n != 0 {
                screen.push('\n');
            }
            if v == x || v == x + 1 || v == x - 1 {
                screen.push_str("##");
            } else {
                screen.push_str("  ");
            }
        }

        cycles = next_cycles;

        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(n) => x += n,
        }
    }

    screen
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

struct InstructionParseErr;

impl From<std::num::ParseIntError> for InstructionParseErr {
    fn from(_: std::num::ParseIntError) -> Self {
        Self
    }
}

impl std::str::FromStr for Instruction {
    type Err = InstructionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        match s.split_once(" ") {
            Some(("addx", n)) => Ok(Instruction::Addx(n.parse()?)),
            _ => Err(InstructionParseErr),
        }
    }
}

#[test]
fn test() {
    let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    assert_eq!(13140, part_one(input));

    let screen = r#"####    ####    ####    ####    ####    ####    ####    ####    ####    ####    
######      ######      ######      ######      ######      ######      ######  
########        ########        ########        ########        ########        
##########          ##########          ##########          ##########          
############            ############            ############            ########
##############              ##############              ##############          "#;

    assert_eq!(screen, part_two(input));
}
