use super::intcode::{Interrupt, Machine};
use std::collections::VecDeque;

pub fn part_one(input: &str) -> String {
    let mut machine = Machine::<i64, _>::new(input);

    let moves = [
        Move::North,
        Move::Take("easter egg"),
        Move::East,
        Move::Take("astrolabe"),
        Move::South,
        Move::Take("space law space brochure"),
        Move::North,
        Move::North,
        Move::North,
        Move::Take("fuel cell"),
        Move::South,
        Move::South,
        Move::West,
        Move::North,
        Move::Take("manifold"),
        Move::North,
        Move::North,
        Move::Take("hologram"),
        Move::North,
        Move::Take("weather machine"),
        Move::North,
        Move::Take("antenna"),
        Move::West,
        Move::Inv,
        Move::South,
        Move::Validate,
    ];

    let mut input_buf = String::new();
    let mut input = VecDeque::new();
    let mut output = String::new();

    const DEBUG: bool = false;

    let mut code_found = false;

    'outer: for code in 0..256 {
        let mut moves_iter = moves.iter();
        machine.reset();
        output.clear();
        input.clear();
        let mut item_num = 0;
        loop {
            match machine.run() {
                Interrupt::Input => {
                    if let Some(next) = input.pop_front() {
                        machine.set_input(next as u8 as i64);
                    } else {
                        let next = loop {
                            if let Some(next) = moves_iter.next() {
                                if let Move::Take(_) = next {
                                    let item_id = 1 << item_num;
                                    item_num += 1;
                                    if code & item_id != 0 {
                                        break Some(next);
                                    } else {
                                        continue;
                                    }
                                } else if let Move::Inv = next {
                                    output.clear();
                                    break Some(next);
                                } else {
                                    break Some(next);
                                }
                            } else {
                                break None;
                            }
                        };

                        if let Some(next) = next {
                            if let Move::Validate = next {
                                if output.contains("ejected back to the checkpoint") {
                                    continue 'outer;
                                }
                            }
                            let next = next.to_string();

                            if DEBUG {
                                println!("{}", next);
                            }

                            input.extend(next.to_string().chars());
                            input.push_back('\n');
                        } else if DEBUG {
                            let _ = std::io::stdin().read_line(&mut input_buf).unwrap();
                            input.extend(input_buf.drain(..))
                        } else {
                            break;
                        }
                    }
                }
                Interrupt::Output(c) => {
                    let c = c as u8 as char;

                    if DEBUG {
                        print!("{}", c)
                    }

                    output.push(c);
                }
                Interrupt::Halt => {
                    code_found = true;
                    break 'outer;
                }
            }
        }
    }

    if code_found {
        output
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
    } else {
        String::from("code not found")
    }
}

enum Move {
    North,
    South,
    East,
    West,
    Take(&'static str),
    Inv,
    Validate,
}

impl Move {
    fn to_string(&self) -> String {
        match self {
            Move::North => "north".into(),
            Move::South => "south".into(),
            Move::East => "east".into(),
            Move::West => "west".into(),
            Move::Take(item) => format!("take {}", item),
            Move::Inv => "inv".into(),
            Move::Validate => "inv".into(),
        }
    }
}

pub fn part_two(_input: &str) -> &'static str {
    "Almost there..."
}
