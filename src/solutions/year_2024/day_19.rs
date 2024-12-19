use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let patterns = Patterns::new(lines.next().unwrap());

    let _ = lines.next().unwrap();

    let mut found = 0;
    let mut search = Vec::new();
    for towel in lines {
        search.clear();
        let towel = towel.trim().as_bytes();
        search.push(towel);

        while let Some(towel) = search.pop() {
            if towel.is_empty() {
                found += 1;
                break;
            }

            for p in patterns.patterns() {
                if towel.starts_with(p) {
                    search.push(&towel[p.len()..]);
                }
            }
        }
    }

    found
}

pub fn part_two(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let patterns = Patterns::new(lines.next().unwrap());

    let _ = lines.next().unwrap();

    let mut towel_map = HashSet::new();
    let mut min_len = usize::MAX;
    let mut max_len = 0;
    for line in lines {
        let towel = line.trim().as_bytes();
        min_len = towel.len().min(min_len);
        max_len = towel.len().max(max_len);
        towel_map.insert(towel);
    }

    let mut len_maps = Vec::with_capacity(max_len);
    for l in 0..=max_len {
        let mut len_map = HashSet::new();
        for t in towel_map.iter() {
            if t.len() >= l {
                len_map.insert(&t[..l]);
            }
        }
        len_maps.push(len_map);
    }

    let mut build_map: HashMap<Towel<60>, u64> = HashMap::new();
    let mut search_map = build_map.clone();

    build_map.insert(Towel::new(), 1);

    let mut total = 0;
    while !build_map.is_empty() {
        std::mem::swap(&mut build_map, &mut search_map);
        for (towel, count) in search_map.drain() {
            if towel.len() >= min_len && towel_map.contains(towel.as_slice()) {
                total += count;
                continue;
            }

            for p in patterns.patterns() {
                let mut towel = towel.clone();
                towel.extend(p);

                if towel.len() > max_len || !len_maps[towel.len()].contains(towel.as_slice()) {
                    continue;
                }

                *build_map.entry(towel).or_default() += count;
            }
        }
    }

    total
}

struct Patterns {
    colors: Vec<u8>,
    patterns: Vec<std::ops::Range<usize>>,
}

impl Patterns {
    fn new(line: &str) -> Self {
        let line = line.trim().as_bytes();
        let mut colors = Vec::with_capacity(line.len());
        let mut patterns = Vec::new();
        let mut current = 0;
        for &b in line {
            match b {
                b',' => {
                    patterns.push(current..colors.len());
                    current = colors.len();
                }
                b' ' => continue,
                _ => {
                    colors.push(b);
                }
            }
        }
        patterns.push(current..colors.len());

        Self { colors, patterns }
    }

    fn patterns(&self) -> impl Iterator<Item = &'_ [u8]> {
        self.patterns
            .iter()
            .cloned()
            .filter_map(|p| self.colors.get(p))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Towel<const N: usize> {
    arr: [u8; N],
    len: usize,
}

impl<const N: usize> Towel<N> {
    fn new() -> Self {
        Self {
            arr: [0; N],
            len: 0,
        }
    }

    fn extend<'a, I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        let iter = iter.into_iter().take(N - self.len).copied();
        for n in iter {
            self.arr[self.len] = n;
            self.len += 1;
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn as_slice(&self) -> &[u8] {
        &self.arr[0..self.len]
    }
}

#[test]
fn test() {
    let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;

    assert_eq!(6, part_one(input));
    assert_eq!(16, part_two(input));
}
