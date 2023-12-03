use crate::{HashMap, HashSet};

use super::{Gcd, Point2};

pub fn part_one(input: &str) -> DayTenResult {
    let grid = AsteroidGrid::new(input);

    let mut max_roids = 0;
    let mut target = Point2::new(0, 0);
    for &p in grid.occupied.iter() {
        let roids = grid.roid_on_angle(p);
        if roids > max_roids {
            target = p;
            max_roids = roids;
        }
    }

    DayTenResult { max_roids, target }
}

pub fn part_two(input: &str) -> i32 {
    let DayTenResult { target, .. } = part_one(input);
    let mut grid = AsteroidGrid::new(input);

    grid.angles
        .sort_unstable_by(|(_, a1), (_, a2)| a1.partial_cmp(a2).unwrap());

    let mut loop_count = 1;
    let mut remove_count = 0;
    let angles = grid.angles.len();
    while loop_count < grid.height * grid.width {
        for idx in 0..angles {
            let (pp, _) = grid.angles[idx];
            let mut target = target + (pp * loop_count);

            while grid.in_bounds(target) {
                if grid.remove(target) {
                    remove_count += 1;
                    if remove_count == 200 {
                        return target.x * 100 + target.y;
                    }
                    break;
                }

                target += pp;
            }
        }
        loop_count += 1;
    }

    panic!("Missed it");
}

#[derive(Debug)]
struct AsteroidGrid {
    occupied: HashSet<Point2<i32>>,
    angles: Vec<(Point2<i32>, f32)>,
    width: i32,
    height: i32,
}

impl AsteroidGrid {
    fn new<S: AsRef<str>>(input: S) -> Self {
        let input = input.as_ref();
        let lines = input.trim().split('\n').map(str::trim).enumerate();

        let mut height = 0;
        let mut width = 0;

        let mut occupied = HashSet::new();

        for (y, row) in lines {
            width = row.len() as i32;
            for (x, cell) in row.chars().enumerate() {
                if cell == '#' {
                    occupied.insert(Point2::new(x as i32, y as i32));
                }
            }
            height += 1;
        }

        let angles = create_angle_grid(width, height);

        AsteroidGrid {
            width,
            height,
            occupied,
            angles,
        }
    }

    fn in_bounds(&self, p: Point2<i32>) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width && p.y < self.height
    }

    fn contains(&self, p: Point2<i32>) -> bool {
        self.occupied.contains(&p)
    }

    fn remove(&mut self, p: Point2<i32>) -> bool {
        self.occupied.remove(&p)
    }

    fn roid_on_angle(&self, p: Point2<i32>) -> usize {
        let mut count = 0;
        'outer: for &(pp, _) in &self.angles {
            let mut p_acc = pp + p;
            while self.in_bounds(p_acc) {
                if self.contains(p_acc) {
                    count += 1;
                    continue 'outer;
                }
                p_acc += pp;
            }
        }

        count
    }
}

fn create_angle_grid(width: i32, height: i32) -> Vec<(Point2<i32>, f32)> {
    use std::f32::consts::PI;
    let mut results = HashMap::new();

    for y in 1..=height {
        for x in 1..=width {
            let angle = PI - (x as f32 / y as f32).atan();
            let gcd = x.gcd(y);
            results.insert(Point2::new(x / gcd, y / gcd), angle);
        }
    }

    let mut corners: Vec<_> = results.into_iter().collect();
    let corner_elements = corners.len();
    corners.reserve(corner_elements * 3);

    #[derive(Clone, Copy)]
    enum Corner {
        TopRight,
        BottomLeft,
        TopLeft,
    }

    for &(p, corner) in &[
        (Point2::new(1, -1), Corner::TopRight),
        (Point2::new(-1, 1), Corner::BottomLeft),
        (Point2::new(-1, -1), Corner::TopLeft),
    ] {
        for idx in 0..corner_elements {
            let (pp, angle) = corners[idx];
            let angle = PI - angle;
            let angle = match corner {
                Corner::TopRight => angle,
                Corner::BottomLeft => PI + angle,
                Corner::TopLeft => (PI * 2.0) - angle,
            };
            corners.push((p * pp, angle));
        }
    }

    let corner_offset = PI / 2.0;
    corners.push((Point2::new(0, -1), 0.0 * corner_offset));
    corners.push((Point2::new(1, 0), 1.0 * corner_offset));
    corners.push((Point2::new(0, 1), 2.0 * corner_offset));
    corners.push((Point2::new(-1, 0), 3.0 * corner_offset));

    corners
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayTenResult {
    max_roids: usize,
    target: Point2<i32>,
}

impl std::fmt::Display for DayTenResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.max_roids)
    }
}

#[test]
fn test() {
    let input = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"#;

    let result = DayTenResult {
        max_roids: 210,
        target: Point2::new(11, 13),
    };
    assert_eq!(result, part_one(input));
    assert_eq!(802, part_two(input));
}
