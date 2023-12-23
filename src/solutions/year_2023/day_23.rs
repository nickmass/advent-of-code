use std::collections::VecDeque;

use crate::HashSet;

pub fn part_one(input: &str) -> u32 {
    let map = Map::new(input, false);
    let graph = Graph::new(map);
    graph.find_long_path()
}

pub fn part_two(input: &str) -> u32 {
    let map = Map::new(input, true);
    let graph = Graph::new(map);
    graph.find_long_path()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Open,
    Forest,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn apply(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
        }
    }

    fn allowed(&self, cell: Cell) -> bool {
        match (self, cell) {
            (_, Cell::Open) => true,
            (Direction::Left, Cell::Left) => true,
            (Direction::Right, Cell::Right) => true,
            (Direction::Up, Cell::Up) => true,
            (Direction::Down, Cell::Down) => true,
            _ => false,
        }
    }
}

struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    start: (i32, i32),
    end: (i32, i32),
}

impl Map {
    fn new(input: &str, dry_trail: bool) -> Self {
        let mut cells = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 0;
        let mut start = None;
        let mut end = (0, 0);

        for (y, line) in input.trim().lines().enumerate() {
            if width < line.len() {
                width = line.len();
            }
            height += 1;

            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as i32, y as i32);
                let cell = match c {
                    '#' => Cell::Forest,
                    '<' | '>' | 'v' | '^' if dry_trail => Cell::Open,
                    '.' => {
                        if start.is_none() {
                            start = Some((x, y));
                        }
                        end = (x, y);
                        Cell::Open
                    }
                    '<' => Cell::Left,
                    '>' => Cell::Right,
                    '^' => Cell::Up,
                    'v' => Cell::Down,
                    _ => unreachable!(),
                };

                cells.push(cell)
            }
        }

        let start = start.unwrap();

        Self {
            cells,
            width,
            height,
            start,
            end,
        }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);

        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = y * self.width + x;
            self.cells.get(idx).copied()
        }
    }

    fn neighbors(
        &self,
        (x, y): (i32, i32),
    ) -> impl Iterator<Item = (Direction, (i32, i32), Cell)> + '_ {
        let mut i = 0;

        std::iter::from_fn(move || loop {
            let dir = match i {
                0 => Direction::Left,
                1 => Direction::Right,
                2 => Direction::Up,
                3 => Direction::Down,
                _ => return None,
            };

            i += 1;

            let p = dir.apply((x, y));

            if let Some(cell) = self.get(p) {
                if cell != Cell::Forest {
                    return Some((dir, p, cell));
                }
            }
        })
    }

    fn allowed_neighbor(
        &self,
        current_point: (i32, i32),
        direction: Direction,
        target_point: (i32, i32),
        target_cell: Cell,
        ignore_slopes: bool,
    ) -> Option<(i32, i32)> {
        let current_cell = self.get(current_point);
        if ignore_slopes {
            Some(target_point)
        } else if let Some(current_cell) = current_cell {
            if direction.allowed(target_cell) && direction.allowed(current_cell) {
                Some(target_point)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn intersection_distance(
        &self,
        start: (i32, i32),
        cross_intersections: bool,
        ignore_slopes: bool,
    ) -> impl Iterator<Item = (u32, (i32, i32))> + '_ {
        let mut visited = HashSet::new();
        let mut haystack = VecDeque::new();
        let mut temp = Vec::new();
        haystack.push_back((0, start));

        std::iter::from_fn(move || {
            while let Some((cost, point)) = haystack.pop_front() {
                if !visited.insert(point) {
                    continue;
                }

                temp.clear();
                temp.extend(self.neighbors(point));

                let filter_neighbors = |(dir, targ, cell)| {
                    self.allowed_neighbor(point, dir, targ, cell, ignore_slopes)
                        .map(|p| (cost + 1, p))
                };

                let intersection = if temp.len() > 2 && point != start {
                    if cross_intersections {
                        haystack.extend(temp.drain(..).filter_map(filter_neighbors));
                    }
                    Some(point)
                } else {
                    haystack.extend(temp.drain(..).filter_map(filter_neighbors));

                    if point == self.start || point == self.end {
                        Some(point)
                    } else {
                        None
                    }
                };

                if let Some(intersection) = intersection {
                    if intersection != start {
                        return Some((cost, intersection));
                    }
                }
            }

            None
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Node {
    Start,
    Intersection((i32, i32)),
    End,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BitNode(u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BitNodeFilter(u64);

impl BitNodeFilter {
    fn new() -> Self {
        BitNodeFilter(0)
    }

    fn insert(self, BitNode(idx): BitNode) -> Self {
        let n = 1 << idx;
        BitNodeFilter(self.0 | n)
    }

    fn contains(&self, BitNode(idx): BitNode) -> bool {
        let n = 1 << idx;
        self.0 & n != 0
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    left: BitNode,
    right: BitNode,
    cost: u32,
}

#[derive(Debug, Copy, Clone)]
struct EdgeId(usize);

#[derive(Debug)]
struct Graph {
    edges: Vec<Edge>,
    edge_map: Vec<EdgeId>,
}

impl Graph {
    fn new(map: Map) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Start must be BitNode(0)
        nodes.push(Node::Start);
        // End must be BitNode(1)
        nodes.push(Node::End);

        for (_, point) in map.intersection_distance(map.start, true, true) {
            if point == map.start || point == map.end {
                continue;
            }
            let node = Node::Intersection(point);
            nodes.push(node);
        }

        assert!(
            nodes.len() < 64,
            "BitNode structure only supports 64 or fewer nodes"
        );

        let mut edge_map = Vec::new();

        for (left_idx, &left_node) in nodes.iter().enumerate() {
            let start = match left_node {
                Node::Start => map.start,
                Node::Intersection(p) => p,
                Node::End => map.end,
            };
            let edge_id = EdgeId(edges.len());
            let left_bit_node = BitNode(left_idx as u8);
            for (cost, right) in map.intersection_distance(start, false, false) {
                if left_node == Node::End {
                    break;
                }
                let right_node = if right == map.start {
                    continue;
                } else if right == map.end {
                    Node::End
                } else {
                    Node::Intersection(right)
                };

                let right_idx = nodes.iter().position(|n| *n == right_node).unwrap();

                let edge = Edge {
                    left: left_bit_node,
                    right: BitNode(right_idx as u8),
                    cost,
                };

                edges.push(edge);
            }
            edge_map.push(edge_id);
        }

        Self { edges, edge_map }
    }

    fn edges<'a>(&'a self, node: BitNode) -> impl Iterator<Item = Edge> + 'a {
        let edge_id = self.edge_map.get(node.0 as usize).unwrap();
        let mut i = edge_id.0;

        std::iter::from_fn(move || {
            if let Some(edge) = self.edges.get(i) {
                i += 1;
                if edge.left == node {
                    Some(*edge)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    fn find_long_path(&self) -> u32 {
        let visited = BitNodeFilter::new();
        let mut haystack = Vec::new();
        haystack.push((0, visited, BitNode(0)));

        let mut max = 0;

        while let Some((cost, visited, node)) = haystack.pop() {
            if node == BitNode(1) {
                max = max.max(cost);
                continue;
            }

            haystack.extend(
                self.edges(node)
                    .filter(|e| !visited.contains(e.right))
                    .map(|e| (cost + e.cost, visited.insert(node), e.right)),
            );
        }

        max
    }
}

#[test]
fn test() {
    let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#;

    assert_eq!(94, part_one(input));
    assert_eq!(154, part_two(input));
}
