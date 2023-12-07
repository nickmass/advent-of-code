pub fn part_one(input: &str) -> usize {
    solve_day::<false>(input)
}

pub fn part_two(input: &str) -> usize {
    solve_day::<true>(input)
}

fn solve_day<const JOKERS: bool>(input: &str) -> usize {
    let mut lines: Vec<_> = input
        .trim()
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(h, b)| (Hand::new::<JOKERS>(h), b.parse::<usize>().unwrap()))
        .collect();

    lines.sort();

    lines
        .into_iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * bet)
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
    rank: Rank,
}

impl Hand {
    fn new<const JOKERS: bool>(h: &str) -> Self {
        let cards_iter = h.chars().map(Card::from).map(|c| {
            if JOKERS && c == Card::J {
                Card::Joker
            } else {
                c
            }
        });

        let mut cards = [Card::A; 5];
        for (idx, card) in cards_iter.enumerate() {
            cards[idx] = card;
        }

        let mut counts = [None; 5];
        let mut jokers = 0;

        for &card in &cards {
            if card == Card::Joker {
                jokers += 1;
                continue;
            }

            for counter in counts.iter_mut() {
                if let Some((counter_card, count)) = counter.as_mut() {
                    if card == *counter_card {
                        *count += 1;
                        break;
                    }
                } else {
                    *counter = Some((card, 1));
                    break;
                }
            }
        }

        counts.sort_by_key(|c| c.map(|c| c.1).unwrap_or(0));

        let mut counts = counts
            .into_iter()
            .rev()
            .filter_map(|c| c)
            .map(|(_, count)| count);

        let a = counts.next().unwrap_or(0);
        let b = counts.next().unwrap_or(0);

        let rank = match (a + jokers, b) {
            (5, 0) => Rank::FiveOfAKind,
            (4, 1) => Rank::FourOfAKind,
            (3, 2) => Rank::FullHouse,
            (3, 1) => Rank::ThreeOfAKind,
            (2, 2) => Rank::TwoPair,
            (2, 1) => Rank::OnePair,
            (1, 1) => Rank::HighCard,
            _ => unreachable!(),
        };

        Hand { cards, rank }
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match a.cmp(b) {
                        std::cmp::Ordering::Equal => (),
                        cmp => return cmp,
                    }
                }
                std::cmp::Ordering::Equal
            }
            cmp => return cmp,
        }
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl std::cmp::Eq for Hand {}

#[derive(Debug, Copy, Clone)]
enum Rank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Rank {
    fn value(&self) -> i32 {
        match self {
            Rank::FiveOfAKind => 7,
            Rank::FourOfAKind => 6,
            Rank::FullHouse => 5,
            Rank::ThreeOfAKind => 4,
            Rank::TwoPair => 3,
            Rank::OnePair => 2,
            Rank::HighCard => 1,
        }
    }
}

impl std::cmp::Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl std::cmp::PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl std::cmp::Eq for Rank {}

#[derive(Debug, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            i => unreachable!("{i:?}"),
        }
    }
}

impl Card {
    fn value(&self) -> i32 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Joker => 1,
        }
    }
}

impl std::cmp::Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl std::cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl std::cmp::Eq for Card {}

#[test]
fn test() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    assert_eq!(6440, part_one(input));
    assert_eq!(5905, part_two(input));
}
