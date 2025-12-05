pub fn part_one(input: &str) -> usize {
    let (ranges, items) = input.split_once("\n\n").expect("valid input");
    let ranges = compact_ranges(ranges);

    return items
        .lines()
        .filter_map(|l| l.parse::<u64>().ok())
        .filter(|i| ranges.iter().any(|r| r.contains(*i)))
        .count();
}

pub fn part_two(input: &str) -> u64 {
    let (ranges, _items) = input.split_once("\n\n").expect("valid input");
    let ranges = compact_ranges(ranges);

    ranges.iter().map(|r| r.len()).sum()
}

fn compact_ranges(range_input: &str) -> Vec<Range> {
    let mut ranges: Vec<_> = range_input.lines().filter_map(Range::parse).collect();
    let mut compacted_ranges = Vec::with_capacity(ranges.len());
    let mut pending_ranges = Vec::with_capacity(ranges.len());

    let mut done = false;
    while !done {
        done = true;
        while let Some(mut next) = ranges.pop() {
            for r in ranges.drain(..) {
                if let Some(compacted) = next.compact(&r) {
                    next = compacted;
                    done = false;
                } else {
                    pending_ranges.push(r);
                }
            }

            compacted_ranges.push(next);
            std::mem::swap(&mut ranges, &mut pending_ranges);
        }
        std::mem::swap(&mut ranges, &mut compacted_ranges);
    }

    ranges
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn parse(line: &str) -> Option<Self> {
        let (start, end) = line.split_once("-")?;
        let start = start.parse().ok()?;
        let end = end.parse().ok()?;

        Some(Range { start, end })
    }

    fn contains(&self, item: u64) -> bool {
        item >= self.start && item <= self.end
    }

    fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    fn compact(&self, other: &Range) -> Option<Range> {
        if self.start <= other.end && other.start <= self.end
            || self.end + 1 == other.start
            || other.end + 1 == self.start
        {
            Some(Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    assert_eq!(3, part_one(input));
    assert_eq!(14, part_two(input));
}
