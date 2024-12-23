use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .filter_map(|n| SecretIter::new(n).skip(1999).next())
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let secrets = input.trim().lines().map(|l| l.parse::<u64>().unwrap());

    let mut total_counts = HashMap::new();
    let mut counts = HashMap::new();
    for s in secrets {
        for (_, n, code) in DiffIter::new(s).take(2000) {
            if let Some(code) = code {
                counts.entry(code).or_insert(n as u8);
            }
        }

        for (k, v) in counts.drain() {
            *total_counts.entry(k).or_insert(0) += v as u32;
        }
    }

    total_counts.into_values().max().unwrap_or(0)
}

struct SecretIter {
    secret: u64,
}

impl SecretIter {
    fn new(secret: u64) -> Self {
        Self { secret }
    }
}

impl Iterator for SecretIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = self.secret;

        n = ((n << 6) ^ n) & 0xffffff;
        n = ((n >> 5) ^ n) & 0xffffff;
        n = ((n << 11) ^ n) & 0xffffff;

        self.secret = n;
        Some(n)
    }
}

struct DiffIter {
    prev: i8,
    code: u32,
    inner: SecretIter,
    count: usize,
}

impl DiffIter {
    fn new(secret: u64) -> Self {
        Self {
            prev: (secret % 10) as i8,
            code: 0,
            inner: SecretIter::new(secret),
            count: 0,
        }
    }
}

impl Iterator for DiffIter {
    type Item = (i8, i8, Option<u32>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let n = (next % 10) as i8;

        let diff = n - self.prev;
        self.prev = n;

        self.code <<= 8;
        self.code |= (diff as u32) & 0xff;

        let code = if self.count < 3 {
            self.count += 1;
            None
        } else {
            Some(self.code)
        };

        Some((diff, n, code))
    }
}

#[test]
fn test() {
    let input = r#"1
10
100
2024
"#;

    assert_eq!(37327623, part_one(input));

    let input = r#"1
2
3
2024
"#;
    assert_eq!(23, part_two(input));
}
