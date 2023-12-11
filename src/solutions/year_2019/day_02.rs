use super::intcode::Machine;

pub fn part_one(input: &str) -> i32 {
    solve_part_one::<12, 2>(input)
}

fn solve_part_one<const A: i32, const B: i32>(input: &str) -> i32 {
    let mut machine = Machine::new(input);
    machine.poke(1, A);
    machine.poke(2, B);
    machine.run();
    machine.peek(0)
}

pub fn part_two(input: &str) -> i32 {
    let mut machine = Machine::new(input);

    for a in 0..10000 {
        machine.poke(1, a / 100);
        machine.poke(2, a % 100);
        machine.run();

        if machine.peek(0) == 19690720 {
            return a;
        }

        machine.reset();
    }

    panic!("Missed It")
}

#[test]
fn test() {
    let input = r#"1,9,10,3,2,3,11,0,99,30,40,50"#;

    assert_eq!(3500, solve_part_one::<9, 10>(input));
}
