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

    solver(lines.into_iter(), max_x as usize + 1, max_y as usize + 1)
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

    solver(lines.into_iter(), max_x as usize + 1, max_y as usize + 1)
}

fn solver<I: Iterator<Item = Segment>>(iter: I, width: usize, height: usize) -> usize {
    let mut map = CellMap::new(width, height);

    let mut count = 0;
    for line in iter {
        for point in line.iter() {
            if let Some(cell) = map.increment(point) {
                if cell == 2 {
                    count += 1;
                }
            }
        }
    }

    count
}

struct CellMap {
    cells: Vec<u8>,
    stride: usize,
}

impl CellMap {
    fn new(width: usize, height: usize) -> Self {
        let cells = vec![0; width * height];
        let stride = width;

        Self { cells, stride }
    }

    fn increment(&mut self, (x, y): (i64, i64)) -> Option<u8> {
        let x = x as usize;
        let y = y as usize;
        let idx = (y * self.stride) + x;
        let mut cell = self.cells.get_mut(idx);
        cell.as_mut().map(|c| **c = c.saturating_add(1));
        cell.copied()
    }
}

#[derive(Debug, Copy, Clone)]
enum Segment {
    Horizontal { row: i64, start: i64, end: i64 },
    Vertical { column: i64, start: i64, end: i64 },
    Diagonal { x0: i64, y0: i64, x1: i64, y1: i64 },
}

impl Segment {
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
