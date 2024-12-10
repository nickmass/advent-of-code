use super::{Gcd, Point3};
pub fn part_one(input: &str) -> i64 {
    solve_part_one::<1000>(input)
}

pub fn solve_part_one<const N: usize>(input: &str) -> i64 {
    let mut points: Vec<_> = input
        .trim()
        .split('\n')
        .map(|l| {
            let mut s = String::new();
            let mut n = Vec::new();
            for c in l.chars() {
                if c.is_ascii_digit() || c == '-' {
                    s.push(c);
                } else if !s.is_empty() {
                    n.push(s.parse::<i64>().unwrap());
                    s.clear();
                }
            }
            Point3::new(n[0], n[1], n[2])
        })
        .collect();

    let mut velo: Vec<_> = (0..points.len()).map(|_| Point3::new(0, 0, 0)).collect();
    for _ in 0..N {
        for (i, (&p, vel)) in points.iter().zip(velo.iter_mut()).enumerate() {
            for (j, pp) in points.iter().enumerate() {
                if i == j {
                    continue;
                }

                vel.x -= match p.x.cmp(&pp.x) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
                vel.y -= match p.y.cmp(&pp.y) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
                vel.z -= match p.z.cmp(&pp.z) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
            }
        }

        for (p, &v) in points.iter_mut().zip(velo.iter()) {
            *p += v;
        }
    }

    points
        .iter()
        .map(|p| p.x.abs() + p.y.abs() + p.z.abs())
        .zip(velo.iter().map(|v| v.x.abs() + v.y.abs() + v.z.abs()))
        .map(|(p, v)| p * v)
        .sum::<i64>()
}

pub fn part_two(input: &str) -> usize {
    let points: Vec<_> = input
        .trim()
        .split('\n')
        .map(|l| {
            let mut s = String::new();
            let mut n = Vec::new();
            for c in l.chars() {
                if c.is_ascii_digit() || c == '-' {
                    s.push(c);
                } else if !s.is_empty() {
                    n.push(s.parse::<i64>().unwrap());
                    s.clear();
                }
            }
            Point3::new(n[0], n[1], n[2])
        })
        .collect();

    let x_coords: Vec<_> = points.iter().map(|&p| p.x).collect();
    let y_coords: Vec<_> = points.iter().map(|&p| p.y).collect();
    let z_coords: Vec<_> = points.iter().map(|&p| p.z).collect();

    let x_handle = std::thread::spawn(move || sequence_length(x_coords.as_slice()));
    let y_handle = std::thread::spawn(move || sequence_length(y_coords.as_slice()));
    let z_handle = std::thread::spawn(move || sequence_length(z_coords.as_slice()));

    let x_length = x_handle.join().unwrap();
    let y_length = y_handle.join().unwrap();
    let z_length = z_handle.join().unwrap();

    let xy_gcd = x_length.gcd(y_length);
    let xy_lcm = (x_length * y_length) / xy_gcd;
    let xyz_gcd = xy_lcm.gcd(z_length);
    (xy_lcm * z_length) / xyz_gcd
}

fn sequence_length(initial_points: &[i64]) -> usize {
    let mut points = initial_points.to_vec();
    let mut velo: Vec<_> = points.iter().map(|_| 0).collect();

    let mut count = 0;

    let mut done = false;
    while !done {
        for (i, (&p, vel)) in points.iter().zip(velo.iter_mut()).enumerate() {
            for (j, pp) in points.iter().enumerate() {
                if i == j {
                    continue;
                }

                *vel -= match p.cmp(pp) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
            }
        }

        done = true;
        for (p, &v) in points.iter_mut().zip(velo.iter()) {
            *p += v;
            done = done && v == 0
        }
        count += 1;
    }

    count * 2
}

#[test]
fn test() {
    let input = r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
"#;

    assert_eq!(1940, solve_part_one::<100>(input));
    assert_eq!(4686774924, part_two(input));
}
