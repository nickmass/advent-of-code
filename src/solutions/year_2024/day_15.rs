pub fn part_one(input: &str) -> usize {
    let (map, moves) = input.trim().split_once("\n\n").unwrap();
    let mut map = Map::new(map);
    let moves = Moves::new(moves);

    for dir in moves {
        map.make_move(dir);
    }

    map.score()
}

pub fn part_two(input: &str) -> u64 {
    let (map, moves) = input.trim().split_once("\n\n").unwrap();
    let mut map = WideMap::new(map);
    let moves = Moves::new(moves);

    for dir in moves {
        map.make_move(dir);
    }

    map.score()
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    bot: (i32, i32),
}

impl Map {
    fn new(input: &str) -> Self {
        let input = input.trim().as_bytes();
        let mut cells = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 0;
        let mut bot = None;

        for &b in input {
            let cell = match b {
                b'\n' => {
                    height += 1;
                    width = 0;
                    continue;
                }
                b'#' => Cell::Wall,
                b'O' => Cell::Box,
                b'.' => Cell::Empty,
                b'@' => {
                    bot = Some((width as i32, height as i32));
                    Cell::Empty
                }
                _ => unreachable!(),
            };
            cells.push(cell);
            width += 1;
        }

        height += 1;

        Self {
            cells,
            width,
            height,
            bot: bot.unwrap(),
        }
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
        let ind = y * self.width + x;

        self.cells.get(ind).copied()
    }

    fn set(&mut self, (x, y): (i32, i32), cell: Cell) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }
        let ind = y * self.width + x;

        if let Some(c) = self.cells.get_mut(ind) {
            *c = cell
        }
    }

    fn nudge(&mut self, p: (i32, i32), dir: Direction) -> bool {
        let target = dir.apply(p);

        let space = match self.get(target) {
            Some(Cell::Empty) => true,
            Some(Cell::Wall) => false,
            Some(Cell::Box) => self.nudge(target, dir),
            None => false,
        };

        if space {
            if let Some(Cell::Box) = self.get(p) {
                self.set(target, Cell::Box);
                self.set(p, Cell::Empty);
            };
        }

        space
    }

    fn make_move(&mut self, dir: Direction) {
        let bot = self.bot;
        if self.nudge(bot, dir) {
            self.bot = dir.apply(bot);
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Cell::Box) = self.get((x as i32, y as i32)) {
                    score += 100 * y + x;
                }
            }
        }

        score
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Box,
}

struct WideMap {
    walls: Vec<bool>,
    boxes: Vec<Box>,
    width: usize,
    height: usize,
    bot: (i32, i32),
}

impl WideMap {
    fn new(input: &str) -> Self {
        let input = input.trim().as_bytes();
        let mut walls = Vec::with_capacity(input.len() * 2);
        let mut width = 0;
        let mut height = 0;
        let mut bot = None;
        let mut boxes = Vec::new();

        for &b in input {
            let cell = match b {
                b'\n' => {
                    height += 1;
                    width = 0;
                    continue;
                }
                b'#' => true,
                b'O' => {
                    boxes.push(Box {
                        x: width as i32,
                        y: height as i32,
                    });
                    false
                }
                b'.' => false,
                b'@' => {
                    bot = Some((width as i32, height as i32));
                    false
                }
                _ => unreachable!(),
            };
            walls.push(cell);
            walls.push(cell);
            width += 2;
        }

        height += 1;

        Self {
            walls,
            boxes,
            width,
            height,
            bot: bot.unwrap(),
        }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<bool> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        let ind = y * self.width + x;

        self.walls.get(ind).copied()
    }

    fn can_nudge(&self, p: (i32, i32), dir: Direction) -> bool {
        if self.get(p).unwrap_or(true) {
            false
        } else {
            let mut nudge = true;
            for b in &self.boxes {
                if b.collides(p) {
                    let (l, r) = b.neighbors(dir);
                    if let Some(l) = l {
                        nudge &= self.can_nudge(l, dir);
                    }
                    if let Some(r) = r {
                        nudge &= self.can_nudge(r, dir);
                    }
                    break;
                }
            }

            nudge
        }
    }

    fn nudge(&mut self, p: (i32, i32), dir: Direction) {
        for b in 0..self.boxes.len() {
            if self.boxes[b].collides(p) {
                let (l, r) = self.boxes[b].neighbors(dir);
                if let Some(l) = l {
                    self.nudge(l, dir);
                }
                if let Some(r) = r {
                    self.nudge(r, dir);
                }
                self.boxes[b].make_move(dir);
                break;
            }
        }
    }

    fn make_move(&mut self, dir: Direction) {
        let bot = dir.apply(self.bot);
        if self.can_nudge(bot, dir) {
            self.nudge(bot, dir);
            self.bot = bot;
        }
    }

    fn score(&self) -> u64 {
        let mut score = 0;
        for b in &self.boxes {
            score += 100 * (b.y as u64) + (b.x as u64);
        }

        score
    }
}

#[derive(Debug, Copy, Clone)]
struct Box {
    x: i32,
    y: i32,
}

impl Box {
    fn make_move(&mut self, dir: Direction) {
        let p = (self.x, self.y);
        let (x, y) = dir.apply(p);
        self.x = x;
        self.y = y;
    }

    fn neighbors(self, dir: Direction) -> (Option<(i32, i32)>, Option<(i32, i32)>) {
        let l = (self.x, self.y);
        let r = (self.x + 1, self.y);
        match dir {
            Direction::Up | Direction::Down => (Some(dir.apply(l)), Some(dir.apply(r))),
            Direction::Left => (Some(dir.apply(l)), None),
            Direction::Right => (None, Some(dir.apply(r))),
        }
    }

    fn collides(&self, (x, y): (i32, i32)) -> bool {
        (self.x == x && self.y == y) || (self.x + 1 == x && self.y == y)
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

struct Moves<'a> {
    input: &'a [u8],
    cursor: usize,
}

impl<'a> Moves<'a> {
    fn new(input: &'a str) -> Self {
        let input = input.trim().as_bytes();
        Self { input, cursor: 0 }
    }
}

impl<'a> Iterator for Moves<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let v = self.input.get(self.cursor)?;
            self.cursor += 1;
            let dir = match &v {
                b'^' => Direction::Up,
                b'v' => Direction::Down,
                b'<' => Direction::Left,
                b'>' => Direction::Right,
                b'\n' => continue,
                _ => unreachable!(),
            };

            return Some(dir);
        }
    }
}

#[test]
fn test() {
    let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    assert_eq!(10092, part_one(input));
    assert_eq!(9021, part_two(input));
}
