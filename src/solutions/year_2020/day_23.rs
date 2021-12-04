pub fn part_one(input: &str) -> usize {
    let cups = input
        .trim()
        .chars()
        .map(|n| (n as u8 - b'0') as usize)
        .chain(10..=9);

    let mut cup_game = CupGame::new(cups, 9, 100);
    cup_game.part_one()
}

pub fn part_two(input: &str) -> usize {
    let cups = input
        .trim()
        .chars()
        .map(|n| (n as u8 - b'0') as usize)
        .chain(10..=1_000_000);

    let mut cup_game = CupGame::new(cups, 1_000_000, 10_000_000);
    cup_game.part_two()
}

struct CupGame {
    cups: Vec<usize>,
    iterations: usize,
    current_cup: usize,
    min: usize,
    max: usize,
}

impl CupGame {
    fn new<T: Iterator<Item = usize>>(values: T, length: usize, iterations: usize) -> Self {
        let mut cups = vec![0; length];
        let mut previous = None;
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        let mut first = 0;
        for n in values {
            if let Some(previous) = previous {
                cups[previous - 1] = n;
            } else {
                first = n;
            }
            min = min.min(n);
            max = max.max(n);
            previous = Some(n);
        }
        if let Some(previous) = previous {
            cups[previous - 1] = first;
        }

        let current_cup = first;

        Self {
            cups,
            iterations,
            current_cup,
            min,
            max,
        }
    }

    fn part_one(&mut self) -> usize {
        self.run();
        let mut val = self.cups[0];
        let mut res = 0;
        for _ in 0..8 {
            res *= 10;
            res += val;
            val = self.cups[val - 1];
        }

        res
    }

    fn part_two(&mut self) -> usize {
        self.run();
        let a = self.cups[0];
        let b = self.cups[a - 1];
        a * b
    }

    fn run(&mut self) {
        for _ in 0..self.iterations {
            let one = self.cups[self.current_cup - 1];
            let two = self.cups[one - 1];
            let three = self.cups[two - 1];

            let mut dest = self.current_cup;
            while dest == one || dest == two || dest == three || dest == self.current_cup {
                if dest == self.min {
                    dest = self.max;
                } else {
                    dest -= 1;
                }
            }

            self.cups[self.current_cup - 1] = self.cups[three - 1];
            let temp = self.cups[dest - 1];
            self.cups[dest - 1] = one;
            self.cups[three - 1] = temp;

            self.current_cup = self.cups[self.current_cup - 1];
        }
    }
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"389125467"#;

    run_a(i, 67384529);
    run_b(i, 149245887792);
}
