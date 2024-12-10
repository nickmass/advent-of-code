pub fn part_one(input: &str) -> u64 {
    let mut lines = input.trim().lines().peekable();

    let line_len = lines.peek().map_or(64, |s| s.len());

    let nums = lines.filter_map(|s| u64::from_str_radix(s, 2).ok());

    let mut one_count = vec![0; line_len];
    let mut line_count = 0;

    for num in nums {
        let mut n = line_len;
        while n != 0 {
            n -= 1;
            let target = 1 << n;

            if num & target != 0 {
                one_count[line_len - 1 - n] += 1;
            }
        }

        line_count += 1;
    }

    let mut gamma = 0;
    let mut epi = 0;

    for &o in one_count.iter().take(line_len) {
        let z = line_count - o;

        gamma <<= 1;
        epi <<= 1;
        if o > z {
            gamma |= 1;
        }

        if o < z {
            epi |= 1;
        }
    }

    gamma * epi
}

pub fn part_two(input: &str) -> u64 {
    let lines: Vec<_> = input.trim().lines().collect();
    let line_len = lines[0].len();

    let find_match = |if_ones, if_zeros| {
        let mut n = 0;
        let mut s = String::with_capacity(line_len);
        let mut lines = lines.clone();
        let mut one_count = vec![0; line_len];
        while lines.len() > 1 {
            one_count.fill(0);

            for line in lines.iter() {
                for (i, c) in line.chars().enumerate() {
                    if c == '1' {
                        one_count[i] += 1;
                    }
                }
            }

            let o = one_count[n];
            let z = lines.len() - o;

            n += 1;

            if o >= z {
                s.push(if_ones);
            } else {
                s.push(if_zeros);
            }

            lines.retain(|l| l.starts_with(&s));
        }

        u64::from_str_radix(lines[0], 2).unwrap_or_default()
    };

    let o = find_match('1', '0');
    let co = find_match('0', '1');

    o * co
}

#[test]
fn test() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    assert_eq!(198, part_one(input));
    assert_eq!(230, part_two(input));
}
