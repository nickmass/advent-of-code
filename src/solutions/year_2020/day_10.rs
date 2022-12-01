use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> u32 {
    let mut adapters: HashSet<u32> = input.lines().filter_map(|l| l.parse().ok()).collect();
    let built_in = adapters.iter().copied().max().unwrap_or(0) + 3;

    adapters.insert(built_in);

    let mut n = 1;
    let mut last_n = 0;
    let mut diffs = HashMap::new();
    while adapters.len() != 0 {
        if adapters.contains(&n) {
            diffs.entry(n - last_n).and_modify(|c| *c += 1).or_insert(1);
            last_n = n;
            adapters.remove(&n);
            n += 1;
        } else {
            n += 1;
            if n - last_n > 3 {
                panic!("wrong input");
            }
        }
    }

    let ones = diffs.get(&1).copied().unwrap_or(0);
    let threes = diffs.get(&3).copied().unwrap_or(0);

    ones * threes
}

pub fn part_two(input: &str) -> u64 {
    let mut adapters: std::collections::BTreeSet<u32> =
        input.lines().filter_map(|l| l.parse().ok()).collect();
    let built_in = adapters.iter().copied().last().unwrap_or(0) + 3;
    adapters.insert(built_in);

    let mut nodes = NodeCollection::with_capacity(adapters.len());
    let mut paths = 0;
    for n in adapters.range(1..=built_in) {
        paths = nodes.insert(*n);
    }

    paths
}

#[derive(Debug, Copy, Clone)]
struct Node {
    paths_to_root: u64,
}

struct NodeCollection {
    map: HashMap<u32, Node>,
}

impl NodeCollection {
    fn with_capacity(capacity: usize) -> Self {
        let root_node = Node { paths_to_root: 1 };
        let mut map = HashMap::with_capacity(capacity);
        map.insert(0, root_node);

        Self { map }
    }

    fn insert(&mut self, value: u32) -> u64 {
        let mut paths_to_root = 0;

        for n in 1..=3.min(value) {
            let parent_value = value - n;
            if let Some(parent) = self.map.get(&parent_value) {
                paths_to_root += parent.paths_to_root;
            }
        }

        if paths_to_root != 0 {
            let node = Node { paths_to_root };
            self.map.insert(value, node);
        }

        paths_to_root
    }
}

#[test]
fn test() {
    let run_a = |inp, res| assert_eq!(part_one(inp), res);
    let run_b = |inp, res| assert_eq!(part_two(inp), res);

    let inp = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    run_a(inp, 7 * 5);
    run_b(inp, 8);

    let inp = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    run_a(inp, 22 * 10);
    run_b(inp, 19208);
}
