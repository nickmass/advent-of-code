use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    let lines = input.as_bytes().chunks(8);

    let mut orbits = HashMap::new();
    let mut store = Vec::new();

    for line in lines {
        let parent: [u8; 3] = line[0..3].try_into().unwrap();
        let child: [u8; 3] = line[4..7].try_into().unwrap();

        let child_orbit = if let Some(&parent_orbit) = orbits.get(&parent) {
            Orbit::Child(parent_orbit)
        } else {
            let parent_orbit = store.len();
            let child = Orbit::Child(parent_orbit);
            store.push(Orbit::Root);
            orbits.insert(parent, parent_orbit);
            child
        };

        if let Some(&existing_child) = orbits.get(&child) {
            store[existing_child] = child_orbit;
        } else {
            store.push(child_orbit);
            orbits.insert(child, store.len() - 1);
        }
    }

    let mut orbit_count = 0;
    for orb in &store {
        orbit_count += match *orb {
            Orbit::Root => 0,
            Orbit::Child(parent_idx) => {
                let mut acc = 1;
                let mut next = Some(parent_idx);
                while let Some(next_idx) = next.take() {
                    match store[next_idx] {
                        Orbit::Root => next = None,
                        Orbit::Child(parent_idx) => {
                            next = Some(parent_idx);
                            acc += 1;
                        }
                    }
                }

                acc
            }
        }
    }

    orbit_count
}

pub fn part_two(input: &str) -> usize {
    let lines = input.as_bytes().chunks(8);

    let mut orbits = HashMap::new();
    let mut store = Vec::new();

    for line in lines {
        let parent: [u8; 3] = line[0..3].try_into().unwrap();
        let child: [u8; 3] = line[4..7].try_into().unwrap();

        let child_orbit = if let Some(&parent_orbit) = orbits.get(&parent) {
            Orbit::Child(parent_orbit)
        } else {
            let parent_orbit = store.len();
            let child = Orbit::Child(parent_orbit);
            store.push(Orbit::Root);
            orbits.insert(parent, parent_orbit);
            child
        };

        if let Some(&existing_child) = orbits.get(&child) {
            store[existing_child] = child_orbit;
        } else {
            store.push(child_orbit);
            orbits.insert(child, store.len() - 1);
        }
    }

    let me = orbits[b"YOU"];
    let santa = orbits[b"SAN"];

    let path_to_root = |start: usize| -> Vec<usize> {
        let mut next = start;
        let mut acc = Vec::new();
        loop {
            match store[next] {
                Orbit::Root => return acc,
                Orbit::Child(idx) => {
                    next = idx;
                    acc.push(idx);
                }
            }
        }
    };

    let my_path = path_to_root(me);
    let santas_path = path_to_root(santa);

    let mut count = 0;
    for node in my_path {
        if let Some(meeting_point) = santas_path.iter().position(|&p| p == node) {
            count += meeting_point;
            break;
        }
        count += 1;
    }

    count
}

#[derive(Debug, Clone)]
enum Orbit {
    Root,
    Child(usize),
}

#[test]
fn test() {
    let input = r#"COM)BBB
BBB)CCC
CCC)DDD
DDD)EEE
EEE)FFF
BBB)GGG
GGG)HHH
DDD)III
EEE)JJJ
JJJ)KKK
KKK)LLL
"#;

    assert_eq!(42, part_one(input));

    let input = r#"COM)BBB
BBB)CCC
CCC)DDD
DDD)EEE
EEE)FFF
BBB)GGG
GGG)HHH
DDD)III
EEE)JJJ
JJJ)KKK
KKK)LLL
KKK)YOU
III)SAN
"#;

    assert_eq!(4, part_two(input));
}
