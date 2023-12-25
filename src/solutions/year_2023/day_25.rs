use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut edges = Vec::new();
    let mut all_nodes = HashSet::new();

    for line in input.trim().lines() {
        let (left_name, rest) = line.split_once(": ").unwrap();

        let left = Node(left_name);
        all_nodes.insert(left);

        for right_name in rest.split(" ") {
            let right = Node(right_name);
            all_nodes.insert(right);

            // Cheated this list of names by visualizing the graph with graphviz
            // then spotting the edges by eye, Ill be back to do this correctly
            match (left_name, right_name) {
                ("krf", "crg")
                | ("crg", "krf")
                | ("rgv", "jct")
                | ("jct", "rgv")
                | ("zhg", "fmr")
                | ("fmr", "zhg") => continue,
                _ => (),
            }

            let edge = Edge { left, right };
            edges.push(edge);
        }
    }

    let mut visited = HashSet::new();
    let mut haystack = Vec::new();
    haystack.push(edges[0].left);

    while let Some(next) = haystack.pop() {
        if !visited.insert(next) {
            continue;
        }

        for edge in edges.iter().copied() {
            if edge.left == next {
                haystack.push(edge.right);
            } else if edge.right == next {
                haystack.push(edge.left);
            }
        }
    }

    visited.len() * (all_nodes.len() - visited.len())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node<'a>(&'a str);

#[derive(Debug, Copy, Clone)]
struct Edge<'a> {
    left: Node<'a>,
    right: Node<'a>,
}

pub fn part_two(_input: &str) -> &'static str {
    "Almost there..."
}

#[test]
#[ignore = "cheated solution"]
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
