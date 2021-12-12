use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    let nodes = Nodes::new(input.lines().filter_map(|l| l.split_once('-')));

    let mut visited = Tree::new();
    backtrack(&nodes, &mut visited.branch(Node::End), true, Node::End)
}

pub fn part_two(input: &str) -> u64 {
    let nodes = Nodes::new(input.lines().filter_map(|l| l.split_once('-')));

    let mut visited = Tree::new();
    backtrack(&nodes, &mut visited.branch(Node::End), false, Node::End)
}

fn backtrack(nodes: &Nodes, visited: &mut Leaf<Node>, extra_small: bool, next: Node) -> u64 {
    let mut count = 0;
    for node in nodes.neighbors(next) {
        count += match node {
            Node::Start => 1,
            Node::End => 0,
            Node::Small(_) => {
                if !visited.contains(node) {
                    let mut visited = visited.branch(node);
                    backtrack(nodes, &mut visited, extra_small, node)
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

    fn complete(mut self) -> Nodes {
        let mut neighbor_map = HashMap::new();

        self.node_map.insert("start", Node::Start);
        self.node_map.insert("end", Node::End);
        for node in self.node_map.values() {
            let mut neighbors = Vec::new();
            for edge in self.connections.iter() {
                if let Some(other) = edge.connects(*node) {
                    neighbors.push(other);
                }
            }

            neighbor_map.insert(*node, neighbors.into_boxed_slice());
        }

        Nodes {
            neighbors: neighbor_map,
        }
    }
}

struct Nodes {
    neighbors: HashMap<Node, Box<[Node]>>,
}

impl Nodes {
    fn new<'a, I: Iterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        let mut nodes = NodeCollection::new();

        for (left, right) in iter {
            let left = nodes.add(left);
            let right = nodes.add(right);

            nodes.connect(left, right)
        }

        nodes.complete()
    }

    fn neighbors<'a>(&'a self, node: Node) -> impl Iterator<Item = Node> + 'a {
        self.neighbors.get(&node).unwrap().iter().copied()
    }
}
struct Tree<T: Eq> {
    entries: Vec<(usize, T)>,
}

impl<T: Eq> Tree<T> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    fn branch(&mut self, node: T) -> Leaf<T> {
        let id = self.entries.len();
        self.entries.push((id, node));
        Leaf { tree: self, id }
    }
}

struct Leaf<'t, T: Eq> {
    tree: &'t mut Tree<T>,
    id: usize,
}

impl<'t, T: Eq> Leaf<'t, T> {
    fn branch(&mut self, node: T) -> Leaf<T> {
        let id = self.tree.entries.len();
        self.tree.entries.push((self.id, node));
        Leaf {
            tree: self.tree,
            id,
        }
    }

    fn contains(&self, node: T) -> bool {
        let mut leaf = self.id;
        loop {
            let entry = &self.tree.entries[leaf];
            if entry.1 == node {
                return true;
            } else {
                leaf = entry.0;
                if leaf == 0 {
                    return false;
                }
            }
        }
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
