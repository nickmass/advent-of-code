use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    let nodes = Nodes::new(input.lines().filter_map(|l| l.split_once('-')));

    let mut visited = Vec::new();
    search(&nodes, &mut visited, false, Node::Start)
}

pub fn part_two(input: &str) -> u64 {
    let nodes = Nodes::new(input.lines().filter_map(|l| l.split_once('-')));

    let mut visited = Vec::new();
    search(&nodes, &mut visited, true, Node::Start)
}

fn search(nodes: &Nodes, visited: &mut Vec<Node>, extra_small: bool, next: Node) -> u64 {
    let mut count = 0;
    for node in nodes.neighbors(next) {
        count += match node {
            Node::Start => 0,
            Node::End => 1,
            Node::Small(_) => {
                if !visited.contains(&node) {
                    visited.push(node);
                    let count = search(nodes, visited, extra_small, node);
                    visited.pop();
                    count
                } else if extra_small {
                    search(nodes, visited, false, node)
                } else {
                    0
                }
            }
            Node::Big(_) => search(nodes, visited, extra_small, node),
        };
    }
    count
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Small(usize),
    Big(usize),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Edge(Node, Node);

impl Edge {
    fn connects(&self, node: Node) -> Option<Node> {
        if self.0 == node {
            Some(self.1)
        } else if self.1 == node {
            Some(self.0)
        } else {
            None
        }
    }
}

struct NodeCollection<'a> {
    node_map: HashMap<&'a str, Node>,
    connections: Vec<Edge>,
}

impl<'a> NodeCollection<'a> {
    fn new() -> Self {
        Self {
            node_map: HashMap::new(),
            connections: Vec::new(),
        }
    }

    fn add(&mut self, name: &'a str) -> Node {
        match name {
            "start" => Node::Start,
            "end" => Node::End,
            node => {
                if let Some(node) = self.node_map.get(node) {
                    *node
                } else {
                    let new_node = if node.chars().all(|n| n.is_ascii_lowercase()) {
                        Node::Small(self.node_map.len())
                    } else {
                        Node::Big(self.node_map.len())
                    };

                    self.node_map.insert(name, new_node);

                    new_node
                }
            }
        }
    }

    fn connect(&mut self, left: Node, right: Node) {
        self.connections.push(Edge(left, right));
    }

    fn complete(mut self) -> Nodes {
        self.node_map.insert("start", Node::Start);
        self.node_map.insert("end", Node::End);

        let mut neighbors = HashMap::with_capacity(self.node_map.len());
        for node in self.node_map.values() {
            let connections: Vec<_> = self
                .connections
                .iter()
                .filter_map(|c| c.connects(*node))
                .collect();

            neighbors.insert(*node, connections.into_boxed_slice());
        }

        Nodes { neighbors }
    }
}

struct Nodes {
    neighbors: HashMap<Node, Box<[Node]>>,
}

impl Nodes {
    fn new<'a, I: Iterator<Item = (&'a str, &'a str)>>(iter: I) -> Nodes {
        let mut nodes = NodeCollection::new();

        for (left, right) in iter {
            let left = nodes.add(left);
            let right = nodes.add(right);

            nodes.connect(left, right)
        }

        nodes.complete()
    }

    fn neighbors(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.neighbors.get(&node).unwrap().iter().copied()
    }
}

#[test]
fn test() {
    let input = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    assert_eq!(226, part_one(input));
    assert_eq!(3509, part_two(input));
}
