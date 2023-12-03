use super::Point2;
use crate::HashMap;

type Point = Point2<i32>;

pub fn part_one(input: &str) -> u32 {
    let map = Map::new(input);

    map.route()
}

pub fn part_two(_input: &str) -> u64 {
    0
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Cell {
    Wall,
    Open,
    Door(Door),
    Key(Key),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Door(u32);
impl Door {
    fn from_byte(b: u8) -> Self {
        Door(1 << (b - b'A') as u32)
    }
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Key(u32);
impl Key {
    fn from_byte(b: u8) -> Self {
        Key(1 << (b - b'a') as u32)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Keyring(u32);
impl Keyring {
    fn empty() -> Self {
        Keyring(0)
    }

    fn unlocks(&self, door: Door) -> bool {
        self.0 | door.0 == self.0
    }

    fn add(&mut self, key: Key) {
        self.0 |= key.0;
    }

    fn contains(&self, key: Key) -> bool {
        self.0 | key.0 == self.0
    }

    fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Move {
    Point(Point),
    Key(Key, Point),
}

impl Move {
    fn point(&self) -> Point {
        match self {
            Move::Point(p) => *p,
            Move::Key(_, p) => *p,
        }
    }
}

struct Map {
    cells: Vec<Cell>,
    keys: HashMap<Key, Point>,
    start: Point,
    width: i32,
    height: i32,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut start = None;
        let mut width = None;
        let mut cells = Vec::new();
        let mut _doors = HashMap::new();
        let mut keys = HashMap::new();

        for b in input.trim().bytes() {
            match b {
                b'#' => cells.push(Cell::Wall),
                b'.' => cells.push(Cell::Open),
                b'@' => {
                    start = Some(Point::new(x, y));
                    cells.push(Cell::Open);
                }
                b'a'..=b'z' => {
                    let key = Key::from_byte(b);
                    keys.insert(key, Point::new(x, y));
                    cells.push(Cell::Key(Key::from_byte(b)))
                }
                b'A'..=b'Z' => {
                    let door = Door::from_byte(b);
                    _doors.insert(door, Point::new(x, y));
                    cells.push(Cell::Door(door));
                }
                b'\n' => {
                    if width.is_none() {
                        width = Some(x);
                    }
                    y += 1;
                    x = 0;
                    continue;
                }
                _ => unreachable!(),
            }
            x += 1;
        }

        let height = y + 1;
        let width = width.unwrap();
        let start = start.unwrap();

        Self {
            cells,
            keys,
            start,
            width,
            height,
        }
    }

    fn get(&self, point: Point) -> Option<Cell> {
        if point.x < 0 || point.x >= self.width || point.y < 0 || point.y >= self.height {
            None
        } else {
            let idx = point.y * self.width + point.x;
            Some(self.cells[idx as usize])
        }
    }

    fn moves<'a>(&'a self, point: Point, keyring: Keyring) -> impl Iterator<Item = Move> + 'a {
        let mut i = 0;
        std::iter::from_fn(move || loop {
            let p = match i {
                0 => Point::new(point.x - 1, point.y),
                1 => Point::new(point.x + 1, point.y),
                2 => Point::new(point.x, point.y - 1),
                3 => Point::new(point.x, point.y + 1),
                _ => return None,
            };
            i += 1;

            let cell = self.get(p)?;

            return match cell {
                Cell::Wall => continue,
                Cell::Open => Some(Move::Point(p)),
                Cell::Door(d) => {
                    if !keyring.unlocks(d) {
                        continue;
                    } else {
                        Some(Move::Point(p))
                    }
                }
                Cell::Key(k) => {
                    if keyring.contains(k) {
                        Some(Move::Point(p))
                    } else {
                        Some(Move::Key(k, p))
                    }
                }
            };
        })
    }

    /*
    fn paths(&self, start: Point, end: Point) -> HashMap<Door, u32> {
        // A* first ignore doors to see if path is at all possible??
        // All shortest routes from start to end per keys required
    }
    */

    fn route(&self) -> u32 {
        let position = self.start;
        let keyring = Keyring::empty();

        let mut visited = HashMap::new();
        let mut haystack = Vec::new();

        let mut min = u32::MAX;

        haystack.push((Move::Point(position), keyring, 0));

        while let Some((next, mut keyring, mut count)) = haystack.pop() {
            let p = match next {
                Move::Point(p) => p,
                Move::Key(k, p) => {
                    keyring.add(k);
                    if keyring.count() == self.keys.len() as u32 {
                        min = min.min(count);
                        continue;
                    }

                    p
                }
            };

            visited.insert((p, keyring), count);

            count += 1;
            if count >= min {
                continue;
            }

            haystack.extend(self.moves(p, keyring).filter_map(|m| {
                if let Some(&old_count) = visited.get(&(m.point(), keyring)) {
                    if old_count <= count {
                        return None;
                    }
                }
                Some((m, keyring, count))
            }));
        }

        min
    }
}
/*

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct NodeId(usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Node {
    Key(Key),
    Start,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Edge {
    left: NodeId,
    right: NodeId,
    cost: u32,
    doors: Door,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    node_map: HashMap<Node, NodeId>,
    key_count: u32,
}

impl Graph {
    fn new(map: &Map) -> Self {
        let mut nodes = Vec::new();
        let mut node_map = HashMap::new();
        let mut edges = Vec::new();

        let start = Node::Start;
        let start_id = NodeId(nodes.len());
        nodes.push(start);
        node_map.insert(start, start_id);

        let mut key_count = 0;
        for &key in map.keys.keys() {
            let key = Node::Key(key);
            let key_id = NodeId(nodes.len());
            nodes.push(key);
            node_map.insert(key, key_id);
            key_count += 1;
        }

        for (&key, &end) in map.keys.iter() {
            let end_node = Node::Key(key);
            let end_node_id = node_map.get(&end_node).unwrap();
            let start = map.start;
            let mut keyring = Keyring::empty();
        }

        Self {
            nodes,
            edges,
            node_map,
            key_count,
        }
    }
}
*/

#[test]
#[ignore]
fn test() {
    let input = r#"#########
#b.A.@.a#
#########"#;
    assert_eq!(8, part_one(input));

    let input = r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"#;
    assert_eq!(86, part_one(input));

    let input = r#"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"#;
    assert_eq!(132, part_one(input));

    let input = r#"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"#;
    assert_eq!(136, part_one(input));

    let input = r#"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"#;
    assert_eq!(81, part_one(input));

    let input = r#"###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############"#;
    assert_eq!(24, part_two(input));

    let input = r#"#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############"#;
    assert_eq!(32, part_two(input));

    let input = r#"#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"#;
    assert_eq!(72, part_two(input));
}
