use std::cmp::Ordering;

pub fn part_one(input: &str) -> usize {
    let (ranges, items) = input.split_once("\n\n").expect("valid input");
    let ranges = OrderedRanges::new(ranges);

    items
        .lines()
        .filter_map(|l| l.parse::<u64>().ok())
        .filter(|&i| ranges.contains(i))
        .count()
}

pub fn part_two(input: &str) -> u64 {
    let (ranges, _items) = input.split_once("\n\n").expect("valid input");
    let ranges = compact_and_sort_ranges(ranges);

    ranges.iter().map(|r| r.len()).sum()
}

fn compact_and_sort_ranges(range_input: &str) -> Vec<Range> {
    let mut ranges: Vec<_> = range_input.lines().filter_map(Range::parse).collect();
    ranges.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let mut results = Vec::new();

    let Some(mut next) = ranges.pop() else {
        return Vec::new();
    };

    while let Some(other) = ranges.pop() {
        if let Some(compacted) = next.compact(&other) {
            next = compacted;
        } else {
            results.push(next);
            next = other;
        }
    }
    results.push(next);

    results
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    fn compare(&self, item: u64) -> Ordering {
        if self.start > item {
            Ordering::Greater
        } else if self.end < item {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    fn compact(&self, other: &Range) -> Option<Range> {
        debug_assert!(self <= other);
        if self.start <= other.end && other.start <= self.end + 1 {
            Some(Range {
                start: self.start,
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }
}

struct OrderedRanges {
    ranges: Vec<Range>,
}

impl OrderedRanges {
    fn new(range_input: &str) -> Self {
        Self {
            ranges: compact_and_sort_ranges(range_input),
        }
    }

    fn contains(&self, item: u64) -> bool {
        self.ranges
            .binary_search_by(|range| range.compare(item))
            .is_ok()
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
