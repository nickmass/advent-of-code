use crate::HashSet;

pub fn part_one(input: &str) -> u64 {
    solve::<2>(input)
}

pub fn part_two(input: &str) -> u64 {
    solve::<1_000_000>(input)
}

pub fn solve<const N: u64>(input: &str) -> u64 {
    let map = GalaxyMap::new(input);
    map.distances::<N>()
}

struct GalaxyMap {
    columns: HashSet<usize>,
    rows: HashSet<usize>,
    galaxies: Vec<(usize, usize)>,
}

impl GalaxyMap {
    fn new(input: &str) -> Self {
        let mut columns = HashSet::new();
        let mut rows = HashSet::new();
        let mut galaxies = Vec::new();

        for (y, line) in input.trim().lines().enumerate() {
            let mut empty_row = true;

            for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                galaxies.push((x, y));

                empty_row = false;
                columns.insert(x);
            }

            if !empty_row {
                rows.insert(y);
            }
        }

        Self {
            columns,
            rows,
            galaxies,
        }
    }

    fn distances<const MULTIPLIER: u64>(&self) -> u64 {
        let mut total = 0;

        for (i, &(x1, y1)) in self.galaxies.iter().enumerate() {
            for &(x2, y2) in self.galaxies.iter().skip(i) {
                let (x1, x2) = if x2 > x1 { (x1, x2) } else { (x2, x1) };
                let (y1, y2) = if y2 > y1 { (y1, y2) } else { (y2, y1) };

                for x in x1..x2 {
                    if !self.columns.contains(&x) {
                        total += MULTIPLIER;
                    } else {
                        total += 1;
                    }
                }

                for y in y1..y2 {
                    if !self.rows.contains(&y) {
                        total += MULTIPLIER;
                    } else {
                        total += 1;
                    }
                }
            }
        }

        total
    }
}

#[test]
fn test() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    assert_eq!(374, part_one(input));
    assert_eq!(1030, solve::<10>(input));
    assert_eq!(8410, solve::<100>(input));
}
