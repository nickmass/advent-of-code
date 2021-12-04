use crate::HashSet;

pub fn part_one(input: &str) -> u64 {
    let mut grid = HashSet::new();

    let lines = input.trim().lines();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_z = -1;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 1;

    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;

            let point = (col, row, 0);

            max_x = max_x.max(col);
            max_y = max_y.max(row);
            min_x = min_x.min(col);
            min_y = min_y.min(row);

            if c == '#' {
                grid.insert(point);
            }
        }
    }

    for _ in 0..6 {
        let mut next_grid = HashSet::new();
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    let point = (x, y, z);
                    let count = count_neighbors(&grid, point);
                    let me = grid.contains(&point);

                    if me {
                        if count == 2 || count == 3 {
                            next_grid.insert(point);
                        } else {
                            next_grid.remove(&point);
                        }
                    } else {
                        if count == 3 {
                            max_x = max_x.max(x);
                            max_y = max_y.max(y);
                            max_z = max_z.max(z);
                            min_x = min_x.min(x);
                            min_y = min_y.min(y);
                            min_z = min_z.min(z);
                            next_grid.insert(point);
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    grid.len() as u64
}

pub fn part_two(input: &str) -> u64 {
    let mut grid = HashSet::new();

    let lines = input.trim().lines();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_z = -1;
    let mut min_w = -1;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 1;
    let mut max_w = 1;

    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;

            let point = (col, row, 0, 0);

            max_x = max_x.max(col);
            max_y = max_y.max(row);
            min_x = min_x.min(col);
            min_y = min_y.min(row);

            if c == '#' {
                grid.insert(point);
            }
        }
    }

    for _ in 0..6 {
        let mut next_grid = HashSet::new();
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    for w in (min_w - 1)..=(max_w + 1) {
                        let point = (x, y, z, w);
                        let count = count_neighbors_4d(&grid, point);
                        let me = grid.contains(&point);

                        if me {
                            if count == 2 || count == 3 {
                                next_grid.insert(point);
                            } else {
                                next_grid.remove(&point);
                            }
                        } else {
                            if count == 3 {
                                max_x = max_x.max(x);
                                max_y = max_y.max(y);
                                max_z = max_z.max(z);
                                max_w = max_w.max(w);
                                min_x = min_x.min(x);
                                min_y = min_y.min(y);
                                min_z = min_z.min(z);
                                min_w = min_w.min(w);
                                next_grid.insert(point);
                            }
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }

    grid.len() as u64
}

fn count_neighbors(grid: &HashSet<(i32, i32, i32)>, point: (i32, i32, i32)) -> i32 {
    let mut count = 0;
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            for z_off in -1..=1 {
                if x_off == 0 && y_off == 0 && z_off == 0 {
                    continue;
                }
                let x = point.0 + x_off;
                let y = point.1 + y_off;
                let z = point.2 + z_off;
                let p = (x, y, z);

                if grid.contains(&p) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_neighbors_4d(grid: &HashSet<(i32, i32, i32, i32)>, point: (i32, i32, i32, i32)) -> i32 {
    let mut count = 0;
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            for z_off in -1..=1 {
                for w_off in -1..=1 {
                    if x_off == 0 && y_off == 0 && z_off == 0 && w_off == 0 {
                        continue;
                    }
                    let x = point.0 + x_off;
                    let y = point.1 + y_off;
                    let z = point.2 + z_off;
                    let w = point.3 + w_off;
                    let p = (x, y, z, w);

                    if grid.contains(&p) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#".#.
..#
###"#;

    run_a(i, 112);
    run_b(i, 848);
}
