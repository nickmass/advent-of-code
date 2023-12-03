use super::intcode;

pub fn part_one(input: &str) -> i64 {
    let mut machine = intcode::Machine::new(input);
    let mut result = 0;
    loop {
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Input => machine.set_input(1),
            intcode::Interrupt::Output(value) => result = value,
        }
    }

    result
}

pub fn part_two(input: &str) -> i64 {
    let mut machine = intcode::Machine::new(input);
    let mut result = 0;
    loop {
        match machine.run() {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Input => machine.set_input(2),
            intcode::Interrupt::Output(value) => result = value,
        }
    }

    result
}

#[test]
fn test() {
    let input = r#"104,1125899906842624,99"#;

    assert_eq!(1125899906842624, part_one(input));
}
