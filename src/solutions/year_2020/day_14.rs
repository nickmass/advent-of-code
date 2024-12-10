use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    let lines = input.lines();

    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut mem = HashMap::new();

    for line in lines {
        let mut split = line
            .split(&['[', ']', '=', ' '][..])
            .filter(|w| !w.trim().is_empty());
        let op = split.next().unwrap();

        match op {
            "mask" => {
                and_mask = 0;
                or_mask = 0;
                let mask = split.last().unwrap();
                for c in mask.chars() {
                    and_mask <<= 1;
                    or_mask <<= 1;
                    match c {
                        'X' => {
                            and_mask |= 1;
                            or_mask |= 0;
                        }
                        '1' => {
                            and_mask |= 1;
                            or_mask |= 1;
                        }
                        '0' => {
                            and_mask |= 0;
                            or_mask |= 0;
                        }
                        _ => {
                            unreachable!("bad input")
                        }
                    }
                }
            }
            "mem" => {
                let addr: usize = split.next().unwrap().parse().unwrap();
                let mut val: u64 = split.next().unwrap().parse().unwrap();

                val &= and_mask;
                val |= or_mask;

                mem.insert(addr, val);
            }
            _ => (),
        }
    }

    mem.values().sum()
}

pub fn part_two(input: &str) -> u64 {
    let lines = input.lines();

    let mut mem = HashMap::new();

    let mut mask = None;
    for line in lines {
        let mut split = line
            .split(&['[', ']', '=', ' '][..])
            .filter(|w| !w.trim().is_empty());

        let op = split.next().unwrap();
        match op {
            "mask" => {
                let mask_str = split.last().unwrap();
                mask = Some(Mask::new(mask_str));
            }
            "mem" => {
                let addr: u64 = split.next().unwrap().parse().unwrap();
                let val: u64 = split.next().unwrap().parse().unwrap();

                if let Some(mask) = &mask {
                    for addr in mask.addrs(addr) {
                        mem.insert(addr, val);
                    }
                }
            }

            _ => (),
        }
    }

    mem.values().sum()
}

struct Mask {
    or_mask: u64,
    and_mask: u64,
    floating_masks: Vec<u64>,
}

impl Mask {
    fn new(mask: &str) -> Self {
        let mut or_mask = 0;
        let mut floating_mask: u64 = 0;
        for c in mask.chars() {
            or_mask <<= 1;
            floating_mask <<= 1;

            match c {
                'X' => {
                    floating_mask |= 1;
                }
                '1' => {
                    or_mask |= 1;
                }
                '0' => {
                    or_mask |= 0;
                }
                _ => {
                    unreachable!("bad input")
                }
            }
        }
        let permutations = 2u64.pow(floating_mask.count_ones());
        let limit = 36.min(64 - floating_mask.leading_zeros()) as u64;

        let mut floating_masks = Vec::with_capacity(permutations as usize);

        for n in 0..permutations {
            let mut mask = 0;
            let mut bit_count = 0;
            let mut floating_bits = floating_mask;
            for bit_position in 0..limit {
                if floating_bits & 1 == 1 {
                    let new_bit = ((n >> bit_count) & 1) << bit_position;
                    mask |= new_bit;
                    bit_count += 1;
                }
                floating_bits >>= 1;
            }

            floating_masks.push(mask);
        }

        Self {
            or_mask,
            and_mask: !floating_mask,
            floating_masks,
        }
    }

    fn addrs(&self, mut addr: u64) -> impl Iterator<Item = u64> + '_ {
        addr &= self.and_mask;
        addr |= self.or_mask;
        self.floating_masks.iter().map(move |mask| addr | *mask)
    }
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

    run_a(i, 165);

    let i = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
    run_b(i, 208);
}
