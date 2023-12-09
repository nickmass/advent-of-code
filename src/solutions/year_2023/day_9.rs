pub fn part_one(input: &str) -> i64 {
    let lines = input
        .trim()
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse::<i64>().unwrap()));

    let mut result = 0;

    let mut diffs = Vec::new();
    for line in lines {
        diffs.clear();
        diffs.extend(line);

        let mut n = 0;
        let mut m = diffs.len();

        loop {
            let mut prev = None;
            let mut all_zero = true;
            for i in n..m {
                let x = diffs[i];
                if let Some(prev) = prev {
                    let diff = x - prev as i64;
                    diffs.push(diff);
                    if diff != 0 {
                        all_zero = false;
                    }
                }
                prev = Some(x)
            }

            n = m;
            m = diffs.len();

            if all_zero {
                break;
            }
        }

        let mut last = 0;
        let mut size = m - n;
        let mut i = n - 1;
        loop {
            last = diffs[i] + last;
            size += 1;

            if size > i {
                break;
            }
            i -= size;
        }

        result += last;
    }

    result
}

pub fn part_two(input: &str) -> i64 {
    let lines = input
        .trim()
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse::<i64>().unwrap()));

    let mut result = 0;

    let mut diffs = Vec::new();
    for line in lines {
        diffs.clear();
        diffs.extend(line);

        let mut n = 0;
        let mut m = diffs.len();

        loop {
            let mut all_zero = true;
            let mut prev = None;
            for i in n..m {
                let x = diffs[i];
                if let Some(prev) = prev {
                    let diff = x - prev as i64;
                    diffs.push(diff);
                    if diff != 0 {
                        all_zero = false;
                    }
                }
                prev = Some(x)
            }

            n = m;
            m = diffs.len();

            if all_zero {
                break;
            }
        }

        let mut last = 0;
        let mut size = m - n;
        let mut i = n - 1 - size;
        size += 1;

        loop {
            last = diffs[i] - last;
            size += 1;

            if size > i {
                break;
            }
            i -= size;
        }

        result += last;
    }

    result
}

#[test]
fn test() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    assert_eq!(114, part_one(input));
    assert_eq!(2, part_two(input));
}
