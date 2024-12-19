pub fn part_one(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let patterns = Patterns::new(lines.next().unwrap());

    let _ = lines.next().unwrap();

    let mut total = 0;
    let mut search = std::collections::BinaryHeap::new();
    for design in lines {
        let design = design.trim().as_bytes();
        search.clear();
        search.push(0);

        while let Some(len) = search.pop() {
            if len == design.len() {
                total += 1;
                break;
            }

            for p in patterns.patterns() {
                let end = len + p.len();
                let range = len..end;
                if Some(p) == design.get(range) {
                    search.push(end);
                }
            }
        }
    }

    total
}

pub fn part_two(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let patterns = Patterns::new(lines.next().unwrap());

    let _ = lines.next().unwrap();

    let mut total = 0;
    let mut search = std::collections::BTreeMap::new();
    for design in lines {
        let design = design.trim().as_bytes();
        search.clear();
        search.insert(0, 1);

        while let Some((len, count)) = search.pop_first() {
            if len == design.len() {
                total += count;
                break;
            }

            for p in patterns.patterns() {
                let end = len + p.len();
                let range = len..end;
                if Some(p) == design.get(range) {
                    *search.entry(end).or_default() += count;
                }
            }
        }
    }

    total
}

struct Patterns<'a> {
    line: &'a [u8],
    patterns: Vec<std::ops::Range<usize>>,
}

impl<'a> Patterns<'a> {
    fn new(line: &'a str) -> Self {
        let line = line.trim().as_bytes();
        let mut patterns = Vec::new();
        let mut current = Some(0);
        for (idx, &b) in line.iter().enumerate() {
            match b {
                b',' => {
                    if let Some(current) = current.take() {
                        patterns.push(current..idx);
                    }
                }
                b' ' => continue,
                _ => {
                    if current.is_none() {
                        current = Some(idx);
                    }
                }
            }
        }

        if let Some(current) = current.take() {
            patterns.push(current..line.len());
        }

        Self { line, patterns }
    }

    fn patterns(&self) -> impl Iterator<Item = &'_ [u8]> {
        self.patterns
            .iter()
            .cloned()
            .filter_map(|p| self.line.get(p))
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
