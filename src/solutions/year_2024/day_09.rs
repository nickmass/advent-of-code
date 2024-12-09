pub fn part_one(input: &str) -> u64 {
    let file_list = input.trim().chars().filter_map(|n| n.to_digit(10));

    let mut total_size = 0;
    let mut index = 0;
    let mut blocks = Vec::with_capacity(input.len() / 2);
    for (i, block) in file_list.enumerate() {
        let block = block as u64;
        if i % 2 == 0 {
            total_size += block;
            blocks.push(Block {
                index,
                id: blocks.len() as u64,
                len: block,
            });
        }
        index += block;
    }

    let mut index = 0;
    let mut front = 0;
    let mut back = blocks.len() - 1;
    let mut checksum = 0;

    while index < total_size {
        let b = loop {
            if blocks[front].index > index {
                if blocks[back].len > 0 {
                    break back;
                } else {
                    back -= 1;
                }
            } else {
                if blocks[front].len > 0 {
                    break front;
                } else {
                    front += 1;
                }
            }
        };

        checksum += blocks[b].id * index;
        blocks[b].len -= 1;
        index += 1;
    }

    checksum
}

pub fn part_two(input: &str) -> u64 {
    let file_list = input.trim().chars().filter_map(|n| n.to_digit(10));

    let mut index = 0;
    let mut blocks = Vec::with_capacity(input.len() / 2);
    let mut empty_blocks = Vec::with_capacity(input.len() / 2);
    for (i, block) in file_list.enumerate() {
        let block = block as u64;
        if i % 2 == 0 {
            blocks.push(Block {
                index,
                id: blocks.len() as u64,
                len: block,
            });
        } else {
            empty_blocks.push(EmptyBlock { index, len: block });
        }
        index += block;
    }

    let mut checksum = 0;

    for b in blocks.iter_mut().rev() {
        for e in empty_blocks.iter_mut() {
            if e.index >= b.index {
                break;
            }

            if e.len >= b.len {
                e.len -= b.len;
                for _ in 0..b.len {
                    checksum += b.id * e.index;
                    e.index += 1;
                }
                b.len = 0;
                break;
            }
        }
    }

    for b in blocks.iter() {
        for i in 0..b.len {
            checksum += b.id * (b.index + i);
        }
    }

    checksum
}

struct Block {
    index: u64,
    id: u64,
    len: u64,
}

struct EmptyBlock {
    index: u64,
    len: u64,
}

#[test]
fn test() {
    let input = r#"2333133121414131402"#;

    assert_eq!(1928, part_one(input));
    assert_eq!(2858, part_two(input));
}
