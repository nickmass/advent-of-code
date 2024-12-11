use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    Stones::new(input).run::<25>()
}

pub fn part_two(input: &str) -> u64 {
    Stones::new(input).run::<75>()
}

struct Stones {
    counts: HashMap<u64, u64>,
}

impl Stones {
    fn new(input: &str) -> Self {
        let stones = input.trim().split(" ").map(|n| n.parse::<u64>().unwrap());
        let mut counts = HashMap::new();

        for stone in stones {
            *counts.entry(stone).or_default() += 1;
        }

        Self { counts }
    }

    fn generation(&mut self, next: &mut HashMap<u64, u64>) {
        next.clear();
        for (&n, &count) in self.counts.iter() {
            if n == 0 {
                *next.entry(1).or_default() += count;
            } else if (n.ilog10() + 1) % 2 == 0 {
                let c = (n.ilog10() + 1) / 2;
                let a = n / 10u64.pow(c);
                let b = n % 10u64.pow(c);
                *next.entry(a).or_default() += count;
                *next.entry(b).or_default() += count;
            } else {
                *next.entry(n * 2024).or_default() += count;
            }
        }

        std::mem::swap(&mut self.counts, next);
    }

    fn run<const N: usize>(&mut self) -> u64 {
        let mut next_gen = HashMap::new();

        for _ in 0..N {
            self.generation(&mut next_gen);
        }

        self.counts.values().sum()
    }
}

#[test]
fn test() {
    let input = r#"125 17"#;

    assert_eq!(55312, part_one(input));
}
