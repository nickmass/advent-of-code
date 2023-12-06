pub fn part_one(input: &str) -> u64 {
    let (times, distances) = input.trim().split_once("\n").unwrap();
    let times = times["Time:".len()..]
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u64>().unwrap());
    let distances = distances["Distance:".len()..]
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u64>().unwrap());

    let mut counts = 1;

    for (time, distance) in times.zip(distances) {
        let mut count = 0;
        for hold_time in 1..time {
            let remaining_time = time - hold_time;

            if (hold_time * remaining_time) > distance {
                count += 1;
            } else if count > 0 {
                break;
            }
        }
        counts *= count;
    }

    counts
}

pub fn part_two(input: &str) -> u64 {
    let (time, distance) = input.trim().split_once("\n").unwrap();
    let time = time
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();
    let distance = distance
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();

    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse::<u64>().unwrap();

    let mut count = 0;
    for hold_time in 1..time {
        let remaining_time = time - hold_time;

        if (hold_time * remaining_time) > distance {
            count += 1;
        } else if count > 0 {
            break;
        }
    }

    count
}

#[test]
fn test() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200
"#;

    assert_eq!(288, part_one(input));
    assert_eq!(71503, part_two(input));
}
