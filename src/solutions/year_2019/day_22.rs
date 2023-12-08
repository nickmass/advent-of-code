pub fn part_one(input: &str) -> i64 {
    solve_part_one::<10007>(input, 2019)
}

pub fn part_two(_input: &str) -> i64 {
    0
}

fn solve_part_one<const DECK_SIZE: i64>(input: &str, tracked: i64) -> i64 {
    let tracker = input
        .trim()
        .lines()
        .map(|s| s.parse::<Action>().unwrap())
        .fold(CardTracker::new(DECK_SIZE, tracked), |tracker, action| {
            tracker.apply(action)
        });

    tracker.position()
}

struct CardTracker {
    deck_size: i64,
    tracked: i64,
}

impl CardTracker {
    fn new(deck_size: i64, tracked: i64) -> Self {
        Self { deck_size, tracked }
    }

    fn deal(&mut self) {
        self.tracked = self.deck_size - self.tracked - 1;
    }

    fn cut(&mut self, cut: i64) {
        self.tracked = (self.tracked - cut).rem_euclid(self.deck_size);
    }

    fn shuffle(&mut self, count: i64) {
        self.tracked = (self.tracked * count) % self.deck_size;
    }

    fn apply(mut self, action: Action) -> Self {
        match action {
            Action::Deal => self.deal(),
            Action::Cut(n) => self.cut(n),
            Action::Shuffle(n) => self.shuffle(n),
            Action::Print => {
                println!("{}", self.tracked);
            }
        }
        self
    }

    fn position(&self) -> i64 {
        self.tracked
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum Action {
    Deal,
    Cut(i64),
    Shuffle(i64),
    Print,
}

#[derive(Debug, Copy, Clone)]
struct ActionParseErr;

impl std::str::FromStr for Action {
    type Err = ActionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, num) = s.rsplit_once(' ').ok_or(ActionParseErr)?;

        match kind {
            "deal into new" => Ok(Action::Deal),
            "cut" => {
                let num = num.parse().map_err(|_| ActionParseErr)?;
                Ok(Action::Cut(num))
            }
            "deal with increment" => {
                let num = num.parse().map_err(|_| ActionParseErr)?;
                Ok(Action::Shuffle(num))
            }
            _ => Err(ActionParseErr),
        }
    }
}

#[test]
fn test() {
    let input = r#"deal with increment 7
deal into new stack
deal into new stack"#;

    assert_eq!(2, solve_part_one::<10>(input, 6));

    let input = r#"cut 6
deal with increment 7
deal into new stack"#;

    assert_eq!(9, solve_part_one::<10>(input, 6));

    let input = r#"deal with increment 7
deal with increment 9
cut -2"#;

    assert_eq!(0, solve_part_one::<10>(input, 6));

    let input = r#"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"#;

    assert_eq!(9, solve_part_one::<10>(input, 6));
}
