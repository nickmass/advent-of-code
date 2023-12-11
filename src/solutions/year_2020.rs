use crate::solutions::SolutionCollection;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

pub fn days() -> SolutionCollection {
    SolutionCollection::new()
        .add(1, day_01::part_one, day_01::part_two)
        .add(2, day_02::part_one, day_02::part_two)
        .add(3, day_03::part_one, day_03::part_two)
        .add(4, day_04::part_one, day_04::part_two)
        .add(5, day_05::part_one, day_05::part_two)
        .add(6, day_06::part_one, day_06::part_two)
        .add(7, day_07::part_one, day_07::part_two)
        .add(8, day_08::part_one, day_08::part_two)
        .add(9, day_09::part_one, day_09::part_two)
        .add(10, day_10::part_one, day_10::part_two)
        .add(11, day_11::part_one, day_11::part_two)
        .add(12, day_12::part_one, day_12::part_two)
        .add(13, day_13::part_one, day_13::part_two)
        .add(14, day_14::part_one, day_14::part_two)
        .add(15, day_15::part_one, day_15::part_two)
        .add(16, day_16::part_one, day_16::part_two)
        .add(17, day_17::part_one, day_17::part_two)
        .add(18, day_18::part_one, day_18::part_two)
        .add(19, day_19::part_one, day_19::part_two)
        .add(20, day_20::part_one, day_20::part_two)
        .add(21, day_21::part_one, day_21::part_two)
        .add(22, day_22::part_one, day_22::part_two)
        .add(23, day_23::part_one, day_23::part_two)
        .add(24, day_24::part_one, day_24::part_two)
        .add(25, day_25::part_one, day_25::part_two)
}

pub fn take_u64(s: &str) -> Option<(&str, u64)> {
    let mut index = 0;
    for c in s.chars() {
        if c.is_digit(10) {
            index += 1;
        } else {
            break;
        }
    }

    if index == 0 {
        None
    } else {
        let n = s[0..index].parse().ok();
        let remainder = &s[index..];
        n.map(|n| (remainder, n))
    }
}

pub fn take_token<'a, 'b>(s: &'a str, n: &'b str) -> Option<&'a str> {
    if n.len() > s.len() {
        None
    } else {
        let search = &s[0..n.len()];
        if search == n {
            Some(&s[n.len()..])
        } else {
            None
        }
    }
}

pub fn take_char(s: &str) -> Option<(&str, char)> {
    if s.len() > 0 {
        let c = s.chars().next().unwrap();
        Some((&s[1..], c))
    } else {
        None
    }
}

pub fn take_until<F: Fn(char) -> bool>(s: &str, f: F) -> Option<(&str, &str)> {
    let mut index = 0;
    for c in s.bytes() {
        if f(c as char) {
            break;
        }
        index += 1;
    }

    let (n, s) = s.split_at(index);
    Some((s, n))
}

#[test]
fn test_parse() {
    let s = "123abc";
    let (s, num) = take_u64(s).unwrap();
    assert_eq!(num, 123);
    assert_eq!(s, "abc");

    let s = "abc123";
    let n = take_u64(s);
    assert!(n.is_none());

    let s = "abc123";
    let s = take_token(s, "abc").unwrap();
    let (s, num) = take_u64(s).unwrap();
    assert_eq!(s, "");
    assert_eq!(num, 123);

    let s = "abc123";
    let s = take_token(s, "abc123efg");
    assert!(s.is_none());

    let s = "abc123";
    let (s, n) = take_until(s, |c| c.is_digit(10)).unwrap();
    assert_eq!(s, "123");
    assert_eq!(n, "abc");
}
