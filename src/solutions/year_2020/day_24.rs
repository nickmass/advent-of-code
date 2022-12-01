use crate::HashMap;

pub fn part_one(input: &str) -> u64 {
    let tiles: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| {
            let mut tile = Vec::new();
            let mut idx = 0;
            while idx < line.len() {
                let dir = if idx == line.len() - 1 {
                    if &line[idx..] == "e" {
                        HexDir::East
                    } else {
                        HexDir::West
                    }
                } else {
                    match &line[idx..idx + 2] {
                        "sw" => HexDir::SouthWest,
                        "se" => HexDir::SouthEast,
                        "nw" => HexDir::NorthWest,
                        "ne" => HexDir::NorthEast,
                        d if d.starts_with("e") => HexDir::East,
                        d if d.starts_with("w") => HexDir::West,
                        _ => unreachable!(),
                    }
                };

                match dir {
                    HexDir::East | HexDir::West => idx += 1,
                    _ => idx += 2,
                }

                tile.push(dir);
            }

            tile
        })
        .collect();

    let mut flips: HashMap<_, bool> = HashMap::new();

    for tile in tiles.iter() {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        for movement in tile.iter() {
            let (o_x, o_y, o_z) = movement.offset();
            x += o_x;
            y += o_y;
            z += o_z;
        }

        flips
            .entry((x, y, z))
            .and_modify(|f| *f = !*f)
            .or_insert(true);
    }

    flips.values().filter(|f| **f).count() as u64
}

pub fn part_two(input: &str) -> u64 {
    let tiles: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| {
            let mut tile = Vec::new();
            let mut idx = 0;
            while idx < line.len() {
                let dir = if idx == line.len() - 1 {
                    if &line[idx..] == "e" {
                        HexDir::East
                    } else {
                        HexDir::West
                    }
                } else {
                    match &line[idx..idx + 2] {
                        "sw" => HexDir::SouthWest,
                        "se" => HexDir::SouthEast,
                        "nw" => HexDir::NorthWest,
                        "ne" => HexDir::NorthEast,
                        d if d.starts_with("e") => HexDir::East,
                        d if d.starts_with("w") => HexDir::West,
                        _ => unreachable!(),
                    }
                };

                match dir {
                    HexDir::East | HexDir::West => idx += 1,
                    _ => idx += 2,
                }

                tile.push(dir);
            }

            tile
        })
        .collect();

    let mut flips: HashMap<_, bool> = HashMap::new();
    let mut max = (0, 0, 0);
    let mut min = (0, 0, 0);

    for tile in tiles.iter() {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        for movement in tile.iter() {
            let (o_x, o_y, o_z) = movement.offset();
            x += o_x;
            y += o_y;
            z += o_z;
        }

        max.0 = max.0.max(x);
        max.1 = max.1.max(y);
        max.2 = max.2.max(z);

        min.0 = min.0.min(x);
        min.1 = min.1.min(y);
        min.2 = min.2.min(z);

        flips
            .entry((x, y, z))
            .and_modify(|f| *f = !*f)
            .or_insert(true);
    }

    max.0 = max.0.max(max.0 + 1);
    max.1 = max.1.max(max.1 + 1);
    max.2 = max.2.max(max.2 + 1);

    min.0 = min.0.min(min.0 - 1);
    min.1 = min.1.min(min.1 - 1);
    min.2 = min.2.min(min.2 - 1);

    let days = 100;

    let mut hex_grid = HexGrid::new(
        min.0 - days,
        max.0 + days,
        min.1 - days,
        max.1 + days,
        min.2 - days,
        max.2 + days,
    );
    let mut next_grid = hex_grid.clone();

    for (pos, v) in flips.iter() {
        hex_grid.set(*pos, *v);
    }

    for _day in 0..days {
        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                for z in min.2..=max.2 {
                    let neighbors = hex_grid.count_neighbors((x, y, z));
                    let t = match hex_grid.get((x, y, z)) {
                        true if neighbors == 0 || neighbors > 2 => false,
                        false if neighbors == 2 => {
                            max.0 = max.0.max(x + 1);
                            max.1 = max.1.max(y + 1);
                            max.2 = max.2.max(z + 1);

                            min.0 = min.0.min(x - 1);
                            min.1 = min.1.min(y - 1);
                            min.2 = min.2.min(z - 1);

                            true
                        }
                        o => o,
                    };

                    next_grid.set((x, y, z), t);
                }
            }
        }
        std::mem::swap(&mut hex_grid, &mut next_grid);
    }

    hex_grid.count()
}

#[derive(Clone)]
struct HexGrid {
    cells: Vec<bool>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    x_size: usize,
    y_size: usize,
}

impl HexGrid {
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Self {
        let x_size = (x_min.abs() + x_max) as usize + 1;
        let y_size = (y_min.abs() + y_max) as usize + 1;
        let z_size = (z_min.abs() + z_max) as usize + 1;
        Self {
            cells: vec![false; x_size * y_size * z_size],
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            x_size,
            y_size,
        }
    }

    fn get(&self, (x, y, z): (i32, i32, i32)) -> bool {
        if x <= self.x_min
            || x >= self.x_max
            || y <= self.y_min
            || y >= self.y_max
            || z <= self.z_min
            || z >= self.z_max
        {
            panic!("hexgrid out of bounds")
        }

        let x = (x + self.x_min.abs()) as usize;
        let y = (y + self.y_min.abs()) as usize;
        let z = (z + self.z_min.abs()) as usize;

        self.cells[(z * self.y_size * self.x_size) + (y * self.x_size) + x]
    }

    fn set(&mut self, (x, y, z): (i32, i32, i32), val: bool) {
        if x <= self.x_min
            || x >= self.x_max
            || y <= self.y_min
            || y >= self.y_max
            || z <= self.z_min
            || z >= self.z_max
        {
            panic!("hexgrid out of bounds")
        }

        let x = (x + self.x_min.abs()) as usize;
        let y = (y + self.y_min.abs()) as usize;
        let z = (z + self.z_min.abs()) as usize;

        self.cells[(z * self.y_size * self.x_size) + (y * self.x_size) + x] = val
    }

    fn count_neighbors(&self, position: (i32, i32, i32)) -> u32 {
        let mut sum = 0;

        let mut do_sum = |dir: HexDir| {
            let new_dir = dir.add(position);
            if self.get(new_dir) {
                sum += 1;
            }
        };

        do_sum(HexDir::East);
        do_sum(HexDir::SouthEast);
        do_sum(HexDir::SouthWest);
        do_sum(HexDir::West);
        do_sum(HexDir::NorthWest);
        do_sum(HexDir::NorthEast);

        sum
    }

    fn count(&self) -> u64 {
        self.cells.iter().filter(|f| **f).count() as u64
    }
}

#[derive(Copy, Clone, Debug)]
enum HexDir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl HexDir {
    fn offset(&self) -> (i32, i32, i32) {
        match self {
            HexDir::West => (-1, 1, 0),
            HexDir::SouthWest => (-1, 0, 1),
            HexDir::SouthEast => (0, -1, 1),
            HexDir::East => (1, -1, 0),
            HexDir::NorthEast => (1, 0, -1),
            HexDir::NorthWest => (0, 1, -1),
        }
    }

    fn add(&self, other: (i32, i32, i32)) -> (i32, i32, i32) {
        let offset = self.offset();
        (other.0 + offset.0, other.1 + offset.1, other.2 + offset.2)
    }
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

    run_a(i, 10);
    run_b(i, 2208);
}
