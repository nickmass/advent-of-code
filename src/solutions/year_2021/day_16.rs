pub fn part_one(input: &str) -> u64 {
    let packets = parse_packets(input);

    packets.into_iter().map(|p| p.version()).sum()
}

pub fn part_two(input: &str) -> u64 {
    let packets = parse_packets(input);

    packets[0].eval(&packets)
}

fn parse_packets(input: &str) -> Vec<Packet> {
    let mut reader = BitReader::new(input);
    let mut packets = Vec::new();
    let mut parse_stack: Vec<(usize, Argument)> = Vec::new();

    while packets.len() == 0 || parse_stack.len() > 0 {
        let mut size = 6;
        let version = reader.take_bits(3).unwrap();
        let type_id = reader.take_bits(3).unwrap();

        let (args, packet) = match type_id {
            4 => {
                let mut reading_literal = true;
                let mut value = 0;
                while reading_literal {
                    reading_literal = reader.take_bits(1).unwrap() == 1;
                    let bits = reader.take_bits(4).unwrap();
                    value <<= 4;
                    value |= bits;
                    size += 5;
                }

                (None, Packet::Literal(version, value))
            }
            _ => {
                let i = reader.take_bits(1).unwrap() == 1;
                size += 1;
                let (args, vec) = if i {
                    let args = reader.take_bits(11).unwrap();
                    size += 11;
                    (Argument::Packets(args), Vec::with_capacity(args as usize))
                } else {
                    let bits = reader.take_bits(15).unwrap();
                    size += 15;
                    (Argument::Bits(bits), Vec::new())
                };

                let packet = match type_id {
                    0 => Packet::Sum(version, vec),
                    1 => Packet::Product(version, vec),
                    2 => Packet::Min(version, vec),
                    3 => Packet::Max(version, vec),
                    5 => Packet::Gt(version, vec),
                    6 => Packet::Lt(version, vec),
                    7 => Packet::Eq(version, vec),
                    _ => unreachable!(),
                };

                (Some(args), packet)
            }
        };
        let id = packets.len();
        packets.push(packet);

        if let Some((parent_id, args_type)) = parse_stack.last_mut() {
            packets
                .get_mut(*parent_id)
                .map(|parent| parent.push_arg(id));

            match args_type {
                Argument::Packets(packets) => {
                    *packets -= 1;
                }
                _ => (),
            }
        }

        for arg in parse_stack.iter_mut() {
            match arg.1 {
                Argument::Bits(ref mut bits) => *bits -= size,
                _ => (),
            }
        }

        while let Some(popped) = parse_stack.pop() {
            match popped.1 {
                Argument::Bits(0) => continue,
                Argument::Packets(0) => continue,
                _ => {
                    parse_stack.push(popped);
                    break;
                }
            }
        }

        if let Some(args) = args {
            parse_stack.push((id, args));
        }
    }

    packets
}

enum Packet {
    Literal(u64, u64),
    Sum(u64, Vec<usize>),
    Product(u64, Vec<usize>),
    Min(u64, Vec<usize>),
    Max(u64, Vec<usize>),
    Gt(u64, Vec<usize>),
    Lt(u64, Vec<usize>),
    Eq(u64, Vec<usize>),
}

impl Packet {
    fn push_arg(&mut self, arg: usize) {
        match self {
            Packet::Sum(_, args) => args.push(arg),
            Packet::Product(_, args) => args.push(arg),
            Packet::Min(_, args) => args.push(arg),
            Packet::Max(_, args) => args.push(arg),
            Packet::Gt(_, args) => args.push(arg),
            Packet::Lt(_, args) => args.push(arg),
            Packet::Eq(_, args) => args.push(arg),
            _ => (),
        }
    }

    fn version(&self) -> u64 {
        match self {
            Packet::Literal(version, _) => *version,
            Packet::Sum(version, _) => *version,
            Packet::Product(version, _) => *version,
            Packet::Min(version, _) => *version,
            Packet::Max(version, _) => *version,
            Packet::Gt(version, _) => *version,
            Packet::Lt(version, _) => *version,
            Packet::Eq(version, _) => *version,
        }
    }

    fn eval(&self, packets: &[Packet]) -> u64 {
        match self {
            Packet::Literal(_, v) => *v,
            Packet::Sum(_, args) => args.iter().map(|id| packets[*id].eval(&packets)).sum(),
            Packet::Product(_, args) => args.iter().map(|id| packets[*id].eval(&packets)).product(),
            Packet::Min(_, args) => args
                .iter()
                .map(|id| packets[*id].eval(&packets))
                .min()
                .unwrap(),
            Packet::Max(_, args) => args
                .iter()
                .map(|id| packets[*id].eval(&packets))
                .max()
                .unwrap(),
            Packet::Gt(_, args) => {
                if packets[args[0]].eval(&packets) > packets[args[1]].eval(&packets) {
                    1
                } else {
                    0
                }
            }
            Packet::Lt(_, args) => {
                if packets[args[0]].eval(&packets) < packets[args[1]].eval(&packets) {
                    1
                } else {
                    0
                }
            }
            Packet::Eq(_, args) => {
                if packets[args[0]].eval(&packets) == packets[args[1]].eval(&packets) {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Argument {
    Bits(u64),
    Packets(u64),
}

struct BitReader {
    bytes: Vec<u8>,
    cursor: usize,
    fine_cursor: usize,
    next_byte: u8,
}

impl BitReader {
    fn new(input: &str) -> BitReader {
        let bytes: Vec<u8> = input
            .as_bytes()
            .chunks(2)
            .filter_map(|b| to_byte(b))
            .collect();

        let cursor = 0;
        let next_byte = bytes[cursor];

        BitReader {
            bytes,
            cursor,
            fine_cursor: 0,
            next_byte,
        }
    }

    fn take_bits(&mut self, mut count: usize) -> Option<u64> {
        let mut result = 0;
        while count != 0 {
            result <<= 1;
            if self.fine_cursor >= 8 {
                self.cursor += 1;
                if self.cursor >= self.bytes.len() {
                    return None;
                }
                self.fine_cursor = 0;
                self.next_byte = self.bytes[self.cursor];
            }

            result |= ((((self.next_byte as u64) << self.fine_cursor) & 0x80) >> 7) & 1;

            self.fine_cursor += 1;
            count -= 1;
        }

        Some(result)
    }
}

fn to_digit(c: u8) -> Option<u8> {
    let d = match c {
        b'0' => 0x0,
        b'1' => 0x1,
        b'2' => 0x2,
        b'3' => 0x3,
        b'4' => 0x4,
        b'5' => 0x5,
        b'6' => 0x6,
        b'7' => 0x7,
        b'8' => 0x8,
        b'9' => 0x9,
        b'A' => 0xa,
        b'B' => 0xb,
        b'C' => 0xc,
        b'D' => 0xd,
        b'E' => 0xe,
        b'F' => 0xf,
        b'a' => 0xa,
        b'b' => 0xb,
        b'c' => 0xc,
        b'd' => 0xd,
        b'e' => 0xe,
        b'f' => 0xf,
        _ => return None,
    };

    Some(d)
}

fn to_byte(b: &[u8]) -> Option<u8> {
    match b {
        [h, l] => to_digit(*h).zip(to_digit(*l)).map(|(h, l)| (h << 4) | l),
        [l] => to_digit(*l),
        _ => None,
    }
}

#[test]
fn test() {
    let input = r#"A0016C880162017C3686B18A3D4780"#;
    assert_eq!(31, part_one(input));

    let input = "9C0141080250320F1802104A08";
    assert_eq!(1, part_two(input));
}
