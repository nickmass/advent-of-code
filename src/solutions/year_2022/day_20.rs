pub fn part_one(input: &str) -> isize {
    let mixer = Mixer::<1, 1>::new(input);
    mixer.answer()
}

pub fn part_two(input: &str) -> isize {
    let mixer = Mixer::<10, 811589153>::new(input);
    mixer.answer()
}

struct Mixer<const COUNT: usize, const KEY: isize> {
    items: Vec<MixerEntry>,
    start: usize,
}

#[derive(Debug, Copy, Clone)]
struct MixerEntry {
    value: isize,
    prev: usize,
    next: usize,
}

impl<const COUNT: usize, const KEY: isize> Mixer<COUNT, KEY> {
    fn new(input: &str) -> Self {
        let mut items = Vec::new();
        let mut prev = None;
        let mut start = 0;
        for (idx, value) in input
            .trim()
            .lines()
            .filter_map(|l| l.parse::<isize>().ok())
            .enumerate()
        {
            let p = if let Some(prev) = prev { prev } else { 0 };
            prev = Some(idx);

            if value == 0 {
                start = idx;
            }

            items.push(MixerEntry {
                value: value * KEY,
                prev: p,
                next: idx + 1,
            })
        }

        let len = items.len();
        items[0].prev = len - 1;
        items[len - 1].next = 0;

        for _ in 0..COUNT {
            for me in 0..len {
                let item = items[me];
                let max = item.value.abs() as usize % (len - 1);
                let (new_prev, new_next) = if item.value < 0 {
                    let mut new_idx = me;
                    for _ in 0..max {
                        new_idx = items[new_idx].prev;
                        if new_idx == me {
                            new_idx = items[new_idx].prev;
                        }
                    }

                    let prev = items[new_idx].prev;
                    let next = new_idx;
                    (prev, next)
                } else if item.value > 0 {
                    let mut new_idx = me;
                    for _ in 0..max {
                        new_idx = items[new_idx].next;
                        if new_idx == me {
                            new_idx = items[new_idx].next;
                        }
                    }

                    let prev = new_idx;
                    let next = items[new_idx].next;
                    (prev, next)
                } else {
                    continue;
                };

                let next = item.next;
                let prev = item.prev;
                items[next].prev = prev;
                items[prev].next = next;
                items[me].next = new_next;
                items[me].prev = new_prev;
                items[new_next].prev = me;
                items[new_prev].next = me;
            }
        }

        Mixer { items, start }
    }

    fn iter(&self) -> MixerIter<'_> {
        MixerIter {
            items: &self.items,
            position: self.start,
        }
    }

    fn answer(&self) -> isize {
        self.iter()
            .enumerate()
            .filter_map(|(i, n)| (i % 1000 == 0).then_some(n))
            .skip(1)
            .take(3)
            .sum()
    }
}

struct MixerIter<'a> {
    items: &'a Vec<MixerEntry>,
    position: usize,
}

impl<'a> Iterator for MixerIter<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items[self.position as usize];
        let v = item.value;
        self.position = item.next;

        Some(v)
    }
}

#[test]
fn test() {
    let input = r#"1
2
-3
3
-2
0
4"#;

    assert_eq!(3, part_one(input));
    assert_eq!(1623178306, part_two(input));
}
