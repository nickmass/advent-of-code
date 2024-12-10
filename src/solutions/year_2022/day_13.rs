use std::cmp::Ordering;

pub fn part_one(input: &str) -> usize {
    let packets = input.trim().split("\n\n").filter_map(|pairs| {
        let (left, right) = pairs.split_once("\n")?;
        Some((parse_packet(left), parse_packet(right)))
    });

    let mut count = 0;
    for (idx, (left, right)) in packets.enumerate() {
        if left < right {
            count += idx + 1;
        }
    }

    count
}

pub fn part_two(input: &str) -> usize {
    let special_packets = ["[[2]]", "[[6]]"].into_iter().map(|l| {
        let p = parse_packet(l);
        (p, true)
    });

    let mut packets: Vec<_> = input
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| (parse_packet(l), false))
        .chain(special_packets)
        .collect();

    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter(|(_, (_, s))| *s)
        .map(|(i, _)| i + 1)
        .product()
}

enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

fn compare_packet(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
        (Packet::List(l), Packet::List(r)) => {
            for (l, r) in l.iter().zip(r.iter()) {
                let result = compare_packet(l, r);
                if result.is_ne() {
                    return result;
                }
            }

            l.len().cmp(&r.len())
        }
        (Packet::List(_), Packet::Num(r)) => {
            compare_packet(left, &Packet::List(vec![Packet::Num(*r)]))
        }
        (Packet::Num(l), Packet::List(_)) => {
            compare_packet(&Packet::List(vec![Packet::Num(*l)]), right)
        }
    }
}

fn parse_packet(input: &str) -> Packet {
    let mut num_start = None;
    let mut stack = Vec::new();

    for (idx, &b) in input.as_bytes().iter().enumerate() {
        if b == b']' || b == b',' {
            if let Some(start) = num_start.take() {
                let num = input[start..idx].parse::<i32>().unwrap();
                if let Some(Packet::List(list)) = stack.last_mut() {
                    list.push(Packet::Num(num));
                }
            }
        }
        match b {
            b'[' => {
                stack.push(Packet::List(Vec::new()));
            }
            b']' => {
                let packet = stack.pop().unwrap();
                if let Some(Packet::List(list)) = stack.last_mut() {
                    list.push(packet)
                } else {
                    return packet;
                }
            }
            b'0'..=b'9' if num_start.is_none() => num_start = Some(idx),
            _ => (),
        }
    }

    unreachable!()
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_packet(self, other)
    }
}

impl std::cmp::PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl std::cmp::Eq for Packet {}

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
