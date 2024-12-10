use crate::{HashMap, HashSet};
use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> i32 {
    let map = Map::new(input);
    let graph = Graph::new(&map);
    graph.path()
}

pub fn part_two(input: &str) -> i32 {
    let map = Map::new(input);
    let graph = Graph::new(&map);
    graph.recursive_path()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Letter(u8),
    Void,
    Wall,
    Open,
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut width = None;
        let mut height = 0;
        for line in input.lines() {
            let line = line.as_bytes();
            if width.is_none() {
                width = Some(line.len());
            }

            for b in line {
                let cell = match b {
                    b' ' => Cell::Void,
                    b'#' => Cell::Wall,
                    b'.' => Cell::Open,
                    b'A'..=b'Z' => Cell::Letter(*b),
                    _ => unreachable!(),
                };

                cells.push(cell);
            }

            if !line.is_empty() {
                height += 1;
            } else {
                panic!()
            }
        }

        let width = width.unwrap();

        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = y * self.width + x;
            self.cells.get(idx).copied()
        }
    }

    fn neighbors(&self, x: i32, y: i32) -> impl Iterator<Item = (Cell, i32, i32)> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || loop {
            let (x, y) = match i {
                0 => (x + 1, y + 0),
                1 => (x - 1, y + 0),
                2 => (x + 0, y + 1),
                3 => (x + 0, y - 1),
                _ => return None,
            };

            i += 1;

            if let Some(cell) = self.get(x, y) {
                return Some((cell, x, y));
            }
        })
    }

    fn paths<'a, T: Copy>(
        &'a self,
        start_x: i32,
        start_y: i32,
        targets: &'a HashMap<(i32, i32), T>,
    ) -> impl Iterator<Item = (T, i32)> + 'a {
        let mut visited = HashSet::new();
        let mut haystack = BinaryHeap::new();

        haystack.push((0, (start_x, start_y)));

        std::iter::from_fn(move || {
            while let Some((cost, (x, y))) = haystack.pop() {
                if !visited.insert((x, y)) {
                    continue;
                }

                haystack.extend(self.neighbors(x, y).filter_map(|(ty, x, y)| {
                    if let Cell::Open = ty {
                        Some((cost + 1, (x, y)))
                    } else {
                        None
                    }
                }));

                if let Some(targ) = targets.get(&(x, y)) {
                    if (start_x, start_y) != (x, y) {
                        return Some((*targ, cost));
                    }
                }
            }

            None
        })
    }

    fn classify(&self, x: i32, y: i32) -> NodeClass {
        const EDGE_WIDTH: i32 = 4;
        if x >= EDGE_WIDTH
            && y >= EDGE_WIDTH
            && x < self.width as i32 - EDGE_WIDTH
            && y < self.height as i32 - EDGE_WIDTH
        {
            NodeClass::Descend
        } else {
            NodeClass::Rise
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    name: [u8; 2],
}
impl Node {
    fn start_end(&self) -> bool {
        self.name == [b'A', b'A'] || self.name == [b'Z', b'Z']
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0] as char, self.name[1] as char)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum NodeClass {
    Rise,
    Descend,
    Equal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct ClassifiedNode(NodeClass, Node);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct RecursiveNode(Node, i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct NodeId(usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct EdgeId(usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Edge {
    left: NodeId,
    right: NodeId,
    cost: i32,
    class: NodeClass,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct RecursiveEdge(Edge, i32);

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    node_map: HashMap<Node, NodeId>,
    edge_map: HashMap<Node, EdgeId>,
}

impl Graph {
    fn new(map: &Map) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let mut node_map = HashMap::new();
        let mut edge_map = HashMap::new();

        let mut teleporters = HashMap::new();

        for y in 0..map.height {
            let y = y as i32;
            for x in 0..map.width {
                let x = x as i32;

                let cell = map.get(x, y);
                if let Some(Cell::Letter(l1)) = cell {
                    let mut l2 = None;
                    let mut point = None;

                    for (cell, x, y) in map.neighbors(x, y) {
                        match cell {
                            Cell::Letter(l) => l2 = Some((l, x, y)),
                            Cell::Open => point = Some((x, y)),
                            _ => (),
                        }
                    }

                    let l2 = l2.unwrap();

                    if point.is_none() {
                        point = map
                            .neighbors(l2.1, l2.2)
                            .find(|(c, _, _)| *c == Cell::Open)
                            .map(|(_, x, y)| (x, y));
                    }

                    let point = point.unwrap();

                    let swap = y > l2.2 || x > l2.1;

                    let node = if swap {
                        Node { name: [l2.0, l1] }
                    } else {
                        Node { name: [l1, l2.0] }
                    };

                    let node = ClassifiedNode(map.classify(point.0, point.1), node);

                    teleporters.insert(point, node);
                }
            }
        }

        for ClassifiedNode(_, node) in teleporters.values() {
            if node_map.contains_key(node) {
                continue;
            }

            let node_id = NodeId(nodes.len());
            nodes.push(*node);
            node_map.insert(*node, node_id);
        }

        let mut ordered_nodes: Vec<_> = teleporters.iter().collect();
        ordered_nodes.sort_by_key(|n| u16::from_be_bytes(n.1 .1.name));

        for (&(x, y), &ClassifiedNode(left_class, node)) in ordered_nodes {
            let left = *node_map.get(&node).unwrap();
            let edge_id = EdgeId(edges.len());

            for (ClassifiedNode(right_class, right_node), cost) in map.paths(x, y, &teleporters) {
                let right = *node_map.get(&right_node).unwrap();

                let class = if left_class == right_class {
                    NodeClass::Equal
                } else {
                    right_class
                };

                let edge = Edge {
                    left,
                    right,
                    cost: cost + 1,
                    class,
                };
                edges.push(edge);
            }

            if !edge_map.contains_key(&node) {
                edge_map.insert(node, edge_id);
            }
        }

        Self {
            nodes,
            edges,
            node_map,
            edge_map,
        }
    }

    fn edges(&self, node: Node) -> impl Iterator<Item = Edge> + '_ {
        let node_id = self.node_map.get(&node).unwrap();
        let edge_id = self.edge_map.get(&node).unwrap();
        let mut i = edge_id.0;

        std::iter::from_fn(move || {
            if let Some(edge) = self.edges.get(i) {
                i += 1;
                if edge.left == *node_id {
                    return Some(*edge);
                } else {
                    return None;
                }
            }
            None
        })
    }

    fn recursive_edges(
        &self,
        RecursiveNode(node, depth): RecursiveNode,
    ) -> impl Iterator<Item = RecursiveEdge> + '_ {
        self.edges(node).filter_map(move |edge| {
            if let Some(right_node) = self.nodes.get(edge.right.0) {
                return match edge.class {
                    NodeClass::Rise if depth > 0 && !right_node.start_end() => {
                        Some(RecursiveEdge(edge, depth - 1))
                    }
                    NodeClass::Rise if depth == 1 && right_node.start_end() => {
                        Some(RecursiveEdge(edge, 0))
                    }
                    NodeClass::Descend if !right_node.start_end() => {
                        Some(RecursiveEdge(edge, depth + 1))
                    }
                    NodeClass::Equal => Some(RecursiveEdge(edge, depth)),
                    _ => None,
                };
            }

            None
        })
    }

    fn path(self) -> i32 {
        let start = Node { name: [b'A', b'A'] };
        let end = Node { name: [b'Z', b'Z'] };

        let mut visited = HashSet::new();
        let mut haystack = BinaryHeap::new();

        haystack.push(Ordered(0, start));

        while let Some(Ordered(cost, node)) = haystack.pop() {
            if !visited.insert(node) {
                continue;
            }

            if node == end {
                return cost - 1;
            }

            haystack.extend(
                self.edges(node)
                    .map(|e| Ordered(cost + e.cost, self.nodes[e.right.0])),
            );
        }

        0
    }

    fn recursive_path(self) -> i32 {
        let start = RecursiveNode(Node { name: [b'A', b'A'] }, 0);
        let end = RecursiveNode(Node { name: [b'Z', b'Z'] }, 0);

        let mut visited = HashSet::new();
        let mut haystack = BinaryHeap::new();

        haystack.push(Ordered(0, start));

        while let Some(Ordered(cost, node)) = haystack.pop() {
            if !visited.insert(node) {
                continue;
            }

            if node == end {
                return cost - 1;
            }

            haystack
                .extend(self.recursive_edges(node).map(|e| {
                    Ordered(cost + e.0.cost, RecursiveNode(self.nodes[e.0.right.0], e.1))
                }));
        }

        0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Ordered<T>(i32, T);
impl<T: Eq> std::cmp::PartialOrd for Ordered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Eq> std::cmp::Ord for Ordered<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

#[test]
fn test() {
    let input = r#"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"#;
    assert_eq!(23, part_one(input));

    let input = r#"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
"#;

    assert_eq!(58, part_one(input));

    let input = r#"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
"#;

    assert_eq!(396, part_two(input));
}
