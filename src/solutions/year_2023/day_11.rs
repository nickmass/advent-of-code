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
    columns: Vec<bool>,
    rows: Vec<bool>,
    galaxies: Vec<(usize, usize)>,
}

impl GalaxyMap {
    fn new(input: &str) -> Self {
        let mut columns = Vec::new();
        let mut rows = Vec::new();
        let mut galaxies = Vec::new();

        for (y, line) in input.trim().lines().enumerate() {
            let mut empty_row = true;
            if columns.len() < line.len() {
                columns.resize(line.len(), false);
            }

            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((x, y));
                    columns[x] = true;
                    empty_row = false;
                }
            }

            rows.push(!empty_row);
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

                total += self.columns[x1..x2]
                    .iter()
                    .map(|&x| if x { 1 } else { MULTIPLIER })
                    .sum::<u64>();

                total += self.rows[y1..y2]
                    .iter()
                    .map(|&y| if y { 1 } else { MULTIPLIER })
                    .sum::<u64>();
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
