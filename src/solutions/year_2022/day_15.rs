use crate::HashSet;

pub fn part_one(input: &str) -> i32 {
    solve_part_one::<2000000>(input)
}

fn solve_part_one<const Y: i32>(input: &str) -> i32 {
    let sensors = input
        .trim()
        .lines()
        .map(parse)
        .filter(|s| s.near_y(Y))
        .collect::<Vec<_>>();

    let mut ranges = sensors
        .iter()
        .filter_map(|s| s.range_on_y(Y))
        .collect::<Vec<_>>();

    ranges.sort_by_key(|&(s, _e)| s);

    let (mut count, _) = ranges
        .into_iter()
        .fold((0, i32::MIN), |(acc, prev), (start, end)| {
            if start > prev {
                (acc + (end - start) + 1, end)
            } else if end > prev {
                (acc + (end - prev), end)
            } else {
                (acc, prev)
            }
        });

    let target_beacons = sensors
        .into_iter()
        .filter_map(|s| (s.beacon.1 == Y).then_some(()))
        .collect::<HashSet<_>>();

    count -= target_beacons.len() as i32;

    count
}

pub fn part_two(input: &str) -> u64 {
    solve_part_two::<4000000>(input)
}

fn solve_part_two<const MAX: i32>(input: &str) -> u64 {
    let sensors = input.trim().lines().map(parse).collect::<Vec<_>>();

    for sensor in sensors.iter() {
        for (x, y) in sensor
            .border()
            .filter(|&(x, y)| x >= 0 && x <= MAX && y >= 0 && y <= MAX)
        {
            if sensors.iter().all(|s| !s.is_covering(x, y)) {
                return x as u64 * 4000000 + y as u64;
            }
        }
    }

    0
}

fn parse(line: &str) -> Sensor {
    let mut numbers = line
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|p| !p.is_empty())
        .filter_map(|p| p.parse::<i32>().ok());

    let pos = (numbers.next().unwrap(), numbers.next().unwrap());
    let beacon = (numbers.next().unwrap(), numbers.next().unwrap());

    Sensor {
        pos,
        beacon,
        distance: distance(pos, beacon),
    }
}

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    distance: i32,
}

impl Sensor {
    fn is_covering(&self, x: i32, y: i32) -> bool {
        distance(self.pos, (x, y)) <= self.distance
    }

    fn near_y(&self, y: i32) -> bool {
        distance(self.pos, (self.pos.0, y)) <= self.distance
    }

    fn border(&self) -> BorderIter {
        BorderIter::new(self.pos, self.distance + 1)
    }

    fn range_on_y(&self, y: i32) -> Option<(i32, i32)> {
        let diff = self.pos.1.abs_diff(y) as i32;
        if diff > self.distance {
            return None;
        }

        let from_edge = self.distance - diff;
        let row_count = from_edge * 2 + 1;

        let half = (row_count - 1) / 2;

        let start = self.pos.0 - half;
        let end = self.pos.0 + half;

        Some((start, end))
    }
}

fn distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1.abs_diff(x2) + y1.abs_diff(y2)) as i32
}

struct BorderIter {
    center: (i32, i32),
    cursor: (i32, i32),
    start: (i32, i32),
    quad: Quadrant,
}

impl BorderIter {
    fn new(center: (i32, i32), radius: i32) -> Self {
        let start = (center.0 - 1, center.1 - radius + 1);

        Self {
            center,
            cursor: start,
            start,
            quad: Quadrant::NW,
        }
    }
}

impl Iterator for BorderIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.quad {
            Quadrant::NW => {
                self.cursor.0 += 1;
                self.cursor.1 -= 1;
                if self.center.0 == self.cursor.0 {
                    self.quad = Quadrant::NE;
                }
            }
            Quadrant::NE => {
                self.cursor.0 += 1;
                self.cursor.1 += 1;
                if self.center.1 == self.cursor.1 {
                    self.quad = Quadrant::SE;
                }
            }
            Quadrant::SE => {
                self.cursor.0 -= 1;
                self.cursor.1 += 1;
                if self.center.0 == self.cursor.0 {
                    self.quad = Quadrant::SW;
                }
            }
            Quadrant::SW => {
                self.cursor.0 -= 1;
                self.cursor.1 -= 1;
                if self.center.1 == self.cursor.1 {
                    self.quad = Quadrant::NW;
                }
            }
        }

        if self.cursor == self.start {
            None
        } else {
            Some(self.cursor)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Quadrant {
    NE,
    SE,
    SW,
    NW,
}

#[test]
fn test() {
    let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    assert_eq!(26, solve_part_one::<10>(input));
    assert_eq!(56000011, solve_part_two::<20>(input));
}
