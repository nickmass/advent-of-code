use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    let (directions, chart) = input.split_once("\n\n").unwrap();
    let directions = directions.chars().map(Direction::from).cycle();

    let route_map: HashMap<_, _> = chart
        .trim()
        .lines()
        .map(Node::new)
        .map(|n| (n.name, n.values))
        .collect();

    let mut node = NodeName::start_node();

    for (n, dir) in directions.enumerate() {
        let next = route_map.get(&node).unwrap();

        node = match dir {
            Direction::Left => next.left,
            Direction::Right => next.right,
        };

        if node.is_final_node() {
            return n + 1;
        }
    }

    unreachable!()
}

pub fn part_two(input: &str) -> usize {
    let (directions, chart) = input.split_once("\n\n").unwrap();
    let directions = directions.chars().map(Direction::from).cycle();

    let mut route_map = HashMap::new();
    let mut nodes = Vec::new();

    for node in chart.trim().lines().map(Node::new) {
        if node.name.is_start_node() {
            nodes.push(node.name);
        }
        route_map.insert(node.name, node.values);
    }

    let target_count = nodes.len();
    let mut distances = Vec::with_capacity(target_count);

    for (n, dir) in directions.enumerate() {
        for node in nodes.iter_mut() {
            if node.is_end_node() {
                continue;
            }

            let next = route_map.get(node).unwrap();
            *node = match dir {
                Direction::Left => next.left,
                Direction::Right => next.right,
            };

            if node.is_end_node() {
                distances.push(n + 1);
                if distances.len() == target_count {
                    return lcm_iter(distances);
                }
            }
        }
    }

    unreachable!()
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    name: NodeName<'a>,
    values: Route<'a>,
}

impl<'a> Node<'a> {
    fn new(line: &'a str) -> Self {
        let (name, rest) = line.split_once(" = ").unwrap();
        let (left, right) = rest.split_once(", ").unwrap();
        let left = NodeName(&left[1..4]);
        let right = NodeName(&right[0..3]);

        Self {
            name: NodeName(name),
            values: Route { left, right },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct NodeName<'a>(&'a str);

impl<'a> NodeName<'a> {
    fn start_node() -> Self {
        NodeName("AAA")
    }

    fn is_start_node(&self) -> bool {
        self.0.ends_with('A')
    }

    fn is_end_node(&self) -> bool {
        self.0.ends_with('Z')
    }

    fn is_final_node(&self) -> bool {
        self.0 == "ZZZ"
    }
}

#[derive(Debug, Copy, Clone)]
struct Route<'a> {
    left: NodeName<'a>,
    right: NodeName<'a>,
}

fn gcd(a: usize, b: usize) -> usize {
    let mut n = a.min(b);
    while n > 1 {
        if a % n == 0 && b % n == 0 {
            return n;
        }
        n -= 1;
    }

    1
}

fn lcm_iter<I: IntoIterator<Item = usize>>(iter: I) -> usize {
    iter.into_iter().fold(1, |acc, n| (acc * n) / gcd(acc, n))
}

#[test]
fn test() {
    let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
    assert_eq!(2, part_one(input));

    let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;
    assert_eq!(6, part_one(input));

    let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;
    assert_eq!(6, part_two(input));
}
