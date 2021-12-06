use crate::HashSet;

// TODO - Delete all this

#[derive(Debug, Copy, Clone)]
enum Segment {
    Horizontal { row: i64, start: i64, end: i64 },
    Vertical { column: i64, start: i64, end: i64 },
    Diagonal { x0: i64, y0: i64, x1: i64, y1: i64 },
}

impl Segment {
    fn length(&self) -> i64 {
        match self {
            Segment::Horizontal { start, end, .. } => (end - start).abs(),
            Segment::Vertical { start, end, .. } => (end - start).abs(),
            Segment::Diagonal { x0, x1, .. } => (x1 - x0).abs(),
        }
    }

    fn intersect(&self, other: &Self, matches: &mut HashSet<(i64, i64)>) {
        use Segment::*;
        match (*self, *other) {
            (Horizontal { row: a_r, .. }, Horizontal { row: b_r, .. }) if a_r == b_r => {
                if self.length() < other.length() {
                    for a in self.iter() {
                        if other.contains(a) {
                            matches.insert(a);
                        }
                    }
                } else {
                    for a in other.iter() {
                        if self.contains(a) {
                            matches.insert(a);
                        }
                    }
                }
            }
            (Vertical { column: a_c, .. }, Vertical { column: b_c, .. }) if a_c == b_c => {
                if self.length() < other.length() {
                    for a in self.iter() {
                        if other.contains(a) {
                            matches.insert(a);
                        }
                    }
                } else {
                    for a in other.iter() {
                        if self.contains(a) {
                            matches.insert(a);
                        }
                    }
                }
            }
            (
                Horizontal {
                    row,
                    start: h_start,
                    end: h_end,
                },
                Vertical {
                    column,
                    start: v_start,
                    end: v_end,
                },
            )
            | (
                Vertical {
                    column,
                    start: v_start,
                    end: v_end,
                },
                Horizontal {
                    row,
                    start: h_start,
                    end: h_end,
                },
            ) => {
                if column >= h_start && column <= h_end && row >= v_start && row <= v_end {
                    matches.insert((column, row));
                }
            }
            (Diagonal { y0, y1, x0, x1 }, Horizontal { row, start, end })
            | (Horizontal { row, start, end }, Diagonal { y0, y1, x0, x1 }) => {
                if ((y1 > y0) && (row >= y0 && row <= y1))
                    || ((y0 > y1) && (row >= y1 && row <= y0))
                {
                    let x = if y1 > y0 {
                        (row - y0) + x0
                    } else {
                        x1 - (row - y1)
                    };

                    if x >= start && x <= end {
                        matches.insert((x, row));
                    }
                }
            }
            (Diagonal { x0, x1, y0, y1 }, Vertical { column, start, end })
            | (Vertical { column, start, end }, Diagonal { x0, x1, y0, y1 }) => {
                if column >= x0 && column <= x1 {
                    let y = if y1 > y0 {
                        (column - x0) + y0
                    } else {
                        y0 - (column - x0)
                    };

                    if y >= start && y <= end {
                        matches.insert((column, y));
                    }
                }
            }
            (
                Diagonal {
                    x0: ax0,
                    y0: ay0,
                    x1: ax1,
                    y1: ay1,
                },
                Diagonal {
                    x0: bx0,
                    y0: by0,
                    x1: bx1,
                    y1: by1,
                },
            ) => match (ay0 > ay1, by0 > by1) {
                (true, true) | (false, false) => {
                    if (ay1 - by1).abs() == (ax1 - bx1).abs()
                        && (ax0 - bx0).abs() == (ay0 - by0).abs()
                    {
                        for a in self.iter() {
                            if other.contains(a) {
                                matches.insert(a);
                            }
                        }
                    }
                }
                (true, false) | (false, true) => {
                    if self.length() < other.length() {
                        for a in self.iter() {
                            if other.contains(a) {
                                matches.insert(a);
                                break;
                            }
                        }
                    } else {
                        for a in other.iter() {
                            if self.contains(a) {
                                matches.insert(a);
                                break;
                            }
                        }
                    }
                }
            },
            _ => (),
        }
    }

    fn contains(&self, (x, y): (i64, i64)) -> bool {
        match *self {
            Segment::Horizontal { row, start, end } => y == row && x >= start && x <= end,
            Segment::Vertical { column, start, end } => x == column && y >= start && y <= end,
            Segment::Diagonal { x0, y0, x1, y1 } => {
                if y1 > y0 {
                    if x > x1 || y > y1 {
                        false
                    } else if x < x0 || y < y0 {
                        false
                    } else {
                        x1 - x == y1 - y
                    }
                } else {
                    if x > x1 || y > y0 {
                        false
                    } else if x < x0 || y < y1 {
                        false
                    } else {
                        x1 - x == y - y1
                    }
                }
            }
        }
    }

    fn iter(&self) -> SegmentIter {
        SegmentIter {
            segment: *self,
            counter: 0,
        }
    }
}

struct SegmentIter {
    segment: Segment,
    counter: i64,
}

impl Iterator for SegmentIter {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        use Segment::*;
        match self.segment {
            Horizontal { row, start, end } => {
                if start + self.counter > end {
                    None
                } else {
                    let res = Some((start + self.counter, row));
                    self.counter += 1;
                    res
                }
            }
            Vertical { column, start, end } => {
                if start + self.counter > end {
                    None
                } else {
                    let res = Some((column, start + self.counter));
                    self.counter += 1;
                    res
                }
            }
            Diagonal { x0, y0, x1, y1 } => {
                let x = x0 + self.counter;
                let y = if y0 > y1 {
                    y0 - self.counter
                } else {
                    y0 + self.counter
                };

                if x > x1 {
                    None
                } else {
                    self.counter += 1;
                    Some((x, y))
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let text = input.trim().lines();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut lines = Vec::new();
    for line in text {
        if let Some((from, to)) = line.split_once(" -> ") {
            let start = from
                .split_once(",")
                .and_then(|(a, b)| a.parse::<i64>().ok().zip(b.parse::<i64>().ok()));
            let end = to
                .split_once(",")
                .and_then(|(a, b)| a.parse::<i64>().ok().zip(b.parse::<i64>().ok()));

            if let Some(((x0, y0), (x1, y1))) = start.zip(end) {
                max_x = x0.max(x1).max(max_x);
                max_y = y0.max(y1).max(max_y);
                if x0 == x1 {
                    lines.push(Segment::Vertical {
                        column: x0,
                        start: y0.min(y1),
                        end: y0.max(y1),
                    });
                } else if y0 == y1 {
                    lines.push(Segment::Horizontal {
                        row: y0,
                        start: x0.min(x1),
                        end: x0.max(x1),
                    });
                }
            }
        }
    }

    let mut matches = HashSet::new();
    for (idx, a) in lines.iter().enumerate() {
        for b in lines[idx + 1..].iter() {
            a.intersect(b, &mut matches);
        }
    }

    matches.len()
}

pub fn part_two(input: &str) -> usize {
    let text = input.trim().lines();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut lines = Vec::new();
    for line in text {
        if let Some((from, to)) = line.split_once(" -> ") {
            let start = from
                .split_once(",")
                .and_then(|(a, b)| a.parse::<i64>().ok().zip(b.parse::<i64>().ok()));
            let end = to
                .split_once(",")
                .and_then(|(a, b)| a.parse::<i64>().ok().zip(b.parse::<i64>().ok()));

            if let Some(((x0, y0), (x1, y1))) = start.zip(end) {
                max_x = x0.max(x1).max(max_x);
                max_y = y0.max(y1).max(max_y);
                if x0 == x1 {
                    lines.push(Segment::Vertical {
                        column: x0,
                        start: y0.min(y1),
                        end: y0.max(y1),
                    });
                } else if y0 == y1 {
                    lines.push(Segment::Horizontal {
                        row: y0,
                        start: x0.min(x1),
                        end: x0.max(x1),
                    });
                } else {
                    if x1 > x0 {
                        lines.push(Segment::Diagonal { x0, y0, x1, y1 });
                    } else {
                        lines.push(Segment::Diagonal {
                            x0: x1,
                            y0: y1,
                            x1: x0,
                            y1: y0,
                        });
                    }
                }
            }
        }
    }

    let mut matches = HashSet::new();
    for (idx, a) in lines.iter().enumerate() {
        for b in lines[idx + 1..].iter() {
            a.intersect(b, &mut matches);
        }
    }

    matches.len()
}

#[test]
fn test() {
    let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    assert_eq!(5, part_one(input));
    assert_eq!(12, part_two(input));
}
