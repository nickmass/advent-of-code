use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    let lines = input
        .trim()
        .lines()
        .map(|l| l.chars().filter_map(|n| n.to_digit(10)));

    let mut max_x = 0;
    let mut max_y = 0;
    let mut map = HashMap::new();
    for (y, line) in lines.enumerate() {
        let y = y as i32;
        for (x, octo) in line.enumerate() {
            let x = x as i32;
            map.insert((x, y), octo);

            max_x = max_x.max(x + 1);
        }
        max_y = max_y.max(y + 1);
    }

    let mut sum = 0;
    let mut flashers = Vec::new();

    for _ in 0..100 {
        for y in 0..max_y {
            for x in 0..max_x {
                do_flash((x, y), &mut map, &mut flashers);
            }
        }

        sum += flashers.len();
        for flash in flashers.drain(..) {
            map.insert(flash, 0);
        }
    }

    sum
}

pub fn part_two(input: &str) -> u64 {
    let lines = input
        .trim()
        .lines()
        .map(|l| l.chars().filter_map(|n| n.to_digit(10)));

    let mut max_x = 0;
    let mut max_y = 0;
    let mut map = HashMap::new();
    for (y, line) in lines.enumerate() {
        let y = y as i32;
        for (x, octo) in line.enumerate() {
            let x = x as i32;
            map.insert((x, y), octo);

            max_x = max_x.max(x + 1);
        }
        max_y = max_y.max(y + 1);
    }

    let mut flashers = Vec::new();

    for i in 1.. {
        for y in 0..max_y {
            for x in 0..max_x {
                do_flash((x, y), &mut map, &mut flashers);
            }
        }

        if flashers.len() as i32 == max_x * max_y {
            return i;
        }

        for flash in flashers.drain(..) {
            map.insert(flash, 0);
        }
    }

    panic!("octopuses never synchronized")
}

fn do_flash(
    (x, y): (i32, i32),
    map: &mut HashMap<(i32, i32), u32>,
    flashers: &mut Vec<(i32, i32)>,
) {
    if let Some(me) = map.get_mut(&(x, y)) {
        *me += 1;
        if *me == 10 {
            flashers.push((x, y));
            for y_off in -1..=1 {
                for x_off in -1..=1 {
                    if y_off == 0 && x_off == 0 {
                        continue;
                    }
                    let (x, y) = (x + x_off, y + y_off);
                    do_flash((x, y), map, flashers);
                }
            }
        }
    }
}

#[test]
fn test() {
    let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    assert_eq!(1656, part_one(input));
    assert_eq!(195, part_two(input));
}
