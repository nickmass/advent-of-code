use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    let graph = Graph::new(input).expect("could not parse");
    graph.count_routes(Node("you"), Node("out"))
}

pub fn part_two(input: &str) -> u64 {
    let graph = Graph::new(input).expect("could not parse");
    graph.count_map()
}

#[allow(unused)]
fn part_two_no_cheats(input: &str) -> u64 {
    // I "cheated" in my solution by tailoring it to the choke points in my specific input
    let graph = Graph::new(input).expect("could not parse");
    graph.count_checkpoint_routes(Node("svr"), [Node("dac"), Node("fft")], Node("out"))
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
        bail: Option<&[Node<'_>]>,
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
                    if bail.map(|b| b.contains(connection)).unwrap_or(false) {
                        continue;
                    }

                    stack.push(*connection);
                }
            }
        }

        count
    }

    fn count_checkpoint_routes(
        &self,
        start: Node<'_>,
        [check0, check1]: [Node<'a>; 2],
        target: Node<'_>,
    ) -> u64 {
        let a = self.count_path(start, check0, check1, target);
        let b = self.count_path(start, check1, check0, target);

        a + b
    }

    fn count_path(
        &self,
        start: Node<'a>,
        check0: Node<'a>,
        check1: Node<'a>,
        target: Node<'a>,
    ) -> u64 {
        let b = self.count_routes(check0, check1);

        if b == 0 {
            return 0;
        }

        let a = self.count_routes(start, check0);

        if a == 0 {
            return 0;
        }

        let c = self.count_routes(check1, target);

        a * b * c
    }

    fn count_map(&self) -> u64 {
        let map = Self::map();
        let mut counts = HashMap::new();
        counts.insert(Node("svr"), 1);
        for i in 0..map.len() - 1 {
            let starts = map[i];
            let targets = map[i + 1];
            let bail = map.get(i + 2).copied();

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

    #[rustfmt::skip]
    fn map() -> &'static [&'static [Node<'static>]] {
        // Created by plotting my input with graphviz, lists the sequence of "narrow" parts of the graph
        &[
            &[Node("svr")],
            &[Node("muy"), Node("zyi"), Node("pzi")],
            &[Node("fft")],
            &[Node("kqn"), Node("vjh"), Node("vht"), Node("edr"), Node("ehw")],
            &[Node("apc"), Node("lpz"), Node("rpn")],
            &[Node("xct"), Node("cix"), Node("jvl"), Node("jyw"), Node("tql")],
            &[Node("dac")],
            &[Node("you"), Node("ire"), Node("sdo"), Node("qdo")],
            &[Node("out")],
        ]
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

    assert_eq!(2, part_two_no_cheats(input));
}
