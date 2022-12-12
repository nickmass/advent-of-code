use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut lines = input.trim().split("\n\n");
    let dots = lines.next().unwrap().trim().lines();
    let mut folds = lines.next().unwrap().trim().lines();

    let dots: HashSet<_> = dots
        .filter_map(|dot| {
            dot.split_once(",")
                .and_then(|(x, y)| x.parse::<i64>().ok().zip(y.parse::<i64>().ok()))
        })
        .collect();

    let mut working_dots = HashSet::new();

    if let Some(fold) = folds.next() {
        if let Some((front, coord)) = fold.split_once('=') {
            if let Ok(coord) = coord.parse::<i64>() {
                let y_flip = front.ends_with('y');

                for dot in dots {
                    if y_flip && dot.1 > coord {
                        let new_coord = coord - (dot.1 - coord);
                        working_dots.insert((dot.0, new_coord));
                    } else if !y_flip && dot.0 > coord {
                        let new_coord = coord - (dot.0 - coord);
                        working_dots.insert((new_coord, dot.1));
                    } else {
                        working_dots.insert(dot);
                    }
                }
            }
        }
    }

    working_dots.len()
}

pub fn part_two(input: &str) -> String {
    let mut lines = input.trim().split("\n\n");
    let dots = lines.next().unwrap().trim().lines();
    let folds = lines.next().unwrap().trim().lines();

    let mut dots: HashSet<_> = dots
        .filter_map(|dot| {
            dot.split_once(",")
                .and_then(|(x, y)| x.parse::<i64>().ok().zip(y.parse::<i64>().ok()))
        })
        .collect();
    let mut working_dots = HashSet::new();

    for fold in folds {
        if let Some((front, coord)) = fold.split_once('=') {
            if let Ok(coord) = coord.parse::<i64>() {
                let y_flip = front.ends_with('y');

                for dot in dots.drain() {
                    if y_flip && dot.1 > coord {
                        let new_coord = coord - (dot.1 - coord);
                        working_dots.insert((dot.0, new_coord));
                    } else if !y_flip && dot.0 > coord {
                        let new_coord = coord - (dot.0 - coord);
                        working_dots.insert((new_coord, dot.1));
                    } else {
                        working_dots.insert(dot);
                    }
                }

                std::mem::swap(&mut dots, &mut working_dots);
            }
        }
    }

    let mut max_y = 0;
    let mut max_x = 0;

    for (x, y) in dots.iter() {
        max_y = max_y.max(*y + 1);
        max_x = max_x.max(*x + 1);
    }

    let mut result = String::with_capacity((max_x * max_y) as usize * 2);
    for y in 0..max_y {
        for x in 0..max_x {
            if dots.contains(&(x, y)) {
                result.push_str("##");
            } else {
                result.push_str("  ");
            }
        }
        result.push('\n');
    }

    result
}

#[test]
fn test() {
    let input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    assert_eq!(17, part_one(input));
}
