use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut sets = HashMap::new();
    for line in lines {
        let Some((a, b)) = line.split_once('-') else {
            continue;
        };
        let a = Computer(a);
        let b = Computer(b);

        sets.entry(a).or_insert(HashSet::new()).insert(b);
        sets.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let mut groups = HashSet::new();

    for (c_a, set_a) in sets.iter() {
        for c_b in set_a.iter() {
            let set_b = sets.get(c_b).unwrap();

            for c_c in set_a.intersection(set_b) {
                let mut union = [c_a, c_b, c_c];

                if union.iter().any(|c| c.maybe_historian()) {
                    union.sort();
                    groups.insert(union);
                }
            }
        }
    }

    groups.len() as u64
}

pub fn part_two(input: &str) -> String {
    let lines = input.trim().lines();

    let mut sets = HashMap::new();
    for line in lines {
        let Some((a, b)) = line.split_once('-') else {
            continue;
        };
        let a = Computer(a);
        let b = Computer(b);

        sets.entry(a).or_insert(HashSet::new()).insert(b);
        sets.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let mut computers = Vec::with_capacity(sets.len());
    computers.extend(sets.keys());

    let mut networks: Vec<Vec<_>> = Vec::with_capacity(computers.len());
    while let Some(computer) = computers.pop() {
        for network in networks.iter_mut() {
            let mut found = true;
            for n in network.iter() {
                if !sets.get(n).unwrap().contains(computer) {
                    found = false;
                    break;
                }
            }

            if found {
                network.push(*computer);
            }
        }

        networks.push(vec![*computer]);
    }

    networks.sort_by_key(|v| v.len());

    let mut biggest = networks.into_iter().rev().next().unwrap();
    biggest.sort();
    let mut result = String::with_capacity(biggest.len() * 3);
    let mut first = true;
    for c in biggest {
        if !first {
            result.push(',');
        }
        result.push_str(c.0);
        first = false;
    }

    result
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Computer<'a>(&'a str);

impl<'a> Computer<'a> {
    fn maybe_historian(&self) -> bool {
        self.0.as_bytes()[0] == b't'
    }
}

#[test]
fn test() {
    let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;

    assert_eq!(7, part_one(input));
    assert_eq!("co,de,ka,ta", part_two(input));
}
