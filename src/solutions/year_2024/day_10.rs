use crate::HashSet;

pub fn part_one(input: &str) -> u64 {
    Map::new(input).part_one()
}

pub fn part_two(input: &str) -> u64 {
    Map::new(input).part_two()
}

struct Map {
    cells: Vec<u8>,
    width: usize,
    height: usize,
    trailheads: Vec<(i32, i32)>,
}

impl Map {
    fn new(input: &str) -> Self {
        let input = input.trim().as_bytes();
        let mut height = 0;
        let mut width = 0;
        let mut cells = Vec::with_capacity(input.len());
        let mut trailheads = Vec::new();

        for b in input {
            match b {
                b'0'..=b'9' => {
                    let n = b - b'0';
                    cells.push(n);
                    if n == 0 {
                        trailheads.push((width as i32, height as i32));
                    }
                    width += 1;
                }
                b'\n' => {
                    height += 1;
                    width = 0;
                }
                _ => unreachable!("invalid map"),
            }
        }
        height += 1;

        Self {
            cells,
            width,
            height,
            trailheads,
        }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = y * self.width + x;
        self.cells.get(idx).copied()
    }

    fn neighbors(&self, (x, y): (i32, i32)) -> impl Iterator<Item = (u8, (i32, i32))> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(o_x, o_y)| (x + o_x, y + o_y))
            .filter_map(|p| self.get(p).zip(Some(p)))
    }

    fn score<S: Scorer>(&self, mut scorer: S) -> u64 {
        let mut search = self
            .trailheads
            .iter()
            .copied()
            .map(|p| (0, p, p))
            .collect::<Vec<_>>();

        while let Some((prev, p, s)) = search.pop() {
            if prev == 9 {
                scorer.add(s, p);
            } else {
                search.extend(
                    self.neighbors(p)
                        .filter(|(n, _)| *n == prev + 1)
                        .map(|(n, p)| (n, p, s)),
                );
            }
        }

        scorer.score()
    }

    fn part_one(&self) -> u64 {
        let scorer = PartOne::new();
        self.score(scorer)
    }

    fn part_two(&self) -> u64 {
        let scorer = PartTwo::new();
        self.score(scorer)
    }
}

struct PartOne {
    routes: HashSet<((i32, i32), (i32, i32))>,
}

impl PartOne {
    fn new() -> Self {
        Self {
            routes: HashSet::new(),
        }
    }
}

impl Scorer for PartOne {
    fn add(&mut self, start: (i32, i32), end: (i32, i32)) {
        self.routes.insert((start, end));
    }

    fn score(self) -> u64 {
        self.routes.len() as u64
    }
}

struct PartTwo {
    count: u64,
}

impl PartTwo {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Scorer for PartTwo {
    fn add(&mut self, _: (i32, i32), _: (i32, i32)) {
        self.count += 1;
    }

    fn score(self) -> u64 {
        self.count
    }
}

trait Scorer {
    fn add(&mut self, start: (i32, i32), end: (i32, i32));
    fn score(self) -> u64;
}

#[test]
fn test() {
    let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    assert_eq!(36, part_one(input));
    assert_eq!(81, part_two(input));
}
