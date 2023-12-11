pub fn part_one(input: &str) -> i64 {
    let (seeds, rest) = input.trim().split_once("\n\n").unwrap();
    let mut seeds: Vec<_> = seeds["seeds: ".len()..]
        .split(" ")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut forwards = Vec::new();
    let mut mappings = Vec::new();

    for map in rest.split("\n\n") {
        let mut map_lines = map.lines();
        let _names = map_lines.next();

        mappings.clear();
        for mapping in map_lines {
            let mut mapping = mapping.split(" ").map(|n| n.parse::<i64>().unwrap());

            let dst = mapping.next().unwrap();
            let src = mapping.next().unwrap();
            let len = mapping.next().unwrap();

            mappings.push((dst, src, len));
        }

        'seed: for seed in seeds.iter().copied() {
            for (dst, src, len) in mappings.iter().copied() {
                let s = seed - src;
                if s < len && s >= 0 {
                    forwards.push(s + dst);
                    continue 'seed;
                }
            }

            forwards.push(seed);
        }

        std::mem::swap(&mut forwards, &mut seeds);
        forwards.clear();
    }

    seeds.into_iter().min().unwrap_or(0)
}

pub fn part_two(input: &str) -> i64 {
    let (seeds, rest) = input.trim().split_once("\n\n").unwrap();
    let mut seeds = seeds["seeds: ".len()..]
        .split(" ")
        .filter_map(|s| s.parse::<i64>().ok());

    let mut ranges = Vec::new();
    while let Some(start) = seeds.next() {
        let len = seeds.next().unwrap();
        ranges.push(Range { start, len })
    }

    let mut forwards = Vec::new();
    let mut mappings = Vec::new();
    let mut remains = Vec::new();

    for map in rest.split("\n\n") {
        let mut map_lines = map.lines();
        let _name = map_lines.next();

        mappings.clear();
        for mapping in map_lines {
            let mut mapping = mapping.split(" ").map(|n| n.parse::<i64>().unwrap());

            let dst = mapping.next().unwrap();
            let src = mapping.next().unwrap();
            let len = mapping.next().unwrap();

            mappings.push(Mapping { src, dst, len });
        }

        let mut changed = true;
        while changed {
            changed = false;
            for mapping in mappings.iter().copied() {
                for range in ranges.drain(..) {
                    let (pre, forward, post) = mapping.apply(range);

                    remains.extend(pre);
                    remains.extend(post);

                    if let Some(forward) = forward {
                        forwards.push(forward);
                        changed = true;
                    }
                }
                std::mem::swap(&mut remains, &mut ranges);
            }
        }

        ranges.extend(forwards.drain(..));
    }

    ranges.into_iter().map(|r| r.start).min().unwrap_or(0)
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    len: i64,
}

impl Range {
    fn end(&self) -> i64 {
        self.start + self.len
    }
}

#[derive(Debug, Copy, Clone)]
struct Mapping {
    src: i64,
    dst: i64,
    len: i64,
}

impl Mapping {
    fn src_end(&self) -> i64 {
        self.src + self.len
    }

    fn apply(&self, range: Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        let len = self.src - range.start;
        let pre = if len > 0 {
            Some(Range {
                start: range.start,
                len: len.min(range.len),
            })
        } else {
            None
        };

        let start = self.src.max(range.start);
        let end = self.src_end().min(range.end());
        let forward = if end == start || (end < range.start || start > range.end()) {
            None
        } else {
            Some(Range {
                start: start + (self.dst - self.src),
                len: end - start,
            })
        };

        let len = range.end() - self.src_end();
        let post = if len > 0 {
            Some(Range {
                start: self.src_end().max(range.start),
                len: range.len.min(len),
            })
        } else {
            None
        };

        (pre, forward, post)
    }
}

#[test]
fn test() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    assert_eq!(35, part_one(input));
    assert_eq!(46, part_two(input));
}
