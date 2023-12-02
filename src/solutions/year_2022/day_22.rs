use crate::{HashMap, HashSet};
use std::cell::RefCell;

pub fn part_one(input: &str) -> usize {
    let (map, movement) = input.split_once("\n\n").unwrap();
    let map = FlatMap::new(map);
    let movement = MovementCommands::new(movement);
    let mut player = Player::new(&map);

    for m in movement {
        player.make_move(m);
    }

    player.score()
}

pub fn part_two(input: &str) -> usize {
    solve_part_two::<50>(input, false)
}

fn solve_part_two<const N: usize>(input: &str, dump: bool) -> usize {
    let (map, movement) = input.split_once("\n\n").unwrap();
    let map = CubeMap::<N>::new(map);
    let movement = MovementCommands::new(movement);
    let mut player = Player::new(&map);

    for m in movement {
        player.make_move(m);

        if dump {
            map.dump();
        }
    }

    player.score()
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

struct Player<'a, M: Map> {
    position: M::Position,
    map: &'a M,
    facing: Facing,
}

impl<'a, M: Map> Player<'a, M> {
    fn new(map: &'a M) -> Self {
        Self {
            position: map.start(),
            map,
            facing: Facing::East,
        }
    }

    fn make_move(&mut self, movement: Movement) {
        match movement {
            Movement::Rotate(r) => self.facing = self.facing.rotate(r),
            Movement::Move(n) => {
                for _ in 0..n {
                    self.position = self.map.make_move(self.facing, self.position);
                }
            }
        }
    }

    fn score(&self) -> usize {
        let (facing, (x, y)) = self.map.normalize(self.facing, self.position);
        let facing = match facing {
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

trait Map {
    type Position: Copy;

    fn make_move(&self, facing: Facing, position: Self::Position) -> Self::Position;
    fn normalize(&self, facing: Facing, position: Self::Position) -> (Facing, (usize, usize));
    fn start(&self) -> Self::Position;
    fn dump(&self) {}
}

struct FlatMap {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    start: (isize, isize),
}

impl FlatMap {
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

        let mut start = None;

        for (y, l) in input.lines().enumerate() {
            for (x, b) in l.bytes().enumerate() {
                let idx = y * width + x;
                let cell = match b {
                    b'.' => {
                        if start.is_none() {
                            start = Some((x as isize, y as isize));
                        }
                        Cell::Open
                    }
                    b'#' => Cell::Wall,
                    b' ' => Cell::Void,
                    _ => unreachable!(),
                };
                cells[idx] = cell;
            }
        }

        Self {
            start: start.unwrap(),
            cells,
            width,
            height,
        }
    }

    fn get(&self, pos: (isize, isize)) -> Cell {
        let (_, (x, y)) = self.normalize(Facing::North, pos);

        let idx = y * self.width + x;
        self.cells.get(idx).copied().unwrap()
    }

    fn valid_point(&self, (x, y): (isize, isize)) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }
}

impl Map for FlatMap {
    type Position = (isize, isize);

    fn start(&self) -> (isize, isize) {
        self.start
    }

    fn make_move(&self, facing: Facing, position: (isize, isize)) -> (isize, isize) {
        let mut next_position = position;
        loop {
            next_position = facing.adjust(next_position);
            match self.get(next_position) {
                Cell::Open => {
                    return next_position;
                }
                Cell::Wall => {
                    return position;
                }
                Cell::Void => {}
            }
        }
    }

    fn normalize(&self, facing: Facing, (x, y): (isize, isize)) -> (Facing, (usize, usize)) {
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

        (facing, (x, y))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Orient {
    Zero(Side),
    Ninty(Side),
    OneEighty(Side),
    TwoSeventy(Side),
}

impl Orient {
    fn west(&self) -> Orient {
        match self {
            Orient::Zero(s) => s.west(),
            Orient::Ninty(s) => s.south().rotate(),
            Orient::OneEighty(s) => s.east().rotate().rotate(),
            Orient::TwoSeventy(s) => s.north().rotate().rotate().rotate(),
        }
    }

    fn east(&self) -> Orient {
        match self {
            Orient::Zero(s) => s.east(),
            Orient::Ninty(s) => s.north().rotate(),
            Orient::OneEighty(s) => s.west().rotate().rotate(),
            Orient::TwoSeventy(s) => s.south().rotate().rotate().rotate(),
        }
    }

    fn north(&self) -> Orient {
        match self {
            Orient::Zero(s) => s.north(),
            Orient::Ninty(s) => s.west().rotate(),
            Orient::OneEighty(s) => s.south().rotate().rotate(),
            Orient::TwoSeventy(s) => s.east().rotate().rotate().rotate(),
        }
    }

    fn south(&self) -> Orient {
        match self {
            Orient::Zero(s) => s.south(),
            Orient::Ninty(s) => s.east().rotate(),
            Orient::OneEighty(s) => s.north().rotate().rotate(),
            Orient::TwoSeventy(s) => s.west().rotate().rotate().rotate(),
        }
    }

    fn rotate(&self) -> Orient {
        match *self {
            Orient::Zero(s) => Orient::Ninty(s),
            Orient::Ninty(s) => Orient::OneEighty(s),
            Orient::OneEighty(s) => Orient::TwoSeventy(s),
            Orient::TwoSeventy(s) => Orient::Zero(s),
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Side {
    White,
    Yellow,
    Red,
    Orange,
    Green,
    Blue,
}

// Neutral Layout:
//  R W O
//    B
//    Y
//    G

impl Side {
    fn west(&self) -> Orient {
        match self {
            Side::White => Orient::Zero(Side::Red),
            Side::Yellow => Orient::OneEighty(Side::Red),
            Side::Red => Orient::OneEighty(Side::Yellow),
            Side::Orange => Orient::Zero(Side::White),
            Side::Green => Orient::Ninty(Side::Red),
            Side::Blue => Orient::TwoSeventy(Side::Red),
        }
    }

    fn east(&self) -> Orient {
        match self {
            Side::White => Orient::Zero(Side::Orange),
            Side::Yellow => Orient::OneEighty(Side::Orange),
            Side::Red => Orient::Zero(Side::White),
            Side::Orange => Orient::OneEighty(Side::Yellow),
            Side::Green => Orient::TwoSeventy(Side::Orange),
            Side::Blue => Orient::Ninty(Side::Orange),
        }
    }

    fn north(&self) -> Orient {
        match self {
            Side::White => Orient::Zero(Side::Green),
            Side::Yellow => Orient::Zero(Side::Blue),
            Side::Red => Orient::TwoSeventy(Side::Green),
            Side::Orange => Orient::Ninty(Side::Green),
            Side::Green => Orient::Zero(Side::Yellow),
            Side::Blue => Orient::Zero(Side::White),
        }
    }

    fn south(&self) -> Orient {
        match self {
            Side::White => Orient::Zero(Side::Blue),
            Side::Yellow => Orient::Zero(Side::Green),
            Side::Red => Orient::Ninty(Side::Blue),
            Side::Orange => Orient::TwoSeventy(Side::Blue),
            Side::Green => Orient::Zero(Side::White),
            Side::Blue => Orient::Zero(Side::Yellow),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CubePosition {
    position: (isize, isize),
    face: Orient,
}

impl CubePosition {
    fn rotate<const N: usize>(&self) -> CubePosition {
        let n = N as isize;

        CubePosition {
            position: ((n - 1) - (self.position.1 % n), (self.position.0 % n)),
            face: self.face.rotate(),
        }
    }
}

struct Face<const N: usize> {
    origin: (isize, isize),
    cells: [[Cell; N]; N],
}

pub struct CubeMap<const N: usize> {
    faces: HashMap<Orient, Face<N>>,
    visited: RefCell<HashMap<(usize, usize), Facing>>,
}

impl<const N: usize> CubeMap<N> {
    pub fn new(map: &str) -> Self {
        let map = FlatMap::new(map);

        let mut searched = HashSet::new();
        let mut search = Vec::new();
        search.push((Orient::Zero(Side::White), (0, 0)));
        searched.insert((0, 0));

        let mut faces = HashMap::new();
        let n = N as isize;

        while let Some((side, cursor)) = search.pop() {
            if map.get(cursor) != Cell::Void {
                let mut cells = [[Cell::Void; N]; N];
                for x in 0..N {
                    for y in 0..N {
                        let cursor = (cursor.0 + x as isize, cursor.1 + y as isize);
                        cells[x][y] = map.get(cursor);
                    }
                }

                faces.insert(
                    side,
                    Face {
                        origin: cursor,
                        cells,
                    },
                );
                if faces.len() == 6 {
                    break;
                }

                let west = (cursor.0 - n, cursor.1);
                let east = (cursor.0 + n, cursor.1);
                let north = (cursor.0, cursor.1 - n);
                let south = (cursor.0, cursor.1 + n);

                if map.valid_point(west)
                    && !searched.contains(&west)
                    && !faces.contains_key(&side.west())
                {
                    searched.insert(west);
                    search.push((side.west(), west));
                }

                if map.valid_point(east)
                    && !searched.contains(&east)
                    && !faces.contains_key(&side.east())
                {
                    searched.insert(east);
                    search.push((side.east(), east));
                }

                if map.valid_point(north)
                    && !searched.contains(&north)
                    && !faces.contains_key(&side.north())
                {
                    searched.insert(north);
                    search.push((side.north(), north));
                }

                if map.valid_point(south)
                    && !searched.contains(&south)
                    && !faces.contains_key(&side.south())
                {
                    searched.insert(south);
                    search.push((side.south(), south));
                }
            }

            if faces.len() == 0 && search.len() == 0 {
                let mut cursor = cursor;
                cursor.0 += N as isize;
                if cursor.0 >= map.width as isize {
                    cursor.0 = 0;
                    cursor.1 += N as isize;
                }

                search.push((Orient::Zero(Side::White), cursor));
                searched.insert(cursor);
            }
        }

        Self {
            faces,
            visited: RefCell::new(HashMap::new()),
        }
    }

    fn step(&self, facing: Facing, cube_position: CubePosition) -> CubePosition {
        let n = N as isize;
        let mut position = facing.adjust(cube_position.position);

        let face = if position.0 < 0 {
            position.0 = n - 1;
            cube_position.face.west()
        } else if position.0 >= n {
            position.0 = 0;
            cube_position.face.east()
        } else if position.1 < 0 {
            position.1 = n - 1;
            cube_position.face.north()
        } else if position.1 >= n {
            position.1 = 0;
            cube_position.face.south()
        } else {
            cube_position.face
        };

        CubePosition { position, face }
    }

    fn get(&self, mut position: CubePosition) -> Cell {
        for _ in 0..4 {
            if let Some(face) = self.faces.get(&position.face) {
                let x = position.position.0 as usize;
                let y = position.position.1 as usize;
                return face.cells[x][y];
            }
            position = position.rotate::<N>();
        }

        unreachable!()
    }
}

impl<const N: usize> Map for CubeMap<N> {
    type Position = CubePosition;

    fn make_move(&self, facing: Facing, position: Self::Position) -> Self::Position {
        let (norm_facing, norm) = self.normalize(facing, position);
        self.visited.borrow_mut().insert(norm, norm_facing);

        let next_position = self.step(facing, position);

        match self.get(next_position) {
            Cell::Open => return next_position,
            Cell::Wall => return position,
            Cell::Void => unreachable!(),
        }
    }

    fn normalize(
        &self,
        mut facing: Facing,
        mut position: Self::Position,
    ) -> (Facing, (usize, usize)) {
        for _ in 0..4 {
            if let Some(face) = self.faces.get(&position.face) {
                let x = position.position.0 as usize;
                let y = position.position.1 as usize;
                return (
                    facing,
                    (x + face.origin.0 as usize, y + face.origin.1 as usize),
                );
            }
            facing = facing.rotate(Rotate::Clockwise);
            position = position.rotate::<N>();
        }

        unreachable!()
    }

    fn start(&self) -> Self::Position {
        if let Some(white_face) = self.faces.get(&Orient::Zero(Side::White)) {
            for y in 0..N {
                for x in 0..N {
                    if white_face.cells[x][y] == Cell::Open {
                        return CubePosition {
                            position: (x as isize, y as isize),
                            face: Orient::Zero(Side::White),
                        };
                    }
                }
            }
        }

        unreachable!()
    }

    fn dump(&self) {
        let visited = self.visited.borrow();
        let mut display = vec![' '; 5 * N * 5 * N];
        for tile_y in 0..5 {
            for tile_x in 0..5 {
                for y in 0..N {
                    for x in 0..N {
                        for (_side, face) in self.faces.iter() {
                            if face.origin == (tile_x * N as isize, tile_y * N as isize) {
                                let pos = ((tile_x as usize) * N + x, tile_y as usize * N + y);
                                let cell = face.cells[x][y];

                                display[pos.1 * 5 * N + pos.0] =
                                    if let Some(facing) = visited.get(&pos) {
                                        match facing {
                                            Facing::North => '^',
                                            Facing::South => 'v',
                                            Facing::East => '>',
                                            Facing::West => '<',
                                        }
                                    } else {
                                        match cell {
                                            Cell::Open => '.',
                                            Cell::Wall => '#',
                                            Cell::Void => ' ',
                                        }
                                    };
                            }
                        }
                    }
                }
            }
        }

        for y in 0..5 * N {
            for x in 0..5 * N {
                let idx = y * 5 * N + x;
                print!("{}", display[idx]);
            }
            println!();
        }
    }
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
    assert_eq!(5031, solve_part_two::<4>(input, true));

    let input = r#"    ........
    ........
    ........
    ........
    ....
    ....
    ....
    ....
........
........
........
........
....
....
....
....

RR4RRRR4RRRR4RRRR4RRRR4RRRR4RRRR4"#;

    solve_part_two::<4>(input, true);
}
