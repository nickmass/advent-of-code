use std::str::FromStr;

pub fn part_one(input: &str) -> usize {
    let mut range = input
        .split('-')
        .map(str::trim)
        .map(usize::from_str)
        .filter_map(Result::ok);

    let minimum = range.next().unwrap();
    let maximum = range.next().unwrap();

    let mut matches = Vec::new();
    'outer: for n in minimum..maximum {
        let mut value = n % 10;
        let mut remainder = n / 10;
        let mut has_pair = false;
        for _ in 0..6 {
            let next_value = remainder % 10;

            if value < next_value {
                continue 'outer;
            }

            if value == next_value {
                has_pair = true;
            }

            value = next_value;
            remainder = remainder / 10;
        }

        if has_pair {
            matches.push(n);
        }
    }

    matches.len()
}

pub fn part_two(input: &str) -> usize {
    let mut range = input
        .split('-')
        .map(str::trim)
        .map(usize::from_str)
        .filter_map(Result::ok);

    let minimum = range.next().unwrap();
    let maximum = range.next().unwrap();

    let mut matches = Vec::new();
    'outer: for n in minimum..maximum {
        let mut value = n % 10;
        let mut remainder = n / 10;
        let mut in_pair = 0;
        let mut has_pair = false;
        for _ in 0..6 {
            let next_value = remainder % 10;

            if value < next_value {
                continue 'outer;
            }

            if value == next_value {
                in_pair += 1;
            }
            if value != next_value {
                if in_pair == 1 {
                    has_pair = true;
                }
                in_pair = 0;
            }

            value = next_value;
            remainder = remainder / 10;
        }

        if has_pair {
            matches.push(n);
        }
    }

    matches.len()
}
