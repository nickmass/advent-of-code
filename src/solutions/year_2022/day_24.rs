use crate::{HashMap, HashSet};
use std::collections::VecDeque;

pub fn part_one(input: &str) -> isize {
    let mut map = Map::new(input);

    map.find_path(Target::End)
}

pub fn part_two(input: &str) -> isize {
    let mut map = Map::new(input);

    map.find_path(Target::Hungry)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Target {
    End,
    Hungry,
    Snacks,
}

struct Map {
    width: isize,
    height: isize,
    start: Point,
    end: Point,
    blizzards: Vec<Blizzard>,
    grids: HashMap<isize, Vec<bool>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut start = None;
        let mut end = None;

        let mut width = 0;
        let mut height = 0;

        let mut blizzards = Vec::new();

        for (y, l) in input.trim().lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;

                match c {
                    '.' => {
                        if start.is_none() {
                            start = Some(Point { x, y });
                        }
                        end = Some(Point { x, y });
                    }
                    '<' => blizzards.push(Blizzard {
                        origin: Point { x, y },
                        direction: Direction::Left,
                    }),
                    '>' => blizzards.push(Blizzard {
                        origin: Point { x, y },
                        direction: Direction::Right,
                    }),
                    '^' => blizzards.push(Blizzard {
                        origin: Point { x, y },
                        direction: Direction::Up,
                    }),
                    'v' => blizzards.push(Blizzard {
                        origin: Point { x, y },
                        direction: Direction::Down,
                    }),
                    _ => (),
                }
            }

            if width < l.len() as isize {
                width = l.len() as isize;
            }

            height += 1;
        }

        Map {
            width,
            height,
            start: start.unwrap(),
            end: end.unwrap(),
            blizzards,
            grids: HashMap::new(),
        }
    }

    fn find_path(&mut self, target: Target) -> isize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let tick = 0;

        queue.push_back((self.start, tick, target));
        while let Some((pos, tick, mut target)) = queue.pop_front() {
            if pos == self.end && target == Target::End {
                return tick;
            } else if pos == self.end && target == Target::Hungry {
                target = Target::Snacks;
            } else if pos == self.start && target == Target::Snacks {
                target = Target::End;
            }

            let tick = tick + 1;

            queue.extend(self.moves_on_tick(pos, tick).filter_map(|m| {
                let pos = pos.movement(m);
                let val = (pos, tick, target);

                if visited.contains(&val) {
                    None
                } else {
                    visited.insert(val);
                    Some(val)
                }
            }));
        }

        0
    }

    fn moves_on_tick(&mut self, player: Point, tick: isize) -> impl Iterator<Item = Movement> {
        let stay_still = self.valid_for_tick(tick, player);
        let right = player.right().x < self.width - 1
            && player != self.start
            && player != self.end
            && self.valid_for_tick(tick, player.right());
        let left = player.left().x >= 1
            && player != self.start
            && player != self.end
            && self.valid_for_tick(tick, player.left());
        let up = (player.up() == self.start || player.up().y >= 1)
            && self.valid_for_tick(tick, player.up());
        let down = (player.down() == self.end || player.down().y < self.height - 1)
            && self.valid_for_tick(tick, player.down());

        [
            right.then_some(Movement::Right),
            down.then_some(Movement::Down),
            stay_still.then_some(Movement::Wait),
            up.then_some(Movement::Up),
            left.then_some(Movement::Left),
        ]
        .into_iter()
        .filter_map(|m| m)
    }

    fn valid_for_tick(&mut self, tick: isize, pos: Point) -> bool {
        let map = self.grids.entry(tick).or_insert_with(|| {
            let size = self.width as usize * self.height as usize;
            let mut map = vec![false; size];
            for blizz in self.blizzards.iter() {
                let pos = blizz.pos_on_tick(tick, self.width, self.height);
                let idx = pos.y * self.width + pos.x;

                map[idx as usize] = true;
            }

            map
        });

        let idx = pos.y * self.width + pos.x;
        !map.get(idx as usize).copied().unwrap_or(false)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn movement(&self, movement: Movement) -> Point {
        match movement {
            Movement::Wait => *self,
            Movement::Left => self.left(),
            Movement::Right => self.right(),
            Movement::Up => self.up(),
            Movement::Down => self.down(),
        }
    }

    fn left(&self) -> Point {
        let &Point { x, y } = self;
        Point { x: x - 1, y }
    }

    fn right(&self) -> Point {
        let &Point { x, y } = self;
        Point { x: x + 1, y }
    }

    fn up(&self) -> Point {
        let &Point { x, y } = self;
        Point { x, y: y - 1 }
    }

    fn down(&self) -> Point {
        let &Point { x, y } = self;
        Point { x, y: y + 1 }
    }
}

#[derive(Debug, Copy, Clone)]
struct Blizzard {
    origin: Point,
    direction: Direction,
}

impl Blizzard {
    fn pos_on_tick(&self, tick: isize, width: isize, height: isize) -> Point {
        let width = width - 2;
        let height = height - 2;
        let x = self.origin.x - 1;
        let y = self.origin.y - 1;
        let Point { x, y } = match self.direction {
            Direction::Left => {
                let x = (x - tick) % width;
                let x = if x < 0 { width + x } else { x };
                Point { x, y }
            }
            Direction::Right => {
                let x = (x + tick) % width;
                Point { x, y }
            }
            Direction::Up => {
                let y = (y - tick) % height;
                let y = if y < 0 { height + y } else { y };
                Point { x, y }
            }
            Direction::Down => {
                let y = (y + tick) % height;
                Point { x, y }
            }
        };

        Point { x: x + 1, y: y + 1 }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
enum Movement {
    Wait,
    Left,
    Right,
    Up,
    Down,
}

#[test]
fn test() {
    let input = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    assert_eq!(18, part_one(input));
    assert_eq!(54, part_two(input));
}
