use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    let lines = input.lines();
    let mut mode = 0;

    let mut rules = Vec::new();
    let mut error_num = 0;
    for line in lines {
        match mode {
            0 => {
                if line == "" {
                    mode += 1;
                    continue;
                }
                let mut splits: Vec<_> = line.split_whitespace().collect();
                splits.reverse();
                let mut left_rules = splits[0].split('-');
                let left_a: u64 = left_rules.next().unwrap().parse().unwrap();
                let left_b: u64 = left_rules.next().unwrap().parse().unwrap();
                let left = left_a..=left_b;

                let mut right_rules = splits[2].split('-');
                let right_a: u64 = right_rules.next().unwrap().parse().unwrap();
                let right_b: u64 = right_rules.next().unwrap().parse().unwrap();
                let right = right_a..=right_b;

                rules.push((left, right));
            }
            1 => {
                mode += 1;
            }
            2 => {
                //my ticket
                mode += 1;
            }
            3 => {
                mode += 1;
            }
            4 => {
                mode += 1;
            }
            5 => {
                let values = line.trim().split(',').filter_map(|n| n.parse().ok());

                'outer: for value in values {
                    for r in rules.iter() {
                        if r.0.contains(&value) {
                            continue 'outer;
                        }
                        if r.1.contains(&value) {
                            continue 'outer;
                        }
                    }

                    error_num += value;
                }
            }
            _ => panic!(),
        }
    }

    error_num
}

pub fn part_two(input: &str) -> u64 {
    let lines = input.lines();
    let mut mode = 0;

    let mut rules = Vec::new();
    let mut tickets = Vec::new();
    let mut my_ticket: Vec<u64> = Vec::new();
    for line in lines {
        match mode {
            0 => {
                if line == "" {
                    mode += 1;
                    continue;
                }
                let mut splits: Vec<_> = line.split_whitespace().collect();
                splits.reverse();
                let mut left_rules = splits[0].split('-');
                let left_a: u64 = left_rules.next().unwrap().parse().unwrap();
                let left_b: u64 = left_rules.next().unwrap().parse().unwrap();
                let left = left_a..=left_b;

                let mut right_rules = splits[2].split('-');
                let right_a: u64 = right_rules.next().unwrap().parse().unwrap();
                let right_b: u64 = right_rules.next().unwrap().parse().unwrap();
                let right = right_a..=right_b;

                let depature = splits.last().unwrap() == &"departure";

                rules.push((left, right, depature, false));
            }
            1 => {
                mode += 1;
            }
            2 => {
                my_ticket = line
                    .trim()
                    .split(',')
                    .filter_map(|n| n.parse().ok())
                    .collect();

                mode += 1;
            }
            3 => {
                mode += 1;
            }
            4 => {
                mode += 1;
            }
            5 => {
                let values: Vec<_> = line
                    .trim()
                    .split(',')
                    .filter_map(|n| n.parse().ok())
                    .collect();

                let mut valid = true;
                'outer: for value in values.iter() {
                    for r in rules.iter() {
                        if r.0.contains(value) {
                            continue 'outer;
                        }
                        if r.1.contains(value) {
                            continue 'outer;
                        }
                    }
                    valid = false;
                }

                if valid {
                    tickets.push(values);
                }
            }
            _ => panic!(),
        }
    }

    let mut rule_map = HashMap::new();
    let column_count = my_ticket.len();

    loop {
        for r in rules.iter_mut() {
            if r.3 {
                continue;
            }
            let mut possible_cols = Vec::new();
            'col: for col in 0..column_count {
                if rule_map.contains_key(&col) {
                    continue 'col;
                }
                for t in tickets.iter() {
                    let v = t[col];
                    if !r.0.contains(&v) && !r.1.contains(&v) {
                        continue 'col;
                    }
                }
                possible_cols.push(col);
            }

            if possible_cols.len() == 1 {
                let col = possible_cols.first().unwrap();
                r.3 = true;
                rule_map.insert(*col, r.clone());
            } else if possible_cols.len() == 0 {
                panic!("shiiit");
            }
        }
        if rule_map.len() == column_count {
            break;
        }
    }

    let mut total = 1;
    for (k, v) in rule_map {
        if v.2 {
            total *= my_ticket[k];
        }
    }

    total
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);

    let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    run_a(input, 71);
}
