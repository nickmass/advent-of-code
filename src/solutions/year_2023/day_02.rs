pub fn part_one(input: &str) -> u64 {
    const MAX_RED: u64 = 12;
    const MAX_GREEN: u64 = 13;
    const MAX_BLUE: u64 = 14;

    let mut valid_games = 0;
    'line: for line in input.trim().lines() {
        let line = &line["Game ".len()..];
        let (id, line) = line.split_once(": ").unwrap();

        for round in line.split("; ") {
            for sample in round.split(", ") {
                let (count, color) = sample.split_once(" ").unwrap();
                let count = count.parse::<u64>().unwrap();

                let valid = match color {
                    "red" if count > MAX_RED => false,
                    "green" if count > MAX_GREEN => false,
                    "blue" if count > MAX_BLUE => false,
                    _ => true,
                };

                if !valid {
                    continue 'line;
                }
            }
        }

        valid_games += id.parse::<u64>().unwrap();
    }

    valid_games
}

pub fn part_two(input: &str) -> u64 {
    let mut power_level = 0;

    for line in input.trim().lines() {
        let line = &line["Game ".len()..];
        let (_id, line) = line.split_once(": ").unwrap();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for round in line.split("; ") {
            for sample in round.split(", ") {
                let (count, color) = sample.split_once(" ").unwrap();
                let count = count.parse().unwrap();

                match color {
                    "red" if count > min_red => min_red = count,
                    "green" if count > min_green => min_green = count,
                    "blue" if count > min_blue => min_blue = count,
                    _ => (),
                }
            }
        }

        power_level += min_red * min_green * min_blue;
    }

    power_level
}

#[test]
fn test() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(8, part_one(input));
    assert_eq!(2286, part_two(input));
}
