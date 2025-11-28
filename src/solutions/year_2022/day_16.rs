use crate::{HashMap, HashSet};

use std::collections::VecDeque;

pub fn part_one(input: &str) -> u64 {
    let mut graph = Graph::new(input);
    graph.optimize_routes();

    graph.solve().unwrap_or(0)
}

pub fn part_two(input: &str) -> u64 {
    let mut graph = Graph::new(input);
    graph.optimize_routes();

    graph.solve_with_partner().unwrap_or(0)
}

type NodeId = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ConnectionId(NodeId, NodeId);

impl ConnectionId {
    fn new(left: NodeId, right: NodeId) -> Self {
        Self(left.min(right), left.max(right))
    }

    fn other(&self, me: NodeId) -> NodeId {
        if self.0 == me {
            self.1
        } else {
            self.0
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Node<'a> {
    id: NodeId,
    rate: u64,
    name: &'a str,
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    cost: u64,
}

#[derive(Debug)]
struct Graph<'a> {
    names: HashMap<&'a str, NodeId>,
    nodes: Vec<Node<'a>>,
    routes: HashMap<ConnectionId, Edge>,
    connections: HashMap<NodeId, Vec<ConnectionId>>,
    good_nodes: HashSet<NodeId>,
    min_routes: HashMap<NodeId, u64>,
}

impl<'a> Graph<'a> {
    fn new(input: &'a str) -> Self {
        let entrys: Vec<_> = input.lines().filter_map(Entry::parse).collect();

        let mut nodes = Vec::new();
        let mut names = HashMap::new();
        for entry in entrys.iter() {
            let id = nodes.len();
            let node = Node {
                id,
                rate: entry.rate,
                name: entry.name,
            };

            nodes.push(node);
            names.insert(entry.name, id);
        }

        let mut connections = HashMap::new();
        let mut routes = HashMap::new();

        for entry in entrys.iter() {
            let Some(me) = names.get(entry.name).and_then(|&id| nodes.get(id)) else {
                continue;
            };

            let mut my_connections = Vec::new();
            for neighbor in entry.neighbors.iter() {
                let Some(them) = names.get(neighbor).and_then(|&id| nodes.get(id)) else {
                    continue;
                };

                let connection_id = ConnectionId::new(me.id, them.id);

                my_connections.push(connection_id);

                if me.id > them.id {
                    continue;
                }

                let edge = Edge { cost: 1 };

                routes.insert(connection_id, edge);
            }

            connections.insert(me.id, my_connections);
        }

        let good_nodes = nodes.iter().filter(|n| n.rate > 0).map(|n| n.id).collect();

        Self {
            names,
            nodes,
            routes,
            connections,
            good_nodes,
            min_routes: HashMap::new(),
        }
    }

    fn optimize_routes(&mut self) {
        let mut new_routes = HashMap::new();
        let mut min_routes = HashMap::new();

        for left in self.nodes.iter() {
            if left.rate == 0 && left.name != "AA" {
                continue;
            }
            let start = left.id + 1;
            for right in self.nodes[start..].iter() {
                if right.rate == 0 && right.name != "AA" {
                    continue;
                }
                let cost = self.shortest_path(left.id, right.id);

                let connection_id = ConnectionId::new(left.id, right.id);
                let edge = Edge { cost };
                new_routes.insert(connection_id, edge);

                min_routes
                    .entry(left.id)
                    .and_modify(|v: &mut u64| {
                        *v = cost.min(*v);
                    })
                    .or_insert(cost);

                min_routes
                    .entry(right.id)
                    .and_modify(|v: &mut u64| {
                        *v = cost.min(*v);
                    })
                    .or_insert(cost);
            }
        }

        self.min_routes = min_routes;
        self.routes = new_routes;
    }

    fn shortest_path(&self, start: NodeId, end: NodeId) -> u64 {
        let mut haystack = VecDeque::new();
        let mut visited = HashSet::new();

        haystack.extend(self.neighbors(start).map(|n| (n, 1)));

        while let Some((next, cost)) = haystack.pop_front() {
            if next == end {
                return cost;
            }

            for neighbor in self.neighbors(next) {
                if visited.contains(&neighbor) {
                    continue;
                }

                visited.insert(neighbor);
                haystack.push_back((neighbor, cost + 1));
            }
        }

        u64::MAX
    }

    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.connections
            .get(&node)
            .unwrap()
            .iter()
            .map(move |c| c.other(node))
    }

    fn solve(&self) -> Option<u64> {
        let walker = Walker::new(self)?;
        let mut walkers = vec![walker];

        let mut max_pressure = 0;

        while let Some(walker) = walkers.pop() {
            if walker.pressure > max_pressure {
                max_pressure = walker.pressure;
            }

            walkers.extend(walker.remaining_valves().filter_map(|n| walker.visit(n)));
        }

        Some(max_pressure)
    }

    fn solve_with_partner(&self) -> Option<u64> {
        let walker = PairWalker::new(self)?;
        let mut walkers = vec![walker];

        let mut max_pressure = 0;

        while let Some(walker) = walkers.pop() {
            if walker.pressure > max_pressure {
                max_pressure = walker.pressure;
            }

            walkers.extend(
                walker
                    .moves()
                    .filter_map(|n| walker.visit(n))
                    .filter(|w| !w.impossible(max_pressure)),
            );
        }

        Some(max_pressure)
    }
}

#[derive(Debug, Clone)]
struct Walker<'a> {
    time_remaining: u64,
    pressure: u64,
    remaining_nodes: HashSet<NodeId>,
    current: NodeId,
    graph: &'a Graph<'a>,
}

impl<'a> Walker<'a> {
    fn new(graph: &'a Graph<'a>) -> Option<Self> {
        Some(Walker {
            graph,
            time_remaining: 30,
            pressure: 0,
            remaining_nodes: graph.good_nodes.clone(),
            current: graph.names.get("AA").copied()?,
        })
    }

    fn visit(&self, node: NodeId) -> Option<Self> {
        let connection = ConnectionId::new(self.current, node);

        let cost = self.graph.routes.get(&connection)?.cost;

        if cost + 1 > self.time_remaining {
            return None;
        }

        let mut new_walker = Walker {
            time_remaining: self.time_remaining - cost,
            pressure: self.pressure,
            current: node,
            graph: self.graph,
            remaining_nodes: self.remaining_nodes.clone(),
        };

        new_walker.activate();

        Some(new_walker)
    }

    fn remaining_valves(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.remaining_nodes.iter().copied()
    }

    fn activate(&mut self) {
        self.remaining_nodes.remove(&self.current);
        let rate = self.node().rate;

        if rate > 0 {
            self.time_remaining -= 1;
            self.pressure += rate * self.time_remaining;
        }
    }

    fn node(&self) -> &Node<'a> {
        self.graph.nodes.get(self.current).unwrap()
    }
}

#[derive(Debug, Clone)]
struct PairWalker<'a> {
    time_limit: u64,
    pressure: u64,
    remaining_nodes: HashSet<NodeId>,
    current: [(NodeId, u64); 2],
    graph: &'a Graph<'a>,
}

impl<'a> PairWalker<'a> {
    fn new(graph: &'a Graph<'a>) -> Option<Self> {
        let start = graph.names.get("AA").copied()?;
        Some(PairWalker {
            graph,
            time_limit: 26,
            pressure: 0,
            remaining_nodes: graph.good_nodes.clone(),
            current: [(start, 0), (start, 0)],
        })
    }

    fn visit(&self, node: Move) -> Option<Self> {
        let (idx, node_id) = match node {
            Move::Me(node) => (0, node),
            Move::Elephant(node) => (1, node),
        };

        let connection = ConnectionId::new(self.current[idx].0, node_id);

        let cost = self.graph.routes.get(&connection)?.cost + self.current[idx].1 + 1;

        if cost > self.time_limit {
            return None;
        }

        let mut new_walker = self.clone();

        new_walker.current[idx] = (node_id, cost);
        new_walker.remaining_nodes.remove(&node_id);

        let node = self.graph.nodes.get(node_id)?;
        new_walker.pressure += node.rate * (self.time_limit - cost);

        Some(new_walker)
    }

    fn impossible(&self, max: u64) -> bool {
        use std::collections::BinaryHeap;
        let mut a = self.current[0].1;
        let mut b = self.current[1].1;

        let mut nodes = self
            .remaining_nodes
            .iter()
            .filter_map(|n| self.graph.nodes.get(*n))
            .map(|n| (n.rate, n.id))
            .collect::<BinaryHeap<_>>();

        let mut sum = self.pressure;

        // Popping from a sorted heap simulates the optimal order of visiting the largest nodes first
        while let Some((next, id)) = nodes.pop() {
            let cost = self.graph.min_routes.get(&id).copied().unwrap_or(1);
            if a < b {
                // shortest edge for travel time and 1 minute to activate
                a += cost + 1;
            } else {
                b += cost + 1;
            }

            // Use the partner with the most time remaining to collect maximum pressure
            let min = a.min(b);
            let time = self.time_limit.saturating_sub(min);

            sum += next * time;
        }

        // If this perfect situation is still less then our previous best,
        // then it must be impossible to beat our highscore from this path
        sum < max
    }

    fn moves(&self) -> impl Iterator<Item = Move> + '_ {
        let me = self
            .remaining_nodes
            .iter()
            .copied()
            .map(Move::Me)
            .filter(|_| self.current[0].1 <= self.current[1].1);
        let elephant = self
            .remaining_nodes
            .iter()
            .copied()
            .map(Move::Elephant)
            .filter(|_| self.current[1].1 <= self.current[0].1);

        me.chain(elephant)
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Me(NodeId),
    Elephant(NodeId),
}

#[derive(Debug)]
struct Entry<'a> {
    name: &'a str,
    rate: u64,
    neighbors: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    fn parse(line: &'a str) -> Option<Self> {
        let line = line.trim_start_matches("Valve ");
        let (name, line) = line.split_once(' ')?;
        let line = line.trim_start_matches("has flow rate=");
        let (rate, line) = line.split_once(';')?;
        let rate = rate.parse().ok()?;
        let line = line.trim_start_matches(" tunnels lead to valves ");
        let line = line.trim_start_matches(" tunnel leads to valve ");
        let neighbors = line.split(",").map(|n| n.trim()).collect();

        Some(Self {
            name,
            rate,
            neighbors,
        })
    }
}

#[test]
fn test() {
    let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    assert_eq!(1651, part_one(input));
    assert_eq!(1707, part_two(input));
}
