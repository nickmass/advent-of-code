pub fn part_one(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|l| {
            let first = l.bytes().find_map(ascii_digit_value).unwrap_or(0);
            let last = l.bytes().rev().find_map(ascii_digit_value).unwrap_or(0);

            first * 10 + last
        })
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut sum = 0;
    for line in lines {
        let line = line.as_bytes();

        for idx in 0..line.len() {
            if let Some(first) = find_match(line, idx) {
                sum += first * 10;
                break;
            }
        }

        for idx in (0..line.len()).rev() {
            if let Some(last) = find_match(line, idx) {
                sum += last;
                break;
            }
        }
    }

    sum
}

const DIGIT_WORDS: [(u64, &[u8]); 9] = [
    (1, b"one"),
    (2, b"two"),
    (3, b"three"),
    (4, b"four"),
    (5, b"five"),
    (6, b"six"),
    (7, b"seven"),
    (8, b"eight"),
    (9, b"nine"),
];

fn find_match(line: &[u8], idx: usize) -> Option<u64> {
    if let Some(n) = ascii_digit_value(line[idx]) {
        return Some(n);
    } else {
        for &(n, m) in DIGIT_WORDS.iter() {
            if line[idx..].starts_with(m) {
                return Some(n);
            }
        }
    }

    None
}

fn ascii_digit_value(byte: u8) -> Option<u64> {
    match byte {
        b'0' => Some(0),
        b'1' => Some(1),
        b'2' => Some(2),
        b'3' => Some(3),
        b'4' => Some(4),
        b'5' => Some(5),
        b'6' => Some(6),
        b'7' => Some(7),
        b'8' => Some(8),
        b'9' => Some(9),
        _ => None,
    }
}

#[test]
fn test() {
    let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    assert_eq!(142, part_one(input));

    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    assert_eq!(281, part_two(input));
}
