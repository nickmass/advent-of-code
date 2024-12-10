use crate::HashSet;

pub fn part_one(input: &str) -> u64 {
    let mut hands = parse(input);

    while !hands.game_over() {
        let (one, two) = hands.pop();

        if one > two {
            hands.player_one_mut().push(one);
            hands.player_one_mut().push(two);
        } else {
            hands.player_two_mut().push(two);
            hands.player_two_mut().push(one);
        }
    }

    if hands.player_two().is_empty() {
        hands.player_one().score()
    } else {
        hands.player_two().score()
    }
}

pub fn part_two(input: &str) -> u64 {
    let mut hands = parse(input);

    let winner = combat_game(&mut hands);

    match winner {
        Winner::PlayerOne => hands.player_one().score(),
        Winner::PlayerTwo => hands.player_two().score(),
    }
}

fn parse(input: &str) -> Hands {
    let mut hands = Hands::new();
    let mut second_deck = false;

    for line in input.trim().lines() {
        if line.is_empty() {
            second_deck = true;
            continue;
        }

        if line.starts_with('P') {
            continue;
        }

        let n = line.parse().expect("valid card in range 0..256");

        if second_deck {
            hands.player_two_mut().push(n);
        } else {
            hands.player_one_mut().push(n);
        }
    }

    hands
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Winner {
    PlayerOne,
    PlayerTwo,
}

fn combat_game(hands: &mut Hands) -> Winner {
    let mut previous_game_states = HashSet::with_capacity(512);

    while !hands.game_over() {
        combat_round(hands);
        let state = HandState::from(&*hands);
        if !previous_game_states.insert(state) {
            return Winner::PlayerOne;
        }
    }

    if hands.player_two().is_empty() {
        Winner::PlayerOne
    } else {
        Winner::PlayerTwo
    }
}

fn combat_round(hands: &mut Hands) {
    let (one, two) = hands.pop();

    let winner = if hands.player_one().len() >= one && hands.player_two().len() >= two {
        let mut sub_p_one = hands.player_one().iter().take(one as usize);
        let mut sub_p_two = hands.player_two().iter().take(two as usize);

        let mut sub_hands =
            std::iter::from_fn(move || match (sub_p_one.next(), sub_p_two.next()) {
                (None, None) => None,
                h => Some(h),
            })
            .collect();

        combat_game(&mut sub_hands)
    } else if one > two {
        Winner::PlayerOne
    } else if two > one {
        Winner::PlayerTwo
    } else {
        panic!("someone has to have the higher card, invalid decks")
    };

    match winner {
        Winner::PlayerOne => {
            hands.player_one_mut().push(one);
            hands.player_one_mut().push(two);
        }
        Winner::PlayerTwo => {
            hands.player_two_mut().push(two);
            hands.player_two_mut().push(one);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct HandState {
    cards: [u8; 50],
    player_one_len: u8,
    player_two_len: u8,
}

impl From<&Hands> for HandState {
    fn from(hands: &Hands) -> Self {
        let mut cards = [0; 50];
        let player_one_len = hands.player_one().len();
        let player_two_len = hands.player_two().len();

        for (i, c) in hands
            .player_one()
            .iter()
            .chain(hands.player_two().iter())
            .enumerate()
        {
            cards[i] = c;
        }

        Self {
            cards,
            player_one_len,
            player_two_len,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Hand {
    cards: [u8; 64],
    len: u8,
    cursor: u8,
}

impl Hand {
    fn new() -> Self {
        Self {
            cards: [0; 64],
            len: 0,
            cursor: 0,
        }
    }

    fn push(&mut self, card: u8) {
        if self.len() == 50 {
            panic!("overflowed hand")
        }
        let idx = (self.cursor.wrapping_add(self.len)) & 0x3F;
        self.cards[idx as usize] = card;
        self.len += 1;
    }

    fn pop(&mut self) -> u8 {
        if self.is_empty() {
            panic!("popped card from empty hand")
        } else {
            let res = self.cards[(self.cursor & 0x3f) as usize];
            self.cursor = self.cursor.wrapping_add(1);
            self.len -= 1;
            res
        }
    }

    fn len(&self) -> u8 {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        let start = self.cursor & 0x3f;
        let end = (start + self.len) & 0x3f;
        let mut idx = start;
        std::iter::from_fn(move || {
            if idx == end {
                None
            } else {
                let res = self.cards[idx as usize];
                idx = (idx + 1) & 0x3f;
                Some(res)
            }
        })
    }

    fn score(&self) -> u64 {
        self.iter()
            .enumerate()
            .map(|(i, c)| c as u64 * (self.len() as u64 - i as u64))
            .sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Hands {
    player_one: Hand,
    player_two: Hand,
}

impl Hands {
    fn new() -> Self {
        Self {
            player_one: Hand::new(),
            player_two: Hand::new(),
        }
    }

    fn pop(&mut self) -> (u8, u8) {
        (self.player_one_mut().pop(), self.player_two_mut().pop())
    }

    fn player_one_mut(&mut self) -> &mut Hand {
        &mut self.player_one
    }

    fn player_one(&self) -> &Hand {
        &self.player_one
    }

    fn player_two_mut(&mut self) -> &mut Hand {
        &mut self.player_two
    }

    fn player_two(&self) -> &Hand {
        &self.player_two
    }

    fn game_over(&self) -> bool {
        self.player_one().is_empty() || self.player_two().is_empty()
    }
}

impl std::iter::FromIterator<(Option<u8>, Option<u8>)> for Hands {
    fn from_iter<T: IntoIterator<Item = (Option<u8>, Option<u8>)>>(iter: T) -> Self {
        let iter = iter.into_iter();

        let mut player_one = [0; 64];
        let mut player_two = [0; 64];
        let mut player_one_len = 0;
        let mut player_two_len = 0;

        for (idx, (p1, p2)) in iter.enumerate() {
            if let Some(p1) = p1 {
                player_one[idx] = p1;
                player_one_len += 1;
            }
            if let Some(p2) = p2 {
                player_two[idx] = p2;
                player_two_len += 1;
            }
        }

        let player_one = Hand {
            cards: player_one,
            len: player_one_len,
            cursor: 0,
        };

        let player_two = Hand {
            cards: player_two,
            len: player_two_len,
            cursor: 0,
        };

        Self {
            player_one,
            player_two,
        }
    }
}

impl std::fmt::Display for Hands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P1: [")?;
        for c in self.player_one().iter() {
            write!(f, " {}", c)?;
        }
        write!(f, " ]  P2: [")?;
        for c in self.player_two().iter() {
            write!(f, " {}", c)?;
        }
        write!(f, " ]")
    }
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

    run_a(i, 306);
    run_b(i, 291);
}
