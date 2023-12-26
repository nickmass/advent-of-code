use std::collections::VecDeque;

use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let mut graph = Graph::new(input);
    graph.remove_connecting_edges();
    graph.score()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node<'a>(&'a str);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Edge<'a> {
    left: Node<'a>,
    right: Node<'a>,
    disable: bool,
}

impl<'a> Edge<'a> {
    fn cannocial(&self) -> Self {
        if self.left.0 < self.right.0 {
            *self
        } else {
            Edge {
                left: self.right,
                right: self.left,
                disable: false,
            }
        }
    }
}

struct Graph<'a> {
    nodes: HashSet<Node<'a>>,
    edges: Vec<Edge<'a>>,
    edge_map: HashMap<Node<'a>, usize>,
}

impl<'a> Graph<'a> {
    fn new(input: &'a str) -> Self {
        let mut edges = Vec::new();
        let mut nodes = HashSet::new();

        for line in input.trim().lines() {
            let (left_name, rest) = line.split_once(": ").unwrap();

            let left = Node(left_name);
            nodes.insert(left);

            for right_name in rest.split(" ") {
                let right = Node(right_name);
                nodes.insert(right);

                let edge = Edge {
                    left,
                    right,
                    disable: false,
                };
                edges.push(edge);

                let edge = Edge {
                    left: right,
                    right: left,
                    disable: false,
                };
                edges.push(edge);
            }
        }

        edges.sort_unstable_by_key(|e| e.left.0);

        let mut edge_map = HashMap::new();
        let mut last = None;

        for (idx, edge) in edges.iter().enumerate() {
            if last != Some(edge.left) {
                edge_map.insert(edge.left, idx);

                last = Some(edge.left);
            }
        }

        Self {
            nodes,
            edges,
            edge_map,
        }
    }

    fn remove_connecting_edges(&mut self) {
        let mut visited = HashSet::new();
        let mut haystack = VecDeque::new();
        let mut distances = Vec::new();
        let mut checked = HashSet::new();

        let mut min = usize::MAX;
        for edge in self.edges.iter() {
            if !checked.insert(edge.cannocial()) {
                continue;
            }
            let dist = self.total_distance(*edge, &mut visited, &mut haystack, min);
            min = min.min(dist);
            distances.push((dist, edge));
        }

        let removals: Vec<_> = distances
            .iter()
            .filter(|(d, _)| *d == min)
            .map(|(_, e)| **e)
            .collect();

        for edge in self.edges.iter_mut() {
            let cannon = edge.cannocial();

            if removals.contains(&cannon) {
                edge.disable = true;
            }
        }
    }

    fn edges(&self, node: Node<'a>) -> impl Iterator<Item = Edge<'a>> + '_ {
        let mut i = self.edge_map.get(&node).copied();

        std::iter::from_fn(move || {
            if let Some(i) = i.as_mut() {
                if let Some(connection) = self.edges.get(*i) {
                    if connection.left != node {
                        None
                    } else {
                        *i += 1;

                        Some(*connection)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    fn total_distance(
        &self,
        edge: Edge<'a>,
        visited: &mut HashSet<Node<'a>>,
        haystack: &mut VecDeque<(Edge<'a>, usize)>,
        min: usize,
    ) -> usize {
        visited.clear();
        haystack.clear();
        haystack.push_back((edge, 0));
        let mut distance = 0;

        while let Some((next, dist)) = haystack.pop_front() {
            if !visited.insert(next.left) && !visited.insert(next.right) {
                continue;
            }

            distance = distance.max(dist);

            if distance > min {
                break;
            }

            haystack.extend(self.edges(next.left).map(|e| (e, dist + 1)));
            haystack.extend(self.edges(next.right).map(|e| (e, dist + 1)));
        }

        distance
    }

    fn count_cluster(&self) -> usize {
        let mut visited = HashSet::new();
        let mut haystack = Vec::new();
        let start = self.edges.iter().find(|e| !e.disable).copied().unwrap();
        haystack.push(start.left);

        while let Some(next) = haystack.pop() {
            if !visited.insert(next) {
                continue;
            }

            haystack.extend(self.edges(next).filter(|e| !e.disable).map(|e| e.right))
        }

        visited.len()
    }

    fn score(&self) -> usize {
        let cluster_size = self.count_cluster();

        (self.nodes.len() - cluster_size) * cluster_size
    }
}

pub fn part_two(_input: &str) -> &'static str {
    "Almost there..."
}

#[test]
fn test() {
    let input = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#;

    assert_eq!(54, part_one(input));
}
