pub fn part_one(mut input: &str) -> u32 {
    let mut sum = 0;
    while input.len() > 0 {
        let (s, n) = match_mul(input);
        input = s;
        sum += n.unwrap_or(0);
    }

    sum
}

pub fn part_two(mut input: &str) -> u32 {
    let mut sum = 0;
    let mut enabled = true;
    while input.len() > 0 {
        if let Some(s) = match_token(input, "do()") {
            input = s;
            enabled = true;
        } else if let Some(s) = match_token(input, "don't()") {
            input = s;
            enabled = false;
        } else {
            let (s, n) = match_mul(input);
            input = s;

            if enabled {
                sum += n.unwrap_or(0);
            }
        }
    }

    sum
}

fn match_mul(s: &str) -> (&str, Option<u32>) {
    let Some(s) = match_token(s, "mul(") else {
        return (&s[1..], None);
    };

    let Some((s, a)) = take_3_digit(s) else {
        return (s, None);
    };

    let Some(s) = match_token(s, ",") else {
        return (s, None);
    };

    let Some((s, b)) = take_3_digit(s) else {
        return (s, None);
    };

    let Some(s) = match_token(s, ")") else {
        return (s, None);
    };

    (&s, Some(a * b))
}

fn match_token<'a>(s: &'a str, token: &str) -> Option<&'a str> {
    if !s.starts_with(token) {
        return None;
    }

    Some(&s[token.len()..])
}

fn take_3_digit(s: &str) -> Option<(&str, u32)> {
    if let Ok(n) = s[0..3].parse::<u32>() {
        Some((&s[3..], n))
    } else if let Ok(n) = s[0..2].parse::<u32>() {
        Some((&s[2..], n))
    } else if let Ok(n) = s[0..1].parse::<u32>() {
        Some((&s[1..], n))
    } else {
        None
    }
}

#[test]
fn test() {
    let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    assert_eq!(161, part_one(input));

    let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    assert_eq!(48, part_two(input));
}
