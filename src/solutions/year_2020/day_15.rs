pub fn part_one(input: &str) -> u32 {
    solution(input, 2020)
}

pub fn part_two(input: &str) -> u32 {
    solution(input, 30000000)
}

fn solution(input: &str, nth_number: u32) -> u32 {
    let nums = input.trim().split(',').filter_map(|n| n.parse().ok());
    let mut turn = 0;
    let mut map = vec![u32::MAX; nth_number as usize];
    let mut last_num = None;
    for num in nums {
        if let Some(last_num) = last_num {
            map[last_num as usize] = turn - 1;
        }
        last_num = Some(num);
        turn += 1;
    }

    if let Some(mut last_num) = last_num {
        let start = turn - 1;
        let end = nth_number - 1;

        for turn in start..end {
            let val = map[last_num as usize];
            let num = if val != u32::MAX { turn - val } else { 0 };

            map[last_num as usize] = turn;
            last_num = num;
        }

        last_num
    } else {
        0
    }
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let input = r#"0,3,6"#;
    run_a(input, 436);
    run_b(input, 175594);

    let input = r#"1,3,2"#;
    run_a(input, 1);

    let input = r#"2,1,3"#;
    run_a(input, 10);
    let input = r#"1,2,3"#;
    run_a(input, 27);
}
