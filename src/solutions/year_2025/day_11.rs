use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> u64 {
    let graph = Graph::new(input).expect("could not parse");
    graph.count_routes(Node("you"), Node("out"))
}

pub fn part_two(input: &str) -> u64 {
    let graph = Graph::new(input).expect("could not parse");
    graph.count_map()
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Node<'a>(&'a str);

struct Graph<'a> {
    nodes: HashMap<Node<'a>, (usize, usize)>,
    connections: Vec<Node<'a>>,
}

impl<'a> Graph<'a> {
    fn new(input: &'a str) -> Option<Self> {
        let lines = input.trim().lines();

        let mut nodes = HashMap::new();
        let mut connections = Vec::new();

        for line in lines {
            let (node, outputs) = line.split_once(":")?;
            let connection_start = connections.len();
            let mut connection_len = 0;

            for output in outputs.split_ascii_whitespace() {
                connections.push(Node(output));
                connection_len += 1;
            }

            nodes.insert(Node(node), (connection_start, connection_len));
        }

        Some(Self { nodes, connections })
    }

    fn count_routes(&self, start: Node<'_>, target: Node<'_>) -> u64 {
        let mut stack = Vec::new();
        stack.push(start);
        let mut count = 0;

        while let Some(next) = stack.pop() {
            if next == target {
                count += 1;
                continue;
            }

            if let Some((index, len)) = self.nodes.get(&next).copied() {
                stack.extend(self.connections.iter().skip(index).take(len));
            }
        }

        count
    }

    fn count_routes_with_bail(
        &self,
        start: Node<'_>,
        target: Node<'_>,
        all_targets: &[Node<'_>],
        bail: Option<&[Vec<Node<'_>>]>,
    ) -> u64 {
        let mut stack = Vec::new();
        stack.push(start);

        let mut count = 0;

        while let Some(next) = stack.pop() {
            if next == target {
                count += 1;
                continue;
            }

            if let Some((index, len)) = self.nodes.get(&next).copied() {
                let connections = self.connections.iter().skip(index).take(len);
                for connection in connections {
                    if *connection == target {
                        count += 1;
                        continue;
                    }
                    if all_targets.contains(connection) {
                        continue;
                    }

                    if bail
                        .map(|s| s.iter().any(|v| v.contains(connection)))
                        .unwrap_or(false)
                    {
                        continue;
                    }

                    stack.push(*connection);
                }
            }
        }

        count
    }

    fn count_map(&self) -> u64 {
        let map = self.create_map(
            Node("svr"),
            &[Node("svr"), Node("dac"), Node("fft"), Node("out")],
        );

        let mut counts = HashMap::new();
        counts.insert(Node("svr"), 1);
        for i in 0..map.len() - 1 {
            let starts = map[i].as_slice();
            let targets = map[i + 1].as_slice();
            let bail = map.get((i + 2)..);

            for &start in starts {
                let prev_count = counts.get(&start).copied().unwrap_or(0);
                for &target in targets {
                    let c = self.count_routes_with_bail(start, target, targets, bail);
                    *counts.entry(target).or_default() += c * prev_count;
                }
            }
        }

        counts.get(&Node("out")).copied().unwrap_or(0)
    }

    fn topo_sort(
        &self,
        start: Node<'a>,
        reverse_index: &HashMap<Node<'_>, Vec<Node<'_>>>,
    ) -> Vec<Node<'a>> {
        let mut sort = Vec::with_capacity(self.nodes.len());
        let mut stack = Vec::new();

        let mut visited_edge_count = HashMap::with_capacity(self.nodes.len());

        stack.push(start);

        while let Some(next) = stack.pop() {
            sort.push(next);
            if let Some((index, len)) = self.nodes.get(&next).copied() {
                let connections = self.connections.iter().skip(index).take(len);
                for child in connections {
                    let entry = visited_edge_count.entry(child).or_insert(0);
                    *entry += 1;

                    let Some(inbound_len) = reverse_index.get(child).map(|l| l.len()) else {
                        continue;
                    };

                    if *entry == inbound_len {
                        stack.push(*child);
                    }
                }
            }
        }

        sort
    }

    fn reverse_index(&self) -> HashMap<Node<'a>, Vec<Node<'a>>> {
        let mut map = HashMap::with_capacity(self.nodes.len());

        for (&node, &(index, len)) in self.nodes.iter() {
            let connections = self.connections.iter().skip(index).take(len);
            for &child in connections {
                let entry = map.entry(child).or_insert(Vec::new());
                entry.push(node);
            }
        }

        map
    }

    fn create_map(&self, start: Node<'a>, pois: &[Node<'a>]) -> Vec<Vec<Node<'a>>> {
        let reverse_index = self.reverse_index();
        let topo = self.topo_sort(start, &reverse_index);

        let mut depth_map = HashMap::with_capacity(self.nodes.len());
        for node in topo {
            let Some(parents) = reverse_index.get(&node) else {
                depth_map.insert(node, 1);
                continue;
            };

            let depth = parents
                .iter()
                .filter_map(|p| depth_map.get(p).copied())
                .max()
                .unwrap_or(0);
            depth_map.insert(node, depth + 1);
        }

        let mut reverse_depth = HashMap::new();
        for (node, depth) in depth_map {
            let entry = reverse_depth.entry(depth).or_insert(HashSet::new());
            entry.insert(node);
        }

        let mut reverse_depth = reverse_depth.into_iter().collect::<Vec<_>>();
        reverse_depth.sort_unstable_by_key(|e| e.0);

        let mut rough_map = Vec::new();
        let pois: HashSet<_> = pois.iter().copied().collect();
        'outer: for (depth, nodes) in reverse_depth {
            for node in nodes.intersection(&pois) {
                rough_map.push((depth, vec![*node]));
                continue 'outer;
            }

            if nodes.len() <= 5 {
                rough_map.push((depth, nodes.iter().copied().collect()));
            }
        }

        let mut map = Vec::new();

        let mut prev_depth: u32 = 0;
        while let Some((depth, nodes)) = rough_map.pop() {
            if nodes.len() == 1 {
                if nodes.iter().all(|n| pois.contains(n)) {
                    map.push(nodes);
                    prev_depth = depth;
                    continue;
                }
            }

            if depth.abs_diff(prev_depth) == 1 {
                prev_depth = depth;
                continue;
            }
            prev_depth = depth;
            map.push(nodes);
        }

        map.reverse();

        map
    }
}

#[test]
fn test() {
    let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    assert_eq!(5, part_one(input));

    let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    assert_eq!(2, part_two(input));
}
