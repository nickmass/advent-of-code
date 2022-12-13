pub fn part_one(input: &str) -> usize {
    let packets = input.trim().split("\n\n").filter_map(|pairs| {
        let (left, right) = pairs.split_once("\n")?;
        Some((left.parse::<Packet>().ok()?, right.parse::<Packet>().ok()?))
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
    let special_packets = ["[[2]]", "[[6]]"].into_iter().filter_map(|l| {
        let mut p = l.parse::<Packet>().ok()?;
        p.special = true;
        Some(p)
    });

    let mut packets: Vec<_> = input
        .trim()
        .lines()
        .filter(|l| l.len() > 0)
        .filter_map(|l| l.parse::<Packet>().ok())
        .chain(special_packets)
        .collect();

    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter(|(_, p)| p.special)
        .map(|(i, _)| i + 1)
        .product()
}

fn compare_packet(left: &[PacketToken], right: &[PacketToken]) -> bool {
    let mut lx = 0;
    let mut rx = 0;

    let mut l_close = Closer::new();
    let mut r_close = Closer::new();

    loop {
        let (left, l_faked) = l_close.get(lx, left);
        let (right, r_faked) = r_close.get(rx, right);

        match (left, right) {
            (Some(PacketToken::Open), Some(PacketToken::Open))
            | (Some(PacketToken::Close), Some(PacketToken::Close)) => (),
            (Some(PacketToken::Open), Some(PacketToken::Num(_))) => {
                r_close.close(rx + 1);

                rx -= 1;
            }
            (Some(PacketToken::Num(_)), Some(PacketToken::Open)) => {
                l_close.close(lx + 1);

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

        if !l_faked {
            lx += 1;
        }
        if !r_faked {
            rx += 1;
        }
    }
}

fn parse_packet(input: &str) -> Vec<PacketToken> {
    let mut num_start = None;
    let mut tokens = Vec::new();
    for (idx, &b) in input.as_bytes().iter().enumerate() {
        if b == b']' || b == b',' {
            if let Some(start) = num_start.take() {
                let num = input[start..idx].parse::<i32>().unwrap();
                tokens.push(PacketToken::Num(num));
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

struct Closer {
    count: usize,
    idx: Option<usize>,
    cursor: usize,
}

impl Closer {
    fn new() -> Self {
        Self {
            count: 0,
            idx: None,
            cursor: 0,
        }
    }

    fn get(&mut self, idx: usize, arr: &[PacketToken]) -> (Option<PacketToken>, bool) {
        self.advance(idx);

        if let Some(close) = self.next() {
            return (Some(close), true);
        }

        (arr.get(idx).copied(), false)
    }

    fn close(&mut self, idx: usize) {
        if let Some(_) = self.idx {
            self.count += 1
        } else {
            self.idx = Some(idx);
            self.count = 1;
        }
    }

    fn advance(&mut self, idx: usize) {
        self.cursor = idx;
    }
}

impl Iterator for Closer {
    type Item = PacketToken;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.idx {
            if self.cursor == idx {
                self.count -= 1;
                if self.count == 0 {
                    self.idx = None;
                }
                return Some(PacketToken::Close);
            }
        }

        None
    }
}

struct Packet {
    tokens: Vec<PacketToken>,
    special: bool,
}

impl std::str::FromStr for Packet {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = parse_packet(s);

        Ok(Packet {
            tokens,
            special: false,
        })
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if compare_packet(&self.tokens, &other.tokens) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl std::cmp::PartialEq for Packet {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl std::cmp::Eq for Packet {}

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
