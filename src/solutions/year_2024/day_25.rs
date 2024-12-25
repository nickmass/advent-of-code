pub fn part_one(input: &str) -> usize {
    let mut lines = input.trim().lines();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let lock = line == "#####";

        let mut pins = [0; 5];

        for _ in 0..5 {
            let row = lines.next().unwrap().as_bytes();
            for (r, p) in row.iter().zip(pins.iter_mut()) {
                *p += (*r == b'#') as u8;
            }
        }

        if lock {
            locks.push(Lock(pins));
        } else {
            keys.push(Key(pins));
        }

        let _ = lines.next().unwrap();
    }

    keys.into_iter()
        .map(|k| locks.iter().filter(|l| k.fits(l)).count())
        .sum()
}

pub fn part_two(_input: &str) -> &'static str {
    "Almost there..."
}

#[derive(Debug, Copy, Clone)]
struct Key([u8; 5]);

impl Key {
    fn fits(&self, lock: &Lock) -> bool {
        self.0
            .into_iter()
            .zip(lock.0)
            .map(|(k, l)| k + l)
            .all(|n| n <= 5)
    }
}

#[derive(Debug, Copy, Clone)]
struct Lock([u8; 5]);

#[test]
fn test() {
    let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;

    assert_eq!(3, part_one(input));
}
