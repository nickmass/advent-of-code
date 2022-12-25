pub fn part_one(input: &str) -> String {
    let sum = input.trim().lines().map(snafu_to_dec).sum();

    dec_to_snafu(sum)
}

pub fn part_two(_input: &str) -> &'static str {
    "Almost done..."
}

fn dec_to_snafu(mut input: i64) -> String {
    let mut snafu = Vec::new();

    while input != 0 {
        let (ones, carry) = match input % 5 {
            3 => (-2, 1),
            4 => (-1, 1),
            ones => (ones, 0),
        };

        snafu.push(dec_digit_to_snafu(ones));

        input = input / 5 + carry;
    }

    snafu.reverse();
    snafu.into_iter().collect()
}

fn snafu_to_dec(input: &str) -> i64 {
    let mut sum = 0;
    let mut base = 1;

    for c in input.chars().rev() {
        sum += snafu_digit_to_dec(c) * base;
        base *= 5;
    }

    sum
}

fn dec_digit_to_snafu(d: i64) -> char {
    match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => unreachable!(),
    }
}

fn snafu_digit_to_dec(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    let input = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

    assert_eq!("2=-1=0".to_string(), part_one(input));
}
