use std::str::FromStr;

pub fn part_one(input: &str) -> i32 {
    input
        .split('\n')
        .map(i32::from_str)
        .filter_map(Result::ok)
        .map(|f| f / 3 - 2)
        .sum::<i32>()
}

pub fn part_two(input: &str) -> i32 {
    input
        .split('\n')
        .map(i32::from_str)
        .filter_map(Result::ok)
        .map(|f| {
            let mut f_mod = f / 3 - 2;
            let mut fuel_acc = 0;
            while f_mod > 0 {
                fuel_acc += f_mod;
                f_mod = f_mod / 3 - 2;
            }

            fuel_acc
        })
        .sum::<i32>()
}

#[test]
fn test() {
    let input = r#"100756"#;

    assert_eq!(33583, part_one(input));
    assert_eq!(50346, part_two(input));
}
