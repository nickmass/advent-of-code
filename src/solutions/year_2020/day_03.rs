#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Map {
    Empty,
    Tree,
}

pub fn part_one(input: &str) -> u64 {
    let lines: Vec<Vec<Map>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ll| match ll {
                    '.' => Map::Empty,
                    '#' => Map::Tree,
                    _ => Map::Empty,
                })
                .collect()
        })
        .collect();

    let offset_x = 3;
    let offset_y = 1;
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;

    while let Some(spot) = lines.get(y).and_then(|l| l.get(x % l.len())) {
        if spot == &Map::Tree {
            count += 1;
        }

        x += offset_x;
        y += offset_y;
    }

    count
}

pub fn part_two(input: &str) -> u64 {
    let lines: Vec<Vec<Map>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ll| match ll {
                    '.' => Map::Empty,
                    '#' => Map::Tree,
                    _ => Map::Empty,
                })
                .collect()
        })
        .collect();

    let mut total = 1;

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (offset_x, offset_y) in &slopes {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;

        while let Some(spot) = lines.get(y).and_then(|l| l.get(x % l.len())) {
            if spot == &Map::Tree {
                count += 1;
            }

            x += offset_x;
            y += offset_y;
        }

        total *= count;
    }

    total
}

#[test]
fn test() {
    let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;

    assert_eq!(7, part_one(input));
    assert_eq!(336, part_two(input));
}
