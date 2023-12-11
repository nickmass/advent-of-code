pub fn part_one(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut max = 0;
    let mut cur = 0;

    for line in lines.chain(std::iter::once("")) {
        if line.len() == 0 {
            if cur > max {
                max = cur;
            }

            cur = 0;
        } else {
            cur += line.parse::<u64>().unwrap();
        }
    }

    max
}

pub fn part_two(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut max = [0; 3];
    let mut cur = 0;

    for line in lines.chain(std::iter::once("")) {
        if line.len() == 0 {
            if cur > max[0] {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = cur;
            } else if cur > max[1] {
                max[2] = max[1];
                max[1] = cur;
            } else if cur > max[2] {
                max[2] = cur;
            }

            cur = 0;
        } else {
            cur += line.parse::<u64>().unwrap();
        }
    }

    max.iter().sum()
}

#[test]
fn test() {
    let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    assert_eq!(24000, part_one(input));
    assert_eq!(45000, part_two(input));
}
