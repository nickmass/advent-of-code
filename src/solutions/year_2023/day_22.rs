use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let lines = input.trim().lines();
    let mut bricks: Vec<_> = lines.map(Brick::from_str).collect();

    bricks.sort_unstable_by_key(|b| std::cmp::Reverse(b.min.2));

    loop {
        let mut did_drop = false;
        'outer: for i in 0..bricks.len() {
            if bricks[i].on_floor() {
                continue;
            }
            for j in i..bricks.len() {
                if i == j {
                    continue;
                }

                if bricks[j].z_max() != bricks[i].z_min() - 1 {
                    continue;
                }

                for p in bricks[i].spaces_below() {
                    if bricks[j].is_inside(p) {
                        continue 'outer;
                    }
                }
            }

            bricks[i].drop();
            did_drop = true;
        }

        if !did_drop {
            break;
        }
    }

    let mut map = HashMap::new();

    for i in 0..bricks.len() {
        for j in i..bricks.len() {
            if i == j {
                continue;
            }

            if bricks[j].z_max() != bricks[i].z_min() - 1 {
                continue;
            }

            for p in bricks[i].spaces_below() {
                if bricks[j].is_inside(p) {
                    let entry = map.entry(i).or_insert(Vec::new());
                    entry.push(j);

                    break;
                }
            }
        }
    }

    let mut required = crate::HashSet::new();
    for (_, v) in map {
        if v.len() == 1 {
            required.insert(v[0]);
        }
    }

    bricks.len() - required.len()
}

pub fn part_two(input: &str) -> usize {
    let lines = input.trim().lines();
    let mut bricks: Vec<_> = lines.map(Brick::from_str).collect();

    bricks.sort_unstable_by_key(|b| std::cmp::Reverse(b.min.2));

    loop {
        let mut did_drop = false;
        'outer: for i in 0..bricks.len() {
            if bricks[i].on_floor() {
                continue;
            }
            for j in i..bricks.len() {
                if i == j {
                    continue;
                }

                if bricks[j].z_max() != bricks[i].z_min() - 1 {
                    continue;
                }

                for p in bricks[i].spaces_below() {
                    if bricks[j].is_inside(p) {
                        continue 'outer;
                    }
                }
            }

            bricks[i].drop();
            did_drop = true;
        }

        if !did_drop {
            break;
        }
    }

    let mut max = 0;
    let mut falling_bricks = HashSet::new();

    for k in 0..bricks.len() {
        let mut bricks = bricks.clone();
        loop {
            let mut did_drop = false;
            'outer: for i in 0..bricks.len() {
                if i == k {
                    continue;
                }

                if bricks[i].on_floor() {
                    continue;
                }
                for j in i..bricks.len() {
                    if i == j || j == k {
                        continue;
                    }

                    if bricks[j].z_max() != bricks[i].z_min() - 1 {
                        continue;
                    }

                    for p in bricks[i].spaces_below() {
                        if bricks[j].is_inside(p) {
                            continue 'outer;
                        }
                    }
                }

                falling_bricks.insert(i);
                bricks[i].drop();
                did_drop = true;
            }

            if !did_drop {
                break;
            }
        }

        max += falling_bricks.len();
        falling_bricks.clear();
    }

    max
}

#[derive(Debug, Clone)]
struct Brick {
    min: (i32, i32, i32),
    max: (i32, i32, i32),
    z_offset: i32,
}

impl Brick {
    fn from_str(line: &str) -> Self {
        let (a, b) = line.split_once('~').unwrap();

        let a_dims = a.split(',').map(|a| a.parse::<i32>().unwrap());
        let b_dims = b.split(',').map(|b| b.parse::<i32>().unwrap());

        let mut dims = a_dims
            .zip(b_dims)
            .map(|(a, b)| if a > b { (b, a) } else { (a, b) });

        let x_dim = dims.next().unwrap();
        let y_dim = dims.next().unwrap();
        let z_dim = dims.next().unwrap();

        let min = (x_dim.0, y_dim.0, z_dim.0);
        let max = (x_dim.1, y_dim.1, z_dim.1);

        Self {
            min,
            max,
            z_offset: 0,
        }
    }

    fn on_floor(&self) -> bool {
        self.min.2 + self.z_offset == 0
    }

    fn drop(&mut self) {
        self.z_offset -= 1;
    }

    fn z_min(&self) -> i32 {
        self.min.2 + self.z_offset
    }

    fn z_max(&self) -> i32 {
        self.max.2 + self.z_offset
    }

    fn is_inside(&self, (x, y, z): (i32, i32, i32)) -> bool {
        let ((x1, y1, z1), (x2, y2, z2)) = self.volume();
        x >= x1 && x <= x2 && y >= y1 && y <= y2 && z >= z1 && z <= z2
    }

    fn volume(&self) -> ((i32, i32, i32), (i32, i32, i32)) {
        let (x1, y1, z1) = self.min;
        let (x2, y2, z2) = self.max;

        ((x1, y1, z1 + self.z_offset), (x2, y2, z2 + self.z_offset))
    }

    fn spaces_below(&self) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
        let ((x1, y1, z1), (x2, y2, _z2)) = self.volume();
        let mut x = x1;
        let mut y = y1;

        std::iter::from_fn(move || {
            if z1 == 0 {
                return None;
            }
            if x > x2 {
                x = x1;
                y += 1;
                if y > y2 {
                    return None;
                }
            }

            let result = (x, y, z1 - 1);
            x += 1;

            Some(result)
        })
    }
}

#[test]
fn test() {
    let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;

    assert_eq!(5, part_one(input));
    assert_eq!(7, part_two(input));
}
