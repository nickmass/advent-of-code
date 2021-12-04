pub fn part_one(input: &str) -> u64 {
    let mut input = input.trim().lines().filter_map(|n| n.parse().ok());
    let card_pub_key: u64 = input.next().unwrap();
    let door_pub_key: u64 = input.next().unwrap();
    let divisor = 20201227;

    let subject_num = 7;
    let mut card_loops = 0;
    let mut value = 1;

    loop {
        card_loops += 1;
        value *= subject_num;
        value %= divisor;

        if value == card_pub_key {
            break;
        }
    }

    let mut result = 1;
    let subject_num = door_pub_key;

    for _ in 0..card_loops {
        result *= subject_num;
        result %= divisor;
    }

    result
}

pub fn part_two(_input: &str) -> &'static str {
    "I did it!!!!"
}
