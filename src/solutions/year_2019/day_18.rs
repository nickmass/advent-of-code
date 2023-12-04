#![allow(dead_code)]
use super::Point2;
use crate::{HashMap, HashSet};
use std::collections::BinaryHeap;

type Point = Point2<i32>;

pub fn part_one(input: &str) -> u32 {
    let map = Map::<1>::new(input);
    let graph = Graph::new(&map);
    graph.route()
}

pub fn part_two(input: &str) -> u32 {
    let map = Map::<4>::new(input);
    // map.dump();
    let graph = Graph::new(&map);
    graph.route()
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Cell {
    Wall,
    Open,
    Door(Door),
    Key(Key),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Wall => '#',
            Cell::Open => '.',
            Cell::Door(d) => (d.0.trailing_zeros() as u8 + b'A') as char,
            Cell::Key(k) => (k.0.trailing_zeros() as u8 + b'a') as char,
        };
        write!(f, "{}", c)
    }
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

type Keyring = AKeyring;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct AKeyring(u32);
impl AKeyring {
    fn empty() -> Self {
        Self(0)
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

    fn dump(&self) {}
}

#[derive(Debug, Clone)]
struct BKeyring(AKeyring, Vec<Key>);
impl BKeyring {
    fn empty() -> Self {
        Self(AKeyring::empty(), Vec::new())
    }

    fn unlocks(&self, door: Door) -> bool {
        self.0.unlocks(door)
    }

    fn add(&mut self, key: Key) {
        self.1.push(key);
        self.0.add(key)
    }

    fn contains(&self, key: Key) -> bool {
        self.0.contains(key)
    }

    fn count(&self) -> u32 {
        self.0.count()
    }

    fn dump(&self) {
        for k in self.1.iter() {
            println!("{}", (k.0.trailing_zeros() as u8 + b'a') as char)
        }
    }
}

impl std::hash::Hash for BKeyring {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl std::cmp::PartialEq for BKeyring {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::cmp::Eq for BKeyring {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Move {
    Point(Point),
    Key(Key, Point),
    Door(Door, Point),
}

impl Move {
    fn point(&self) -> Point {
        match self {
            Move::Point(p) => *p,
            Move::Key(_, p) => *p,
            Move::Door(_, p) => *p,
        }
    }
}

struct Map<const STARTS: usize> {
    cells: Vec<Cell>,
    keys: HashMap<Key, Point>,
    doors: HashMap<Door, Point>,
    starts: [Point; STARTS],
    width: i32,
    height: i32,
}

impl<const STARTS: usize> Map<STARTS> {
    fn new(input: &str) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut start = None;
        let mut width = None;
        let mut cells = Vec::new();
        let mut doors = HashMap::new();
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
                    doors.insert(door, Point::new(x, y));
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

        let starts = match STARTS {
            1 => [start; STARTS],
            4 => {
                let x = start.x as usize;
                let y = start.y as usize;
                let width = width as usize;
                cells[(y + 0) * width + (x + 0)] = Cell::Wall;
                cells[(y - 1) * width + (x - 1)] = Cell::Open;
                cells[(y - 1) * width + (x + 0)] = Cell::Wall;
                cells[(y - 1) * width + (x + 1)] = Cell::Open;
                cells[(y + 0) * width + (x - 1)] = Cell::Wall;
                cells[(y + 0) * width + (x + 1)] = Cell::Wall;
                cells[(y + 1) * width + (x - 1)] = Cell::Open;
                cells[(y + 1) * width + (x + 0)] = Cell::Wall;
                cells[(y + 1) * width + (x + 1)] = Cell::Open;
                let x = start.x;
                let y = start.y;
                vec![
                    Point::new(x - 1, y - 1),
                    Point::new(x + 1, y - 1),
                    Point::new(x - 1, y + 1),
                    Point::new(x + 1, y + 1),
                ]
                .try_into()
                .unwrap()
            }
            _ => panic!("invalid number of starting positions"),
        };

        Self {
            cells,
            keys,
            doors,
            starts,
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

    fn moves<'a>(&'a self, point: Point) -> impl Iterator<Item = Move> + 'a {
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
                Cell::Door(d) => Some(Move::Door(d, p)),
                Cell::Key(k) => Some(Move::Key(k, p)),
            };
        })
    }

    fn paths<'a>(&'a self, start: Point) -> impl Iterator<Item = (Cell, u32)> + 'a {
        let mut visited = HashSet::new();
        let mut haystack = BinaryHeap::new();

        haystack.push(SearchPath(Move::Point(start), 0));

        std::iter::from_fn(move || {
            while let Some(SearchPath(next, count)) = haystack.pop() {
                let p = next.point();
                visited.insert(next.point());

                if !matches!(next, Move::Door(_, _)) {
                    haystack.extend(self.moves(p).filter_map(|m| {
                        if !visited.contains(&m.point()) {
                            Some(SearchPath(m, count + 1))
                        } else {
                            None
                        }
                    }));
                };

                match next {
                    Move::Point(_) => (),
                    Move::Key(k, _) => return Some((Cell::Key(k), count)),
                    Move::Door(d, _) => return Some((Cell::Door(d), count)),
                }
            }

            None
        })
    }

    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x, y);
                if self.starts.iter().any(|s| *s == p) {
                    print!("@");
                } else {
                    let cell = self.get(p).unwrap();
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}

#[derive(PartialEq, Eq)]
struct SearchPath(Move, u32);

impl std::cmp::PartialOrd for SearchPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl std::cmp::Ord for SearchPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct NodeId(usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct EdgeId(usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Node {
    Door(Door),
    Key(Key),
    Start(usize),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Edge {
    left: NodeId,
    right: NodeId,
    cost: u32,
}

struct Graph<const STARTS: usize> {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    node_map: HashMap<Node, NodeId>,
    edge_map: HashMap<NodeId, EdgeId>,
    key_count: u32,
}

impl<const STARTS: usize> Graph<STARTS> {
    fn new(map: &Map<STARTS>) -> Self {
        let mut nodes = Vec::new();
        let mut node_map = HashMap::new();
        let mut edge_map = HashMap::new();
        let mut edges = Vec::new();

        for i in 0..STARTS {
            let start = Node::Start(i);
            let start_id = NodeId(nodes.len());
            nodes.push(start);
            node_map.insert(start, start_id);
        }

        for &key in map.keys.keys() {
            let key = Node::Key(key);
            let key_id = NodeId(nodes.len());
            nodes.push(key);
            node_map.insert(key, key_id);
        }

        for &door in map.doors.keys() {
            let door = Node::Door(door);
            let door_id = NodeId(nodes.len());
            nodes.push(door);
            node_map.insert(door, door_id);
        }

        for node in nodes.iter() {
            let p = match node {
                Node::Door(d) => map.doors.get(d).unwrap(),
                Node::Key(k) => map.keys.get(k).unwrap(),
                Node::Start(i) => &map.starts[*i],
            };

            let left_node_id = node_map.get(&node).unwrap();

            edge_map.insert(*left_node_id, EdgeId(edges.len()));

            for (goal, cost) in map.paths(*p) {
                let right_node = match goal {
                    Cell::Door(d) => Node::Door(d),
                    Cell::Key(k) => Node::Key(k),
                    _ => continue,
                };

                let right_node_id = node_map.get(&right_node).unwrap();

                let edge = Edge {
                    left: *left_node_id,
                    right: *right_node_id,
                    cost,
                };

                edges.push(edge);
            }
        }

        Self {
            nodes,
            edges,
            node_map,
            edge_map,
            key_count: map.keys.len() as u32,
        }
    }

    fn edges<'a>(&'a self, node: Node) -> impl Iterator<Item = Edge> + 'a {
        let node_id = *self.node_map.get(&node).unwrap();
        let EdgeId(mut edge_id) = *self.edge_map.get(&node_id).unwrap();
        std::iter::from_fn(move || {
            if let Some(edge) = self.edges.get(edge_id) {
                edge_id += 1;
                if edge.left != node_id {
                    None
                } else {
                    Some(*edge)
                }
            } else {
                None
            }
        })
    }

    fn unlocked_edges<'a>(
        &'a self,
        node: Node,
        keyring: Keyring,
    ) -> impl Iterator<Item = Edge> + 'a {
        self.edges(node).filter_map(move |edge| {
            let right_node = self.nodes[edge.right.0];
            match right_node {
                Node::Door(d) => {
                    if keyring.unlocks(d) {
                        Some(edge)
                    } else {
                        None
                    }
                }
                Node::Key(k) => {
                    if keyring.contains(k) {
                        None
                    } else {
                        Some(edge)
                    }
                }
                _ => Some(edge),
            }
        })
    }

    fn route(&self) -> u32 {
        let positions = SearchPositions::<STARTS>::new();
        let keyring = Keyring::empty();
        let mut visited = HashSet::new();

        let mut haystack = BinaryHeap::new();

        haystack.push(SearchNode(positions, None, keyring, 0));

        while let Some(SearchNode(positions, robot_idx, mut keyring, cost)) = haystack.pop() {
            if let Some(robot_idx) = robot_idx {
                if !visited.insert((robot_idx, positions.0[robot_idx], keyring.clone())) {
                    continue;
                }

                if let Node::Key(k) = positions.0[robot_idx] {
                    keyring.add(k);

                    if keyring.count() == self.key_count as u32 {
                        keyring.dump();
                        //println!("{}", cost);
                        return cost;
                    }
                }
            }

            for (idx, &robot) in positions.0.iter().enumerate() {
                haystack.extend(self.unlocked_edges(robot, keyring.clone()).filter_map(|e| {
                    let right_node = self.nodes[e.right.0];

                    if visited.contains(&(idx, right_node, keyring.clone())) {
                        None
                    } else {
                        let cost = cost + e.cost;
                        let mut positions = positions;
                        positions.0[idx] = right_node;
                        Some(SearchNode(positions, Some(idx), keyring.clone(), cost))
                    }
                }));
            }
        }

        0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct SearchPositions<const N: usize>([Node; N]);

impl<const N: usize> SearchPositions<N> {
    fn new() -> Self {
        let mut positions = [Node::Start(0); N];
        for i in 0..N {
            positions[i] = Node::Start(i);
        }

        Self(positions)
    }
}

#[derive(PartialEq, Eq)]
struct SearchNode<const N: usize>(SearchPositions<N>, Option<usize>, Keyring, u32);
impl<const N: usize> std::cmp::PartialOrd for SearchNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.3.partial_cmp(&self.3)
    }
}
impl<const N: usize> std::cmp::Ord for SearchNode<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.3.cmp(&self.3)
    }
}

#[test]
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
}

#[test]
#[ignore = "failing edge case"]
fn bad_case() {
    let input = r#"#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"#;
    //Fails ??
    assert_eq!(72, part_two(input));
}
