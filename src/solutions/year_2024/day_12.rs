use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    let map = Map::new(input);
    let regions = map.find_regions();

    regions.into_iter().map(|r| r.cost()).sum()
}

pub fn part_two(input: &str) -> usize {
    let map = Map::new(input);
    let regions = map.find_regions();

    regions.into_iter().map(|r| r.discount_cost()).sum()
}

struct Map {
    cells: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let input = input.trim().as_bytes();
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::new();

        for &b in input {
            match b {
                b'\n' => {
                    height += 1;
                    width = 0;
                }
                _ => {
                    cells.push(b);
                    width += 1;
                }
            }
        }

        height += 1;

        Self {
            cells,
            width,
            height,
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

    fn neighbors(
        &self,
        (x, y): (i32, i32),
    ) -> impl Iterator<Item = (Option<u8>, (i32, i32), Direction)> + '_ {
        Direction::all().into_iter().copied().map(move |dir| {
            let (x_off, y_off) = dir.offset();
            let p = (x + x_off, y + y_off);
            (self.get(p), p, dir)
        })
    }

    fn find_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();

        let first = (0, 0);
        let Some(mut current) = self.get(first) else {
            return regions;
        };
        let mut search = vec![first];
        let mut to_search = HashSet::new();
        let mut searched = HashSet::new();
        let mut area = HashSet::new();
        let mut perimeter = HashSet::new();
        let mut edged = HashSet::new();

        loop {
            while let Some(p) = search.pop() {
                area.insert(p);
                for (region, p, dir) in self.neighbors(p) {
                    if area.contains(&p) {
                        continue;
                    }
                    if region == Some(current) {
                        search.push(p);
                    } else {
                        perimeter.insert((p, dir));
                    }
                }
            }

            edged.clear();
            let mut edges = 0;
            let mut cur: Option<Edge> = None;
            while edged.len() < perimeter.len() {
                let mut done = true;
                for &(p, d) in &perimeter {
                    if edged.contains(&(p, d)) {
                        continue;
                    }

                    if let Some(cur) = cur.as_mut() {
                        if cur.extend(p, d) {
                            done = false;
                            edged.insert((p, d));
                        }
                    } else {
                        done = false;
                        cur = Some(Edge::new(p, d));
                        edged.insert((p, d));
                    }
                }

                if done {
                    if let Some(_) = cur.take() {
                        edges += 1;
                    }
                }
            }

            if cur.is_some() {
                edges += 1;
            }

            regions.push(Region {
                area: area.len(),
                permimeter: perimeter.len(),
                edges,
            });

            for p in area.drain() {
                to_search.remove(&p);
                searched.insert(p);
            }

            for (p, _) in perimeter.drain() {
                if !searched.contains(&p) {
                    to_search.insert(p);
                }
            }

            let next = to_search
                .iter()
                .copied()
                .filter_map(|p| self.get(p).zip(Some(p)))
                .next();

            let Some((region, next)) = next else {
                break;
            };

            to_search.remove(&next);
            search.push(next);
            current = region;
        }

        regions
    }
}

#[derive(Debug, Copy, Clone)]
struct Region {
    area: usize,
    permimeter: usize,
    edges: usize,
}

impl Region {
    fn cost(&self) -> usize {
        self.area * self.permimeter
    }

    fn discount_cost(&self) -> usize {
        self.area * self.edges
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> &'static [Direction] {
        &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn fixed_flex(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up | Direction::Down => (y, x),
            Direction::Left | Direction::Right => (x, y),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    dir: Direction,
    fixed: i32,
    start: i32,
    end: i32,
}

impl Edge {
    fn new(p: (i32, i32), dir: Direction) -> Self {
        let (fixed, flex) = dir.fixed_flex(p);

        Self {
            dir,
            fixed,
            start: flex,
            end: flex,
        }
    }

    fn extend(&mut self, p: (i32, i32), dir: Direction) -> bool {
        if self.dir != dir {
            return false;
        }

        let (fixed, flex) = dir.fixed_flex(p);

        if self.fixed != fixed {
            return false;
        }

        if self.start - 1 == flex {
            self.start = flex;
            true
        } else if self.end + 1 == flex {
            self.end = flex;
            true
        } else {
            false
        }
    }
}

#[test]
fn test() {
    let input = r#"AAAA
BBCD
BBCC
EEEC
"#;

    assert_eq!(140, part_one(input));
    assert_eq!(80, part_two(input));

    let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

    assert_eq!(1930, part_one(input));
    assert_eq!(1206, part_two(input));
}
