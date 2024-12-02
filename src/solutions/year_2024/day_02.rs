pub fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|l| {
            l.trim()
                .split(' ')
                .filter_map(|n| n.parse::<i32>().ok())
                .report()
                .is_safe()
        })
        .count()
}

pub fn part_two(input: &str) -> usize {
    let reports = input.trim().lines().map(|l| {
        l.trim()
            .split(' ')
            .filter_map(|n| n.parse::<i32>().ok())
            .report()
    });

    let mut safe_count = 0;
    for report in reports {
        let level_count = report.clone().count();
        for idx in 0..=level_count {
            let mut report = report.clone();
            report.skip_level(idx);

            if report.is_safe() {
                safe_count += 1;
                break;
            }
        }
    }

    safe_count
}

trait ReportIterExt: Iterator + Sized {
    fn report(self) -> ReportIter<Self>;
}

impl<I: Iterator<Item = T>, T: Copy> ReportIterExt for I {
    fn report(self) -> ReportIter<Self> {
        ReportIter::new(self)
    }
}

#[derive(Debug, Clone)]
struct ReportIter<I: Iterator> {
    inner: I,
    skip: Option<usize>,
    idx: usize,
    prev: Option<I::Item>,
}

impl<I: Iterator<Item = T>, T: Copy> ReportIter<I> {
    fn new(iter: I) -> Self {
        Self {
            inner: iter,
            idx: 0,
            prev: None,
            skip: None,
        }
    }

    fn skip_level(&mut self, idx: usize) {
        self.skip = Some(idx);
    }
}

impl<I: Iterator<Item = i32>> ReportIter<I> {
    fn is_safe(self) -> bool {
        let mut sign = None;
        for (a, b) in self {
            let diff = a - b;
            if diff == 0 || diff.abs() > 3 {
                return false;
            }

            if sign.is_none() {
                sign = Some(diff.signum())
            }

            if diff.signum() != sign.unwrap_or(0) {
                return false;
            }
        }

        true
    }
}

impl<I: Iterator<Item = T>, T: Copy> Iterator for ReportIter<I> {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let prev = if let Some(prev) = self.prev {
            prev
        } else {
            if self.skip == Some(self.idx) {
                let _ = self.inner.next()?;
            }
            self.inner.next()?
        };

        self.idx += 1;
        if self.skip == Some(self.idx) {
            let _ = self.inner.next()?;
        }

        let next = self.inner.next()?;

        let value = (prev, next);

        self.prev = Some(next);

        Some(value)
    }
}

#[test]
fn test() {
    let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    assert_eq!(2, part_one(input));
    assert_eq!(4, part_two(input));
}
