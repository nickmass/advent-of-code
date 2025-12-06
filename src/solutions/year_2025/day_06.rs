pub fn part_one(input: &str) -> u64 {
    let (numbers, mut ops) = parse_operations(input);

    for line in numbers.lines() {
        for (num, op) in line
            .split_ascii_whitespace()
            .filter_map(|x| x.parse::<u64>().ok())
            .zip(ops.iter_mut())
        {
            op.push(num);
        }
    }

    ops.iter().map(|op| op.total()).sum()
}

pub fn part_two(input: &str) -> u64 {
    let (numbers, mut ops) = parse_operations(input);
    let numbers = NumberGrid::new(numbers);

    let mut numbers_iter = numbers.numbers();

    for op in ops.iter_mut() {
        while let Some(Some(num)) = numbers_iter.next() {
            op.push(num);
        }
    }

    ops.iter().map(|op| op.total()).sum()
}

fn parse_operations(input: &str) -> (&str, Vec<Operation>) {
    let input = input.trim_end_matches('\n');
    let op_line_idx = input.rfind('\n').expect("op line") + 1;

    let (numbers, op_line) = input.split_at(op_line_idx);

    let mut operations = Vec::new();
    for b in op_line.as_bytes() {
        match b {
            b'+' => operations.push(Operation::Add(None)),
            b'*' => operations.push(Operation::Mul(None)),
            _ => (),
        }
    }

    (numbers, operations)
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(Option<u64>),
    Mul(Option<u64>),
}

impl Operation {
    fn push(&mut self, num: u64) {
        *self = match self {
            Operation::Add(x) => Operation::Add(Some(num + x.unwrap_or(0))),
            Operation::Mul(x) => Operation::Mul(Some(num * x.unwrap_or(1))),
        };
    }

    fn total(&self) -> u64 {
        match self {
            Operation::Add(x) => x.unwrap_or(0),
            Operation::Mul(x) => x.unwrap_or(0),
        }
    }
}

struct NumberGrid<'a> {
    text: &'a [u8],
    stride: usize,
}

impl<'a> NumberGrid<'a> {
    fn new(text: &'a str) -> Self {
        let text = text.as_bytes();
        let stride = text
            .iter()
            .position(|b| *b == b'\n')
            .expect("atleast one line")
            + 1;

        Self { text, stride }
    }

    fn numbers(&self) -> impl Iterator<Item = Option<u64>> + '_ {
        let mut column = 0;
        std::iter::from_fn(move || {
            if column >= self.stride {
                return None;
            }
            let mut row = 0;
            let mut value = None;
            loop {
                let idx = row * self.stride + column;
                let Some(digit) = self.text.get(idx) else {
                    break;
                };

                let digit = match digit {
                    b'0'..=b'9' => Some(digit - b'0'),
                    _ => None,
                };

                if let Some(digit) = digit {
                    let digit = digit as u64;
                    if let Some(value) = value.as_mut() {
                        *value *= 10;
                        *value += digit;
                    } else {
                        value = Some(digit);
                    }
                }
                row += 1;
            }

            column += 1;

            Some(value)
        })
    }
}

#[test]
fn test() {
    let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    assert_eq!(4277556, part_one(input));
    assert_eq!(3263827, part_two(input));
}
