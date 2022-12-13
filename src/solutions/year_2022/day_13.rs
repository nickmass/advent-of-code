pub fn part_one(input: &str) -> usize {
    let packets = input.trim().split("\n\n").filter_map(|pairs| {
        let (left, right) = pairs.split_once("\n")?;
        Some((parse_packet(left), parse_packet(right)))
    });

    let mut count = 0;
    for (idx, (left, right)) in packets.enumerate() {
        if compare_packet(&left, &right) {
            count += idx + 1;
        }
    }

    count
}

pub fn part_two(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .trim()
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| (l, false))
        .chain(["[[2]]", "[[6]]"].into_iter().map(|l| (l, true)))
        .map(|(l, s)| (parse_packet(l), s))
        .collect();

    packets.sort_by(|(l, _), (r, _)| {
        if compare_packet(l, r) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    packets
        .iter()
        .enumerate()
        .filter(|(_, (_, s))| *s)
        .map(|(i, _)| i + 1)
        .product()
}

fn compare_packet(left: &[PacketToken], right: &[PacketToken]) -> bool {
    let mut vleft = left.to_vec();
    let mut vright = right.to_vec();

    let mut lx = 0;
    let mut rx = 0;

    loop {
        let left = vleft.get(lx).copied();
        let right = vright.get(rx).copied();
        match (left, right) {
            (Some(PacketToken::Open), Some(PacketToken::Open))
            | (Some(PacketToken::Close), Some(PacketToken::Close)) => (),
            (Some(PacketToken::Open), Some(PacketToken::Num(_))) => {
                vright.insert(rx + 1, PacketToken::Close);

                rx -= 1;
            }
            (Some(PacketToken::Num(_)), Some(PacketToken::Open)) => {
                vleft.insert(lx + 1, PacketToken::Close);

                lx -= 1;
            }
            (Some(PacketToken::Num(nl)), Some(PacketToken::Num(nr))) => {
                if nl < nr {
                    return true;
                } else if nl > nr {
                    return false;
                }
            }
            (_, Some(PacketToken::Close)) => return false,
            (Some(PacketToken::Close), _) => return true,
            (None, Some(_)) => return true,
            (Some(_), None) => return false,
            (None, None) => return true,
        }
        lx += 1;
        rx += 1;
    }
}

fn parse_packet(input: &str) -> Vec<PacketToken> {
    let mut num_start = None;
    let mut tokens = Vec::new();
    for (idx, &b) in input.as_bytes().iter().enumerate() {
        if b == b']' || b == b',' {
            if let Some(start) = num_start {
                let num = input[start..idx].parse::<i32>().unwrap();
                tokens.push(PacketToken::Num(num));
                num_start = None;
            }
        }
        match b {
            b'[' => tokens.push(PacketToken::Open),
            b']' => tokens.push(PacketToken::Close),
            b'0'..=b'9' if num_start.is_none() => num_start = Some(idx),
            _ => (),
        }
    }

    tokens
}

#[derive(Debug, Copy, Clone)]
enum PacketToken {
    Open,
    Close,
    Num(i32),
}

#[test]
fn test() {
    let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    assert_eq!(13, part_one(input));
    assert_eq!(140, part_two(input));
}
