pub fn part_one(input: &str) -> u64 {
    let items = parse_input(input);

    let mut invalids = 0;

    for (start, end) in items {
        let mut n = start;
        while n <= end {
            let len = n.ilog10() + 1;

            if len & 1 == 1 {
                n = 10u64.pow(len);
                continue;
            }

            let half_len = (len >> 1) as usize;

            if digit_iter(n)
                .take(half_len)
                .zip(digit_iter(n).skip(half_len))
                .all(|(a, b)| a == b)
            {
                invalids += n;
            }

            n += 1;
        }
    }

    invalids
}

pub fn part_two(input: &str) -> u64 {
    let items = parse_input(input);

    let mut invalids = 0;

    for (start, end) in items {
        for n in start..=end {
            let len = (n.ilog10() + 1) as usize;

            for sub_len in 1..=(len / 2) {
                if len % sub_len != 0 {
                    continue;
                }

                let check = digit_iter(n).take(sub_len).cycle();
                let test = digit_iter(n).skip(sub_len);

                if check.zip(test).any(|(a, b)| a != b) {
                    continue;
                }

                invalids += n;
                break;
            }
        }
    }

    invalids
}

fn parse_input(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input
        .trim()
        .split(',')
        .filter_map(|i| i.split_once('-'))
        .filter_map(|(s, e)| s.parse::<u64>().ok().zip(e.parse::<u64>().ok()))
}

fn digit_iter(mut num: u64) -> impl Iterator<Item = u64> + Clone {
    std::iter::from_fn(move || {
        if num == 0 {
            None
        } else {
            let v = num % 10;
            num /= 10;
            Some(v)
        }
    })
}

#[test]
fn test() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(1227775554, part_one(input));
    assert_eq!(4174379265, part_two(input));
}
