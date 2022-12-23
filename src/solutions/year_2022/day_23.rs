use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let mut map = Map::new(input);

    map.tick(SearchDir::patterns().take(10));

    map.score()
}

pub fn part_two(input: &str) -> usize {
    let mut map = Map::new(input);

    map.tick(SearchDir::patterns());

    map.round
}

struct Map {
    elves: HashSet<Point>,
    round: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut elves = HashSet::new();

        for (y, l) in input.trim().lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => {
                        elves.insert(Point {
                            x: x as isize,
                            y: y as isize,
                        });
                    }
                    '.' => (),
                    _ => unreachable!(),
                }
            }
        }

        Self { elves, round: 0 }
    }

    fn tick<I: Iterator<Item = [SearchDir; 4]>>(&mut self, patterns: I) {
        let mut proposals = HashMap::new();
        let mut elf_wishes = HashMap::new();
        let mut new_elves = HashSet::new();

        for pattern in patterns {
            proposals.clear();
            elf_wishes.clear();
            new_elves.clear();

            'next_elf: for &elf in self.elves.iter() {
                let mut alone = true;

                'all_dirs: for dir in elf.all_dirs() {
                    if self.elves.contains(&dir) {
                        alone = false;
                        break 'all_dirs;
                    }
                }

                if !alone {
                    'next_dir: for &dir in pattern.iter() {
                        let movement = elf.movement(dir);
                        for point in elf.search(dir) {
                            if self.elves.contains(&point) {
                                continue 'next_dir;
                            }
                        }

                        proposals
                            .entry(movement)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                        elf_wishes.insert(elf, movement);

                        continue 'next_elf;
                    }
                }

                elf_wishes.insert(elf, elf);
            }

            for (&elf, &target) in elf_wishes.iter() {
                if let Some(1) = proposals.get(&target) {
                    new_elves.insert(target);
                } else {
                    new_elves.insert(elf);
                }
            }

            let mut no_changes = true;
            for elf in new_elves.iter() {
                if !self.elves.contains(elf) {
                    no_changes = false;
                    break;
                }
            }

            self.round += 1;
            std::mem::swap(&mut self.elves, &mut new_elves);

            if no_changes {
                break;
            }
        }
    }

    fn score(&self) -> usize {
        let count = self.elves.len();

        let mut min = Point {
            x: isize::MAX,
            y: isize::MAX,
        };
        let mut max = Point {
            x: isize::MIN,
            y: isize::MIN,
        };

        for Point { x, y } in self.elves.iter().copied() {
            min.x = x.min(min.x);
            max.x = x.max(max.x);
            min.y = y.min(min.y);
            max.y = y.max(max.y);
        }

        let width = min.x.abs_diff(max.x) + 1;
        let height = min.y.abs_diff(max.y) + 1;

        width * height - count
    }
}

#[derive(Debug, Clone, Copy)]
enum SearchDir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn all_dirs(&self) -> [Point; 8] {
        let &Point { x, y } = self;

        [
            Point { x: x - 1, y: y - 1 },
            Point { x: x + 0, y: y - 1 },
            Point { x: x + 1, y: y - 1 },
            Point { x: x + 1, y: y + 0 },
            Point { x: x + 1, y: y + 1 },
            Point { x: x + 0, y: y + 1 },
            Point { x: x - 1, y: y + 1 },
            Point { x: x - 1, y: y + 0 },
        ]
    }
    fn search(&self, dir: SearchDir) -> [Point; 3] {
        let &Point { x, y } = self;

        match dir {
            SearchDir::North => [
                Point { x: x - 1, y: y - 1 },
                Point { x: x + 0, y: y - 1 },
                Point { x: x + 1, y: y - 1 },
            ],
            SearchDir::South => [
                Point { x: x - 1, y: y + 1 },
                Point { x: x + 0, y: y + 1 },
                Point { x: x + 1, y: y + 1 },
            ],
            SearchDir::East => [
                Point { x: x + 1, y: y - 1 },
                Point { x: x + 1, y: y + 0 },
                Point { x: x + 1, y: y + 1 },
            ],
            SearchDir::West => [
                Point { x: x - 1, y: y - 1 },
                Point { x: x - 1, y: y + 0 },
                Point { x: x - 1, y: y + 1 },
            ],
        }
    }

    fn movement(&self, dir: SearchDir) -> Point {
        let &Point { x, y } = self;

        match dir {
            SearchDir::North => Point { x, y: y - 1 },
            SearchDir::South => Point { x, y: y + 1 },
            SearchDir::East => Point { x: x + 1, y },
            SearchDir::West => Point { x: x - 1, y },
        }
    }
}

impl SearchDir {
    fn patterns() -> impl Iterator<Item = [SearchDir; 4]> {
        [
            [
                SearchDir::North,
                SearchDir::South,
                SearchDir::West,
                SearchDir::East,
            ],
            [
                SearchDir::South,
                SearchDir::West,
                SearchDir::East,
                SearchDir::North,
            ],
            [
                SearchDir::West,
                SearchDir::East,
                SearchDir::North,
                SearchDir::South,
            ],
            [
                SearchDir::East,
                SearchDir::North,
                SearchDir::South,
                SearchDir::West,
            ],
        ]
        .into_iter()
        .cycle()
    }
}

#[test]
fn test() {
    let input = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;

    assert_eq!(110, part_one(input));
    assert_eq!(20, part_two(input));
}
