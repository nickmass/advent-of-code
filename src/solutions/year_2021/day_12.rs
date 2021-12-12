use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> u64 {
    let lines = input.trim().lines();
    let mut nodes = NodeCollection::new();

    for line in lines {
        if let Some((left, right)) = line.split_once('-') {
            let left = nodes.add(left);
            let right = nodes.add(right);

            nodes.connect(left, right);
        }
    }
    let nodes = nodes.complete();

    let visited = HashSet::new();
    backtrack(&nodes, &visited, true, Node::End)
}

pub fn part_two(input: &str) -> u64 {
    let lines = input.trim().lines();
    let mut nodes = NodeCollection::new();

    for line in lines {
        if let Some((left, right)) = line.split_once('-') {
            let left = nodes.add(left);
            let right = nodes.add(right);

            nodes.connect(left, right);
        }
    }
    let nodes = nodes.complete();

    let visited = HashSet::new();
    backtrack(&nodes, &visited, false, Node::End)
}

fn backtrack(
    nodes: &NodeCollection<Complete>,
    visited: &HashSet<Node>,
    extra_small: bool,
    next: Node,
) -> u64 {
    let mut count = 0;
    for node in nodes.neighbors(next) {
        count += match node {
            Node::Start => 1,
            Node::End => 0,
            Node::Small(_) => {
                if !visited.contains(&node) {
                    let mut visited = visited.clone();
                    visited.insert(node);
                    backtrack(nodes, &visited, extra_small, node)
                } else if !extra_small {
                    backtrack(nodes, visited, true, node)
                } else {
                    0
                }
            }
            Node::Big(_) => backtrack(nodes, visited, extra_small, node),
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

trait GraphState {}

struct Complete;
struct Incomplete;

impl GraphState for Complete {}
impl GraphState for Incomplete {}

struct NodeCollection<'a, T: GraphState> {
    node_map: HashMap<&'a str, Node>,
    connections: Vec<Edge>,
    neighbors: HashMap<Node, Vec<Node>>,
    marker: std::marker::PhantomData<T>,
}

impl<'a> NodeCollection<'a, Incomplete> {
    fn new() -> Self {
        Self {
            node_map: HashMap::new(),
            connections: Vec::new(),
            neighbors: HashMap::new(),
            marker: Default::default(),
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
                    let new_node = if node.to_lowercase() == node {
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

    fn complete(mut self) -> NodeCollection<'a, Complete> {
        self.node_map.insert("start", Node::Start);
        self.node_map.insert("end", Node::End);
        for node in self.node_map.values() {
            let mut neighbors = Vec::new();
            for edge in self.connections.iter() {
                if let Some(other) = edge.connects(*node) {
                    neighbors.push(other);
                }
            }

            self.neighbors.insert(*node, neighbors);
        }

        NodeCollection {
            node_map: self.node_map,
            connections: self.connections,
            neighbors: self.neighbors,
            marker: Default::default(),
        }
    }
}

impl<'a> NodeCollection<'a, Complete> {
    fn neighbors<'b>(&'b self, node: Node) -> impl Iterator<Item = Node> + 'b {
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
