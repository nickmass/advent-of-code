use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    solution(input, 10)
}

pub fn part_two(input: &str) -> u64 {
    solution(input, 40)
}

fn solution(input: &str, generations: u64) -> u64 {
    let mut lines = input.trim().lines();

    let state = lines.next().unwrap().as_bytes().to_vec();

    let subsitutions: HashMap<_, _> = lines
        .filter_map(|l| l.split_once(" -> "))
        .map(|(l, r)| ((l.as_bytes()[0], l.as_bytes()[1]), r.as_bytes()[0]))
        .collect();

    let mut mapping = HashMap::new();
    let mut count = CharacterCounts::new();
    count.push(state[0]);
    for window in state.windows(2) {
        let new_count = solve_pair(
            &subsitutions,
            &mut mapping,
            window[0],
            window[1],
            generations - 1,
        );

        count = count.add(new_count);
    }

    count.solution()
}

fn solve_pair(
    subsitutions: &HashMap<(u8, u8), u8>,
    mapping: &mut HashMap<(u64, u8, u8), CharacterCounts>,
    left: u8,
    right: u8,
    generation: u64,
) -> CharacterCounts {
    if let Some(counts) = mapping.get(&(generation, left, right)) {
        return counts.clone();
    } else {
        if generation == 0 {
            let mut counts = CharacterCounts::new();
            counts.push(right);
            if let Some(additional) = subsitutions.get(&(left, right)).copied() {
                counts.push(additional);
            }
            mapping.insert((0, left, right), counts.clone());
            counts
        } else {
            if let Some(additional) = subsitutions.get(&(left, right)).copied() {
                let one = solve_pair(subsitutions, mapping, left, additional, generation - 1);
                let two = solve_pair(subsitutions, mapping, additional, right, generation - 1);

                let count = one.add(two);
                mapping.insert((generation, left, right), count.clone());
                count
            } else {
                let mut count = CharacterCounts::new();
                count.push(right);
                mapping.insert((generation, left, right), count.clone());
                count
            }
        }
    }
}

#[derive(Clone)]
struct CharacterCounts {
    counts: [u64; 24],
}

impl CharacterCounts {
    fn new() -> Self {
        Self { counts: [0; 24] }
    }

    fn push(&mut self, character: u8) {
        self.counts[(character - b'A') as usize] += 1;
    }

    fn add(&self, other: CharacterCounts) -> CharacterCounts {
        let mut new_count = [0; 24];

        for idx in 0..new_count.len() {
            new_count[idx] = self.counts[idx] + other.counts[idx];
        }

        CharacterCounts { counts: new_count }
    }

    fn solution(&self) -> u64 {
        let min = self.counts.iter().filter(|n| **n > 0).min().unwrap();
        let max = self.counts.iter().max().unwrap();

        max - min
    }
}

#[test]
fn test() {
    let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    assert_eq!(1588, part_one(input));
    assert_eq!(2188189693529, part_two(input));
}
