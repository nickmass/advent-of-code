pub fn part_one(input: &str) -> u64 {
    let mut result = 0;
    for line in input.trim().lines() {
        result += eval_expression(line, true);
    }
    result
}

pub fn part_two(input: &str) -> u64 {
    let mut result = 0;
    for line in input.trim().lines() {
        result += eval_expression(line, false);
    }
    result
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Value(u64),
    Op(Op),
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

impl Op {
    fn precedence(&self) -> i32 {
        match *self {
            Op::Add => 2,
            Op::Mul => 1,
            _ => 0,
        }
    }
}

fn eval_expression(expression: &str, equal_precedence: bool) -> u64 {
    let mut tokens = Vec::with_capacity(expression.len());
    let mut value_builder = None;
    for c in expression.chars() {
        if c.is_ascii_digit() {
            let v = (c as u8 - b'0') as u64;
            value_builder = Some((value_builder.unwrap_or(0) * 10) + v);
        } else if let Some(val) = value_builder {
            tokens.push(Token::Value(val));
            value_builder = None;
        }
        match c {
            '+' => tokens.push(Token::Op(Op::Add)),
            '*' => tokens.push(Token::Op(Op::Mul)),
            '(' => tokens.push(Token::Op(Op::OpenParen)),
            ')' => tokens.push(Token::Op(Op::CloseParen)),
            _ => (),
        }
    }

    if let Some(val) = value_builder {
        tokens.push(Token::Value(val));
    }

    let mut output = Vec::with_capacity(tokens.len());
    let mut operators = Vec::with_capacity(tokens.len());
    for token in tokens {
        match token {
            Token::Value(n) => output.push(Token::Value(n)),
            Token::Op(Op::OpenParen) => operators.push(Op::OpenParen),
            Token::Op(Op::CloseParen) => {
                while let Some(op) = operators.pop() {
                    match op {
                        Op::OpenParen => {
                            break;
                        }
                        op => output.push(Token::Op(op)),
                    }
                }
            }
            Token::Op(op) => {
                while let Some(next_op) = operators.last() {
                    match next_op {
                        Op::OpenParen | Op::CloseParen => {
                            break;
                        }
                        next_op if equal_precedence || next_op.precedence() >= op.precedence() => {
                            let next_op = operators.pop().unwrap();
                            output.push(Token::Op(next_op));
                        }
                        _ => {
                            break;
                        }
                    }
                }
                operators.push(op)
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(Token::Op(op));
    }

    let mut eval_stack = Vec::with_capacity(output.len());
    for token in output {
        match token {
            Token::Value(n) => eval_stack.push(n),
            Token::Op(Op::Add) => {
                let lhs = eval_stack.pop().unwrap();
                let rhs = eval_stack.pop().unwrap();
                eval_stack.push(lhs + rhs);
            }
            Token::Op(Op::Mul) => {
                let lhs = eval_stack.pop().unwrap();
                let rhs = eval_stack.pop().unwrap();
                eval_stack.push(lhs * rhs);
            }
            _ => unreachable!("invalid rpn"),
        }
    }

    eval_stack.pop().unwrap()
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"1 + 2 * 3 + 4 * 5 + 6"#;
    run_a(i, 71);
    run_b(i, 231);

    let i = r#"1 + (2 * 3) + (4 * (5 + 6))"#;
    run_a(i, 51);
    run_b(i, 51);

    let i = r#"2 * 3 + (4 * 5)"#;
    run_a(i, 26);
    run_b(i, 46);

    let i = r#"5 + (8 * 3 + 9 + 3 * 4 * 3)"#;
    run_a(i, 437);
    run_b(i, 1445);

    let i = r#"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"#;
    run_a(i, 12240);
    run_b(i, 669060);

    let i = r#"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#;
    run_a(i, 13632);
    run_b(i, 23340);
}
