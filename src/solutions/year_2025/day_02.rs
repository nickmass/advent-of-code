use std::fmt::Write;

pub fn part_one(input: &str) -> u64 {
    let items = parse_input(input);

    let mut buf = String::new();
    let mut invalids = 0;

    for (start, end) in items {
        for n in start..=end {
            buf.clear();
            let _ = write!(buf, "{}", n);

            let len = buf.len();

            if len & 1 == 1 {
                continue;
            }

            let len = len >> 1;

            if buf[0..len] == buf[len..] {
                invalids += n;
            }
        }
    }

    invalids
}

pub fn part_two(input: &str) -> u64 {
    let items = parse_input(input);

    let mut buf = String::new();
    let mut invalids = 0;

    for (start, end) in items {
        for n in start..=end {
            buf.clear();
            let _ = write!(buf, "{}", n);

            let len = buf.len();

            'outer: for sub_len in 1..=(len / 2) {
                if len % sub_len != 0 {
                    continue;
                }

                let check = &buf[0..sub_len];

                for offset in (sub_len..len).step_by(sub_len) {
                    let test = &buf[offset..(offset + sub_len)];
                    if check != test {
                        continue 'outer;
                    }
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

#[test]
fn test() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,\
        446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(1227775554, part_one(input));
    assert_eq!(4174379265, part_two(input));
}
