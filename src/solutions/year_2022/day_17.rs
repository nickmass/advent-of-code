use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    solve(input, 2022)
}

pub fn part_two(input: &str) -> usize {
    solve(input, 1000000000000)
}

fn solve(input: &str, rounds: usize) -> usize {
    let mut grid = vec![0u8; 2048 * 4 * 4];

    let mut moves = input.trim().bytes().enumerate().cycle();
    let blocks = Blocks::default();

    let mut drop_state = HashMap::new();
    let mut skip_height = None;

    let mut height = 0;
    let mut round = 0;
    let mut block_y;

    for (mut block, block_height) in blocks {
        round += 1;
        block_y = height + 3;

        loop {
            if block_y + 4 > grid.len() {
                grid.resize(grid.len() * 2, 0);
            }

            let move_area = read_grid(&grid, block_y);
            let input_idx = match moves.next() {
                Some((idx, b'<')) => {
                    if (block & LEFT_LIMITS) == 0 && ((block << 1) & move_area) == 0 {
                        block <<= 1;
                    }
                    idx
                }
                Some((idx, b'>')) => {
                    if (block & RIGHT_LIMITS) == 0 && ((block >> 1) & move_area) == 0 {
                        block >>= 1;
                    }
                    idx
                }
                _ => unreachable!(),
            };

            let freeze = block_y == 0 || (read_grid(&grid, block_y - 1) & block) != 0;

            if freeze {
                if skip_height.is_none() {
                    if let Some((repeat_round, repeat_height)) =
                        drop_state.insert((move_area, block, input_idx), (round, height))
                    {
                        let remaining_rounds = rounds - round;
                        let cycle_length = round - repeat_round;
                        let cycle_height = height - repeat_height;
                        let skip_count = remaining_rounds / cycle_length;
                        round += skip_count * cycle_length;
                        skip_height = Some(skip_count * cycle_height);
                    }
                }

                write_grid(&mut grid, block_y, block);
                height = height.max(block_y + block_height);
                break;
            } else {
                block_y -= 1;
            }
        }

        if round == rounds {
            break;
        }
    }

    height + skip_height.unwrap_or(0)
}

fn read_grid(grid: &[u8], y_offset: usize) -> u32 {
    u32::from_le_bytes(grid[y_offset..y_offset + 4].try_into().unwrap())
}

fn write_grid(grid: &mut [u8], y_offset: usize, block: u32) {
    let move_area = read_grid(grid, y_offset);

    let grid_bytes = (move_area | block).to_le_bytes();
    grid[y_offset] = grid_bytes[0];
    grid[y_offset + 1] = grid_bytes[1];
    grid[y_offset + 2] = grid_bytes[2];
    grid[y_offset + 3] = grid_bytes[3];
}

#[derive(Debug, Default, Clone)]
struct Blocks {
    index: usize,
}

impl Iterator for Blocks {
    type Item = (u32, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let v = match self.index % 5 {
            0 => Some((B_LINE, 1)),
            1 => Some((B_PLUS, 3)),
            2 => Some((B_L, 3)),
            3 => Some((B_BAR, 4)),
            4 => Some((B_BLOCK, 2)),
            _ => unreachable!(),
        };
        self.index += 1;

        v
    }
}

const LEFT_LIMITS: u32 = 0b01000000010000000100000001000000;
const RIGHT_LIMITS: u32 = 0b00000001000000010000000100000001;

const B_PLUS: u32 = u32::from_be_bytes([0b00000000, 0b00001000, 0b00011100, 0b00001000]);
const B_L: u32 = u32::from_be_bytes([0b00000000, 0b00000100, 0b00000100, 0b00011100]);
const B_BAR: u32 = u32::from_be_bytes([0b00010000, 0b00010000, 0b00010000, 0b00010000]);
const B_BLOCK: u32 = u32::from_be_bytes([0b00000000, 0b00000000, 0b00011000, 0b00011000]);
const B_LINE: u32 = u32::from_be_bytes([0b00000000, 0b00000000, 0b00000000, 0b00011110]);

#[test]
fn test() {
    let input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    assert_eq!(3068, part_one(input));
    assert_eq!(1514285714288, part_two(input));
}
