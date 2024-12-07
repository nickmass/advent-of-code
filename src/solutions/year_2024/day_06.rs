use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut map = Map::new(input);
    let mut visited = HashSet::new();

    loop {
        visited.insert(map.position());
        if !map.make_move() {
            break;
        }
    }

    visited.len()
}

pub fn part_two(input: &str) -> u64 {
    let mut map = Map::new(input);
    let mut visited = HashSet::new();

    loop {
        visited.insert(map.position());
        if !map.make_move() {
            break;
        }
    }

    visited.remove(&map.start.position);

    let mut looping = 0;
    let mut player_states = HashSet::new();

    for possible in visited {
        map.set(possible, Cell::Occupied);

        player_states.clear();
        map.reset_player();

        loop {
            player_states.insert(map.player);
            if !map.make_move() {
                break;
            }
            if player_states.contains(&map.player) {
                looping += 1;
                break;
            }
        }

        map.set(possible, Cell::Empty);
    }

    looping
}

struct Map {
    cells: Vec<Cell>,
    start: Player,
    player: Player,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let input = input.trim().as_bytes();
        let mut width = 0;
        let mut height = 0;
        let mut player = None;
        let mut cells = Vec::with_capacity(input.len());

        for b in input {
            width += 1;
            match b {
                b'.' => cells.push(Cell::Empty),
                b'#' => cells.push(Cell::Occupied),
                b'\n' => {
                    height += 1;
                    width = 0;
                }
                b'^' | b'<' | b'>' | b'v' => {
                    cells.push(Cell::Empty);
                    let Ok(direction) = Direction::try_from(*b) else {
                        unreachable!("only valid directions possible")
                    };

                    player = Some(Player {
                        position: (width - 1, height),
                        direction,
                    });
                }
                _ => unreachable!("invalid input"),
            }
        }

        height += 1;
        let player = player.expect("valid player in input");

        Self {
            cells,
            width: width as usize,
            height: height as usize,
            start: player,
            player,
        }
    }

    fn reset_player(&mut self) {
        self.player = self.start;
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = y * self.width + x;
        self.cells.get(idx).copied()
    }

    fn get_mut(&mut self, (x, y): (i32, i32)) -> Option<&mut Cell> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = y * self.width + x;
        self.cells.get_mut(idx)
    }

    fn set(&mut self, pos: (i32, i32), new_cell: Cell) {
        let Some(cell) = self.get_mut(pos) else {
            return;
        };
        *cell = new_cell;
    }

    fn make_move(&mut self) -> bool {
        let new_position = self.player.next_move();

        match self.get(new_position) {
            Some(Cell::Empty) => {
                self.player.position = new_position;
                true
            }
            Some(Cell::Occupied) => {
                self.player.direction = self.player.direction.turn_right();
                true
            }
            None => false,
        }
    }

    fn position(&self) -> (i32, i32) {
        self.player.position
    }
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Occupied,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Player {
    position: (i32, i32),
    direction: Direction,
}

impl Player {
    fn next_move(&self) -> (i32, i32) {
        let offset = self.direction.offset();
        (offset.0 + self.position.0, offset.1 + self.position.1)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn offset(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct DirectionFromU8Err;

impl TryFrom<u8> for Direction {
    type Error = DirectionFromU8Err;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Direction::North),
            b'>' => Ok(Direction::East),
            b'v' => Ok(Direction::South),
            b'<' => Ok(Direction::West),
            _ => Err(DirectionFromU8Err),
        }
    }
}

#[test]
fn test() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    assert_eq!(41, part_one(input));
    assert_eq!(6, part_two(input));
}
