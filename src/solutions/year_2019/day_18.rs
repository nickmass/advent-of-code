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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Keyring(u32, bool);
impl Keyring {
    fn empty() -> Self {
        Self(0, false)
    }

    fn master() -> Self {
        Self(0, true)
    }

    fn unlocks(&self, door: Door) -> bool {
        self.0 | door.0 == self.0 || self.1
    }

    fn add(&mut self, key: Key) {
        self.0 |= key.0;
    }

    fn contains(&self, key: Key) -> bool {
        self.0 | key.0 == self.0
    }

    fn overlaps(&self, other: Keyring) -> bool {
        self.0 | other.0 == self.0
    }

    fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

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

        haystack.push(Ordered(0, Move::Point(start)));

        std::iter::from_fn(move || {
            while let Some(Ordered(count, next)) = haystack.pop() {
                let p = next.point();
                visited.insert(next.point());

                if !matches!(next, Move::Door(_, _)) {
                    haystack.extend(self.moves(p).filter_map(|m| {
                        if !visited.contains(&m.point()) {
                            Some(Ordered(count + 1, m))
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

    #[allow(dead_code)]
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct NodeId(usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct EdgeId(usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Node {
    Door(Door),
    Key(Key),
    Start(u8),
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
            let start = Node::Start(i as u8);
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
                Node::Start(i) => &map.starts[*i as usize],
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

    fn virtual_edges<'a>(
        &'a self,
        node: Node,
        keyring: Keyring,
        visited: &'a mut HashSet<Node>,
        haystack: &'a mut BinaryHeap<Ordered<Edge>>,
    ) -> impl Iterator<Item = Edge> + 'a {
        visited.clear();
        haystack.clear();
        haystack.extend(
            self.unlocked_edges(node, keyring)
                .map(|e| Ordered(e.cost, e)),
        );

        std::iter::from_fn(move || {
            while let Some(Ordered(cost, next)) = haystack.pop() {
                let node = self.nodes[next.right.0];

                if !visited.insert(node) {
                    continue;
                }

                haystack.extend(self.unlocked_edges(node, keyring).filter_map(|edge| {
                    let right_node = self.nodes[edge.right.0];
                    if !visited.contains(&right_node) {
                        Some(Ordered(cost + edge.cost, edge))
                    } else {
                        None
                    }
                }));

                if let Node::Key(_) = node {
                    let mut next = next;
                    next.cost = cost;
                    return Some(next);
                }
            }

            None
        })
    }

    fn reachable_keys(
        &self,
        start: Node,
        visited: &mut HashSet<Node>,
        haystack: &mut BinaryHeap<Ordered<Edge>>,
    ) -> Keyring {
        self.virtual_edges(start, Keyring::master(), visited, haystack)
            .filter_map(|e| {
                let node = self.nodes[e.right.0];
                if let Node::Key(k) = node {
                    Some(k)
                } else {
                    None
                }
            })
            .fold(Keyring::empty(), |mut keyring, key| {
                keyring.add(key);
                keyring
            })
    }

    fn route(&self) -> u32 {
        let positions = SearchPositions::<STARTS>::new();
        let mut scratch_set = HashSet::new();
        let mut scratch_heap = BinaryHeap::new();
        let reachable_keys = positions
            .0
            .iter()
            .map(|p| self.reachable_keys(*p, &mut scratch_set, &mut scratch_heap))
            .collect::<Vec<_>>();

        let keyring = Keyring::empty();
        let mut visited = HashSet::new();
        let mut haystack = BinaryHeap::new();

        haystack.push(Ordered(0, (positions, None, keyring)));

        while let Some(Ordered(cost, (positions, robot_idx, mut keyring))) = haystack.pop() {
            if let Some(robot_idx) = robot_idx {
                if !visited.insert((positions, keyring)) {
                    continue;
                }

                if let Node::Key(k) = positions.0[robot_idx] {
                    keyring.add(k);

                    if keyring.count() == self.key_count {
                        return cost;
                    }
                }
            }

            for (idx, &robot) in positions.0.iter().enumerate() {
                if keyring.overlaps(reachable_keys[idx]) {
                    continue;
                }

                haystack.extend(
                    self.virtual_edges(robot, keyring, &mut scratch_set, &mut scratch_heap)
                        .filter_map(|e| {
                            let right_node = self.nodes[e.right.0];
                            let mut positions = positions;
                            positions.0[idx] = right_node;

                            if visited.contains(&(positions, keyring)) {
                                None
                            } else {
                                Some(Ordered(cost + e.cost, (positions, Some(idx), keyring)))
                            }
                        }),
                );
            }
        }

        0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Ordered<T>(u32, T);
impl<T: Eq> std::cmp::PartialOrd for Ordered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl<T: Eq> std::cmp::Ord for Ordered<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct SearchPositions<const N: usize>([Node; N]);

impl<const N: usize> SearchPositions<N> {
    fn new() -> Self {
        let mut positions = [Node::Start(0); N];
        for i in 0..N {
            positions[i] = Node::Start(i as u8);
        }

        Self(positions)
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
