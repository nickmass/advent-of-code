use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    let nodes = Nodes::new(input.lines().filter_map(|l| l.split_once('-')));

    let mut visited = Tree::new();
    let mut start = visited.branch(Node::End);
    backtrack(&nodes, &mut start, true, Node::End)
}

pub fn part_two(input: &str) -> u64 {
    let nodes = Nodes::new(input.lines().filter_map(|l| l.split_once('-')));

    let mut visited = Tree::new();
    let mut start = visited.branch(Node::End);
    backtrack(&nodes, &mut start, false, Node::End)
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
    entries: Vec<Option<(usize, T)>>,
    empty_slot: Option<usize>,
}

impl<T: Eq> Tree<T> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            empty_slot: None,
        }
    }
    fn branch(&mut self, node: T) -> Leaf<T> {
        let id = self.entries.len();
        self.entries.push(Some((id, node)));
        Leaf { tree: self, id }
    }

    fn add(&mut self, parent: usize, node: T) -> usize {
        let mut id = None;
        for idx in self.empty_slot.unwrap_or(usize::MAX)..self.entries.len() {
            if self.entries[idx].is_none() {
                id = Some(idx);
                break;
            }
        }

        if let Some(id) = id {
            self.empty_slot.as_mut().map(|c| *c += 1);
            self.entries[id] = Some((parent, node));

            id
        } else {
            let id = self.entries.len();
            self.empty_slot = None;
            self.entries.push(Some((parent, node)));

            id
        }
    }
}

struct Leaf<'t, T: Eq> {
    tree: &'t mut Tree<T>,
    id: usize,
}

impl<'t, T: Eq> Leaf<'t, T> {
    fn branch(&mut self, node: T) -> Leaf<T> {
        let id = self.tree.add(self.id, node);
        Leaf {
            tree: self.tree,
            id,
        }
    }

    fn contains(&self, node: T) -> bool {
        let mut leaf = self.id;
        loop {
            if let Some(entry) = &self.tree.entries[leaf] {
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
}

impl<'t, T: Eq> Drop for Leaf<'t, T> {
    fn drop(&mut self) {
        self.tree.entries[self.id] = None;
        self.tree.empty_slot = Some(self.tree.empty_slot.unwrap_or(usize::MAX).min(self.id));
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
