use std::{cmp::Reverse, collections::BinaryHeap};

use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    solve_part_one::<1000>(input)
}

fn solve_part_one<const CONNECTIONS: usize>(input: &str) -> u64 {
    let points: Vec<_> = input.trim().lines().filter_map(Point::parse).collect();
    let mut distances = order_by_distance(&points);

    let mut groups = Vec::new();
    let mut connected = HashMap::new();
    let mut count = 0;

    while let Some((_, a, b)) = distances.pop() {
        if count >= CONNECTIONS {
            break;
        }
        count += 1;

        let has_a = connected.get(&a).copied();
        let has_b = connected.get(&b).copied();

        match (has_a, has_b) {
            (None, None) => {
                connected.insert(a, groups.len());
                connected.insert(b, groups.len());
                groups.push(2);
            }
            (Some(a_id), Some(b_id)) if a_id == b_id => (),
            (Some(a_id), Some(b_id)) => {
                for current_id in connected.values_mut() {
                    if *current_id == a_id {
                        *current_id = b_id;
                        groups[b_id] += 1;
                    }
                }
                groups[a_id] = 0;
            }
            (Some(a_id), None) => {
                connected.insert(b, a_id);
                groups[a_id] += 1;
            }
            (None, Some(b_id)) => {
                connected.insert(a, b_id);
                groups[b_id] += 1;
            }
        }
    }

    groups.sort_unstable_by_key(|&g| Reverse(g));
    groups.into_iter().take(3).filter(|&g| g != 0).product()
}

pub fn part_two(input: &str) -> u64 {
    let points: Vec<_> = input.trim().lines().filter_map(Point::parse).collect();
    let mut distances = order_by_distance(&points);

    let mut group_id = 0;
    let mut empty_groups = 0;
    let mut connected = HashMap::new();

    while let Some((_, a, b)) = distances.pop() {
        let has_a = connected.get(&a).copied();
        let has_b = connected.get(&b).copied();

        match (has_a, has_b) {
            (None, None) => {
                connected.insert(a, group_id);
                connected.insert(b, group_id);
                group_id += 1;
            }
            (Some(a_id), Some(b_id)) if a_id == b_id => (),
            (Some(a_id), Some(b_id)) => {
                empty_groups += 1;
                for current_id in connected.values_mut() {
                    if *current_id == a_id {
                        *current_id = b_id;
                    }
                }
            }
            (Some(a_id), None) => {
                connected.insert(b, a_id);
            }
            (None, Some(b_id)) => {
                connected.insert(a, b_id);
            }
        }

        if group_id - empty_groups == 1 && connected.len() == points.len() {
            return points[a as usize].0 * points[b as usize].0;
        }
    }

    0
}

fn order_by_distance(points: &[Point]) -> BinaryHeap<(Reverse<u64>, u32, u32)> {
    let mut distances = BinaryHeap::with_capacity(points.len() * points.len() / 2);

    for (a_idx, &a) in points.iter().enumerate() {
        for (b_idx, &b) in points.iter().enumerate().skip(a_idx + 1) {
            let dist = a.distance(b);

            distances.push((Reverse(dist), a_idx as u32, b_idx as u32));
        }
    }

    distances
}

#[derive(Debug, Copy, Clone)]
struct Point(u64, u64, u64);

impl Point {
    fn parse(line: &str) -> Option<Self> {
        let mut parts = [0; 3];

        let mut idx = 0;
        for &b in line.as_bytes() {
            match b {
                b',' => {
                    idx += 1;
                }
                b'0'..=b'9' => {
                    parts[idx] *= 10;
                    parts[idx] += (b - b'0') as u64;
                }
                _ => return None,
            }
        }

        Some(Self(parts[0], parts[1], parts[2]))
    }

    fn distance(self, Point(x2, y2, z2): Self) -> u64 {
        let Point(x1, y1, z1) = self;
        (x1.abs_diff(x2)).pow(2) + (y1.abs_diff(y2)).pow(2) + (z1.abs_diff(z2)).pow(2)
    }
}

#[test]
fn test() {
    let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    assert_eq!(40, solve_part_one::<10>(input));
    assert_eq!(25272, part_two(input));
}
