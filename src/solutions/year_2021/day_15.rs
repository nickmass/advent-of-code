use crate::HashMap;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub fn part_one(input: &str) -> u64 {
    let map = Map::new(input, 1);

    pathfind(&map)
}

pub fn part_two(input: &str) -> u64 {
    let map = Map::new(input, 5);

    pathfind(&map)
}

fn pathfind(map: &Map) -> u64 {
    let start = (0, 0);
    let mut discovered = BinaryHeap::new();
    let f_score = Rc::new(RefCell::new(HashMap::new()));
    discovered.push(Reverse(HeapItem::new(
        map.node(start).unwrap(),
        f_score.clone(),
    )));

    let mut came_from: HashMap<(i64, i64), ((i64, i64), u64)> = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    f_score.borrow_mut().insert(start, map.distance(start));

    while let Some(Reverse(HeapItem { node: next, .. })) = discovered.pop() {
        if next.point == map.target() {
            let mut sum = next.score;
            let mut current = next.point;
            while let Some(next) = came_from.get(&current) {
                if next.0 != start {
                    sum += next.1;
                }
                current = next.0;
            }

            return sum;
        }

        for neighbor in map.neighbors(next.point) {
            let current_g_score = g_score.get(&(next.point)).copied().unwrap() + neighbor.score;

            if current_g_score
                < g_score
                    .get(&(neighbor.point))
                    .copied()
                    .unwrap_or(u32::MAX as u64)
            {
                came_from.insert(neighbor.point, (next.point, next.score));
                g_score.insert(neighbor.point, current_g_score);
                f_score
                    .borrow_mut()
                    .insert(neighbor.point, current_g_score + neighbor.distance);

                discovered.push(Reverse(HeapItem::new(neighbor, f_score.clone())));
            }
        }
    }

    panic!("no path found")
}

#[derive(Eq, PartialEq)]
struct HeapItem {
    node: Node,
    f_scores: Rc<RefCell<HashMap<(i64, i64), u64>>>,
}

impl HeapItem {
    fn new(node: Node, f_scores: Rc<RefCell<HashMap<(i64, i64), u64>>>) -> Self {
        Self { node, f_scores }
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_score = self
            .f_scores
            .borrow()
            .get(&self.node.point)
            .copied()
            .unwrap_or(u32::MAX as u64);
        let other_score = self
            .f_scores
            .borrow()
            .get(&other.node.point)
            .copied()
            .unwrap_or(u32::MAX as u64);

        my_score.cmp(&other_score)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Node {
    point: (i64, i64),
    score: u64,
    distance: u64,
}

struct Map {
    cells: Vec<u8>,
    stride: usize,
    height: usize,
    scale: usize,
}

impl Map {
    fn new(input: &str, scale: usize) -> Self {
        let lines = input.trim().lines().peekable();
        let stride = lines.clone().peek().map(|l| l.len()).unwrap_or_default();
        let height = lines.clone().count();

        let mut cells = vec![0; stride * height];

        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                cells[(y * stride) + x] = c.to_digit(10).unwrap_or_default() as u8;
            }
        }

        Map {
            cells,
            stride,
            height,
            scale,
        }
    }

    fn get(&self, (x, y): (i64, i64)) -> Option<u8> {
        if x < 0
            || y < 0
            || x as usize >= self.stride * self.scale
            || y as usize >= self.height * self.scale
        {
            return None;
        }
        let y = y as usize;
        let x = x as usize;
        let orig_y = y;
        let orig_x = x;
        let y = y % self.height;
        let x = x % self.stride;

        let idx = (y * self.stride) + x;

        if let Some(cell) = self.cells.get(idx).copied() {
            let scale_y = orig_y / self.height;
            let scale_x = orig_x / self.stride;
            let new = (cell - 1) as usize + scale_y + scale_x;
            let new = (new % 9) + 1;
            Some(new as u8)
        } else {
            None
        }
    }

    fn cost(&self, point: (i64, i64)) -> Option<u64> {
        self.get(point).map(|n| n as u64)
    }

    fn node(&self, point: (i64, i64)) -> Option<Node> {
        self.cost(point).map(|score| Node {
            point,
            score,
            distance: self.distance(point),
        })
    }

    fn target(&self) -> (i64, i64) {
        (
            ((self.height * self.scale) - 1) as i64,
            ((self.stride * self.scale) - 1) as i64,
        )
    }

    fn distance(&self, point: (i64, i64)) -> u64 {
        let other = self.target();
        ((point.0 - other.0).abs() + (point.1 - other.1).abs()) as u64
    }

    fn neighbors(&self, (x, y): (i64, i64)) -> impl IntoIterator<Item = Node> + '_ {
        [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)]
            .into_iter()
            .filter_map(|p| self.node(p))
    }
}

#[test]
fn test() {
    let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    assert_eq!(40, part_one(input));
    assert_eq!(315, part_two(input));
}
