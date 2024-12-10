use crate::HashSet;

pub fn part_one(input: &str) -> u32 {
    let lines = input.trim().lines();

    let max_x = lines.clone().next().map(str::len).unwrap();
    let max_y = lines.clone().count();

    let mut map = CellMap::new(max_x, max_y);

    for (y, l) in lines.enumerate() {
        let y = y as i32;
        for (x, n) in l.chars().filter_map(|n| n.to_digit(10)).enumerate() {
            let x = x as i32;
            map.insert((x, y), n);
        }
    }

    let mut sum = 0;

    for y in 0..max_y {
        let y = y as i32;
        for x in 0..max_x {
            let x = x as i32;
            let me = map.get(&(x, y)).unwrap();

            let left = map.get(&(x - 1, y)).unwrap_or(9);
            let right = map.get(&(x + 1, y)).unwrap_or(9);
            let up = map.get(&(x, y - 1)).unwrap_or(9);
            let down = map.get(&(x, y + 1)).unwrap_or(9);

            if me < left && me < right && me < up && me < down {
                sum += me as u32 + 1;
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> i32 {
    let lines = input.trim().lines();

    let max_x = lines.clone().next().map(str::len).unwrap();
    let max_y = lines.clone().count();

    let mut map = CellMap::new(max_x, max_y);

    for (y, l) in lines.enumerate() {
        let y = y as i32;
        for (x, n) in l.chars().filter_map(|n| n.to_digit(10)).enumerate() {
            let x = x as i32;
            map.insert((x, y), n);
        }
    }

    let mut low_points = Vec::new();

    for y in 0..max_y {
        let y = y as i32;
        for x in 0..max_x {
            let x = x as i32;
            let me = map.get(&(x, y)).unwrap();

            let left = map.get(&(x - 1, y)).unwrap_or(9);
            let right = map.get(&(x + 1, y)).unwrap_or(9);
            let up = map.get(&(x, y - 1)).unwrap_or(9);
            let down = map.get(&(x, y + 1)).unwrap_or(9);

            if me < left && me < right && me < up && me < down {
                low_points.push((x, y));
            }
        }
    }

    let mut basins = Vec::with_capacity(low_points.len());
    let mut search = Vec::new();
    let mut searched = HashSet::new();
    for low in low_points {
        search.clear();
        searched.clear();
        search.push(low);
        searched.insert(low);

        let mut basin_size = 0;

        while let Some(next) = search.pop() {
            if let Some(value) = map.get(&next) {
                if value < 9 {
                    basin_size += 1;
                    let left = (next.0 - 1, next.1);
                    let right = (next.0 + 1, next.1);
                    let up = (next.0, next.1 - 1);
                    let down = (next.0, next.1 + 1);

                    if !searched.contains(&left) {
                        searched.insert(left);
                        search.push(left);
                    }
                    if !searched.contains(&right) {
                        searched.insert(right);
                        search.push(right);
                    }
                    if !searched.contains(&up) {
                        searched.insert(up);
                        search.push(up);
                    }
                    if !searched.contains(&down) {
                        searched.insert(down);
                        search.push(down);
                    }
                }
            }
        }

        basins.push(basin_size);
    }

    basins.sort();
    basins.iter().rev().take(3).product()
}

struct CellMap {
    cells: Vec<Option<u8>>,
    stride: usize,
    height: usize,
}

impl CellMap {
    fn new(width: usize, height: usize) -> Self {
        let stride = width;
        let cells = vec![None; width * height];

        CellMap {
            cells,
            stride,
            height,
        }
    }

    fn get(&self, &(x, y): &(i32, i32)) -> Option<u8> {
        if x < 0 || y < 0 || x as usize >= self.stride || y as usize >= self.height {
            None
        } else {
            let x = x as usize;
            let y = y as usize;

            let idx = (y * self.stride) + x;
            self.cells.get(idx).copied().flatten()
        }
    }

    fn insert(&mut self, (x, y): (i32, i32), value: u32) {
        if x >= 0 && y >= 0 && (x as usize) < self.stride && (y as usize) < self.height {
            let x = x as usize;
            let y = y as usize;

            let idx = (y * self.stride) + x;
            if let Some(cell) = self.cells.get_mut(idx) {
                *cell = Some(value as u8);
            }
        }
    }
}

#[test]
fn test() {
    let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    assert_eq!(15, part_one(input));
    assert_eq!(1134, part_two(input));
}
