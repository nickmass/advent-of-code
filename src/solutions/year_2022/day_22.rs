pub fn part_one(input: &str) -> usize {
    let (map, movement) = input.split_once("\n\n").unwrap();
    let map = Map::new(map);
    let movement = MovementCommands::new(movement);
    let mut player = Player::new(&map);

    for m in movement {
        player.make_move(m);
    }

    player.score()
}

pub fn part_two(input: &str) -> u64 {
    let _lines = input.trim().lines();

    0
}

struct Player<'a> {
    position: (isize, isize),
    map: &'a Map,
    facing: Facing,
}

impl<'a> Player<'a> {
    fn new(map: &'a Map) -> Self {
        let y = 0;
        let mut x = 0;
        for i in 0..map.width {
            match map.get((i as isize, y)) {
                Cell::Open => {
                    x = i as isize;
                    break;
                }
                _ => continue,
            }
        }

        let position = (x, y);

        Self {
            position,
            map,
            facing: Facing::East,
        }
    }

    fn make_move(&mut self, movement: Movement) {
        match movement {
            Movement::Rotate(r) => self.facing = self.facing.rotate(r),
            Movement::Move(mut n) => {
                let mut good_pos = self.position;
                while n != 0 {
                    let pos = self.facing.adjust(self.position);
                    match self.map.get(pos) {
                        Cell::Open => {
                            good_pos = pos;
                            n -= 1
                        }
                        Cell::Wall => break,
                        Cell::Void => (),
                    }
                    self.position = pos;
                }
                self.position = good_pos;
            }
        }
    }

    fn score(&self) -> usize {
        let (x, y) = self.map.normalize(self.position);
        let facing = match self.facing {
            Facing::North => 3,
            Facing::South => 1,
            Facing::East => 0,
            Facing::West => 2,
        };

        let column = x + 1;
        let row = y + 1;

        row * 1000 + column * 4 + facing
    }
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        for l in input.lines() {
            if l.len() > width {
                width = l.len();
            }
            height += 1;
        }

        let mut cells = vec![Cell::Void; height * width];

        for (y, l) in input.lines().enumerate() {
            for (x, b) in l.bytes().enumerate() {
                let idx = y * width + x;
                let cell = match b {
                    b'.' => Cell::Open,
                    b'#' => Cell::Wall,
                    b' ' => Cell::Void,
                    _ => unreachable!(),
                };
                cells[idx] = cell;
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, pos: (isize, isize)) -> Cell {
        let (x, y) = self.normalize(pos);

        let idx = y * self.width + x;
        self.cells.get(idx).copied().unwrap()
    }

    fn normalize(&self, (x, y): (isize, isize)) -> (usize, usize) {
        let x = if x < 0 {
            let x = x.abs() as usize;
            (self.width - (x % self.width)) % self.width
        } else {
            let x = x as usize;
            x % self.width
        };

        let y = if y < 0 {
            let y = y.abs() as usize;
            (self.height - (y % self.height)) % self.height
        } else {
            let y = y as usize;
            y % self.height
        };

        (x, y)
    }
}

struct MovementCommands<'a> {
    input: &'a str,
}

impl<'a> MovementCommands<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.trim(),
        }
    }
}

impl Iterator for MovementCommands<'_> {
    type Item = Movement;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.get(0..1) {
            Some("R") | Some("L") => {
                let (c, input) = self.input.split_at(1);
                self.input = input;

                match c {
                    "R" => Some(Movement::Rotate(Rotate::Clockwise)),
                    "L" => Some(Movement::Rotate(Rotate::CounterClockwise)),
                    _ => unreachable!(),
                }
            }
            Some(_) => {
                let idx = self
                    .input
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(self.input.len());

                let (n, input) = self.input.split_at(idx);
                self.input = input;

                Some(Movement::Move(n.parse().ok()?))
            }
            None => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Open,
    Wall,
    Void,
}

#[derive(Debug, Copy, Clone)]
enum Rotate {
    CounterClockwise,
    Clockwise,
}

#[derive(Debug, Copy, Clone)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn rotate(self, dir: Rotate) -> Self {
        match (self, dir) {
            (Facing::North, Rotate::CounterClockwise) => Facing::West,
            (Facing::North, Rotate::Clockwise) => Facing::East,
            (Facing::South, Rotate::CounterClockwise) => Facing::East,
            (Facing::South, Rotate::Clockwise) => Facing::West,
            (Facing::East, Rotate::CounterClockwise) => Facing::North,
            (Facing::East, Rotate::Clockwise) => Facing::South,
            (Facing::West, Rotate::CounterClockwise) => Facing::South,
            (Facing::West, Rotate::Clockwise) => Facing::North,
        }
    }

    fn adjust(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Facing::North => (x, y - 1),
            Facing::South => (x, y + 1),
            Facing::East => (x + 1, y),
            Facing::West => (x - 1, y),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Movement {
    Rotate(Rotate),
    Move(u32),
}

#[test]
fn test() {
    let input = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

    assert_eq!(6032, part_one(input));
    assert_eq!(5031, part_two(input));
}
