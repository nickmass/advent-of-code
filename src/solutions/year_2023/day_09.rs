pub fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(OasisDiffTree::new)
        .map(|l| l.rows().fold(0, |acc, row| row.last() + acc))
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    input
        .lines()
        .map(OasisDiffTree::new)
        .map(|l| l.rows().fold(0, |acc, row| row.first() - acc))
        .sum()
}

struct OasisDiffTree {
    values: Vec<i64>,
    final_len: usize,
}

impl OasisDiffTree {
    fn new(line: &str) -> Self {
        let mut diffs: Vec<_> = line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect();

        let mut n = 0;
        let mut m = diffs.len();

        loop {
            let mut all_zero = true;
            let mut prev = None;
            for i in n..m {
                let x = diffs[i];
                if let Some(prev) = prev {
                    let diff = x - prev;
                    diffs.push(diff);
                    if diff != 0 {
                        all_zero = false;
                    }
                }
                prev = Some(x)
            }

            n = m;
            m = diffs.len();

            if all_zero {
                break;
            }
        }

        Self {
            values: diffs,
            final_len: m - n,
        }
    }

    fn rows(&self) -> impl Iterator<Item = DiffRow<'_>> {
        let mut n = self.values.len();
        let mut next_len = self.final_len;

        std::iter::from_fn(move || {
            if n == 0 {
                return None;
            }

            let m = n;
            n -= next_len;
            let res = &self.values[n..m];

            next_len += 1;

            Some(DiffRow(res))
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct DiffRow<'a>(&'a [i64]);

impl<'a> DiffRow<'a> {
    fn first(&self) -> i64 {
        self.0[0]
    }

    fn last(&self) -> i64 {
        self.0[self.0.len() - 1]
    }
}

#[test]
fn test() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    assert_eq!(114, part_one(input));
    assert_eq!(2, part_two(input));
}
