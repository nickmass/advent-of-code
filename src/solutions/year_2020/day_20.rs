use crate::HashMap;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut count = 0;
    let mut all_tiles = Vec::new();
    let mut cur_tile = None;
    for line in lines {
        if line.len() == 0 {
            count = 0;
            continue;
        }
        if count == 0 {
            let id = line[5..9].parse().unwrap();
            if let Some(cur_tile) = cur_tile.take() {
                all_tiles.push(cur_tile);
            }
            cur_tile = Some(Tile::new(id));
        } else if let Some(cur_tile) = cur_tile.as_mut() {
            cur_tile.add_row(line);
        }

        count += 1;
    }
    if let Some(cur_tile) = cur_tile.take() {
        all_tiles.push(cur_tile);
    }

    let mut corners = 1;

    'outer: for tile in &all_tiles {
        let mut align_count = 0;
        for other_tile in &all_tiles {
            if tile.aligns_with(other_tile) {
                align_count += 1;
                if align_count > 2 {
                    continue 'outer;
                }
            }
        }

        if align_count == 2 {
            corners *= tile.id;
        }
    }

    corners
}

pub fn part_two(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut count = 0;
    let mut all_tiles = VecDeque::new();
    let mut cur_tile: Option<Tile> = None;
    for line in lines {
        if line.len() == 0 {
            count = 0;
            continue;
        }
        if count == 0 {
            let id = line[5..9].parse().unwrap();
            if let Some(cur_tile) = cur_tile.take() {
                all_tiles.push_back(cur_tile);
            }
            cur_tile = Some(Tile::new(id));
        } else if let Some(cur_tile) = cur_tile.as_mut() {
            cur_tile.add_row(line);
        }

        count += 1;
    }
    if let Some(cur_tile) = cur_tile.take() {
        all_tiles.push_back(cur_tile);
    }

    let mut image = TileImage::new();

    while let Some(next_tile) = all_tiles.pop_front() {
        if let Err(tile) = image.attempt_insert(next_tile) {
            all_tiles.push_back(tile);
        }
    }

    let pattern = r#"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "#;

    image.process_pattern(pattern)
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
struct Edges {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

impl Edges {
    fn new(top: u32, bottom: u32, left: u32, right: u32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    fn permutations(self) -> impl Iterator<Item = (Edges, usize)> {
        let mut count = 0;
        std::iter::from_fn(move || {
            count += 1;
            match count {
                1 => Some((self, count)),
                2 => Some((self.rotate(), count)),
                3 => Some((self.rotate().rotate(), count)),
                4 => Some((self.rotate().rotate().rotate(), count)),
                5 => Some((self.vert_flip(), count)),
                6 => Some((self.vert_flip().rotate(), count)),
                7 => Some((self.vert_flip().rotate().rotate(), count)),
                8 => Some((self.vert_flip().rotate().rotate().rotate(), count)),
                _ => None,
            }
        })
    }

    fn vert_flip(self) -> Self {
        let left = self.left.reverse_bits() >> 22;
        let right = self.right.reverse_bits() >> 22;
        let vert_flip = Edges::new(self.bottom, self.top, left, right);
        vert_flip
    }

    fn rotate(self) -> Self {
        let top = self.top.reverse_bits() >> 22;
        let bottom = self.bottom.reverse_bits() >> 22;
        Edges::new(self.right, self.left, top, bottom)
    }

    fn adjacent(self, other: Self) -> Option<RelativePosition> {
        if self.left == other.right {
            Some(RelativePosition::Left)
        } else if self.right == other.left {
            Some(RelativePosition::Right)
        } else if self.top == other.bottom {
            Some(RelativePosition::Above)
        } else if self.bottom == other.top {
            Some(RelativePosition::Below)
        } else {
            None
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        if (x != 0 && x != 9) && (y != 0 && y != 9) {
            panic!("invalid edge cell {} {}", x, y);
        }

        let mut top = self.top;
        let mut bottom = self.bottom;
        let mut left = self.left;
        let mut right = self.right;

        if y == 0 {
            for i_x in 0..10 {
                if i_x == x {
                    return top & 1 == 1;
                }
                top >>= 1;
            }
        } else if y == 9 {
            for i_x in 0..10 {
                if i_x == x {
                    return bottom & 1 == 1;
                }
                bottom >>= 1;
            }
        } else if x == 0 || x == 9 {
            left >>= 1;
            right >>= 1;
            for i_y in 1..9 {
                if x == 0 && i_y == y {
                    return left & 1 == 1;
                }
                left >>= 1;

                if x == 9 && i_y == y {
                    return right & 1 == 1;
                }
                right >>= 1;
            }
        }
        panic!("could not find edge cell")
    }
}

impl std::fmt::Display for Edges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut top = self.top;
        let mut bottom = self.bottom;
        let mut left = self.left;
        let mut right = self.right;

        for _ in 0..10 {
            if top & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            top >>= 1;
        }
        write!(f, "\n")?;

        left >>= 1;
        right >>= 1;
        for _ in 0..8 {
            if left & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            left >>= 1;
            write!(f, "        ")?;

            if right & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            right >>= 1;
            write!(f, "\n")?;
        }

        for _ in 0..10 {
            if bottom & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            bottom >>= 1;
        }

        write!(f, "\n")
    }
}

struct Tile {
    id: u64,
    img: Vec<bool>,
    edges: Edges,
    indexer: Option<Box<dyn TileIndex>>,
}

#[derive(Debug, Copy, Clone)]
enum RelativePosition {
    Left,
    Above,
    Below,
    Right,
}

impl Tile {
    fn new(id: u64) -> Self {
        Self {
            id,
            img: Vec::with_capacity(100),
            edges: Edges::new(0, 0, 0, 0),
            indexer: Some(Box::new(TileIndexIdentity)),
        }
    }

    fn add_row(&mut self, row: &str) {
        assert_eq!(row.len(), 10);
        assert!(row.len() < 100);

        for c in row.chars() {
            self.img.push(c == '#');
        }

        if self.img.len() == 100 {
            let mut bottom = 0;
            for &n in &self.img[0..10] {
                bottom <<= 1;
                bottom |= if n { 1 } else { 0 };
            }
            let mut top = 0;
            for &n in &self.img[90..100] {
                top <<= 1;
                top |= if n { 1 } else { 0 };
            }

            let mut right = 0;
            let right_range = (0..100).step_by(10);
            for n in right_range {
                right <<= 1;
                right |= if self.img[n] { 1 } else { 0 };
            }

            let mut left = 0;
            let left_range = (9..100).step_by(10);
            for n in left_range {
                left <<= 1;
                left |= if self.img[n] { 1 } else { 0 };
            }

            self.edges = Edges::new(top, bottom, left, right);
        }
    }

    fn aligns_with(&self, other: &Tile) -> bool {
        if self.id == other.id {
            false
        } else {
            for (e, _) in other.edges.permutations() {
                if self.edges.adjacent(e).is_some() {
                    return true;
                }
            }
            false
        }
    }

    fn aligns_orientation(&self, other: &Tile) -> Option<(RelativePosition, usize)> {
        if self.id == other.id {
            None
        } else {
            for (e, o) in other.edges.permutations() {
                if let Some(relative) = self.edges.adjacent(e) {
                    return Some((relative, o));
                }
            }
            None
        }
    }

    fn set_orientation(&mut self, code: usize) {
        match code {
            1 => (),
            2 => {
                self.rotate();
            }
            3 => {
                self.rotate().rotate();
            }
            4 => {
                self.rotate().rotate().rotate();
            }
            5 => {
                self.vert_flip();
            }
            6 => {
                self.vert_flip().rotate();
            }
            7 => {
                self.vert_flip().rotate().rotate();
            }
            8 => {
                self.vert_flip().rotate().rotate().rotate();
            }
            _ => panic!("invalid orientation code"),
        };
    }

    fn vert_flip(&mut self) -> &mut Self {
        self.edges = self.edges.vert_flip();
        let old = self.indexer.take().unwrap_or(Box::new(TileIndexIdentity));
        self.indexer = Some(Box::new(VertFlip(old)));
        self
    }

    fn rotate(&mut self) -> &mut Self {
        self.edges = self.edges.rotate();
        let old = self.indexer.take().unwrap_or(Box::new(TileIndexIdentity));
        self.indexer = Some(Box::new(Rotate(old)));
        self
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        let i = self.indexer.as_ref().unwrap();
        let (x, y) = i.coords(x, y);
        self.img[y * 10 + x]
    }

    #[allow(dead_code)]
    fn valid_edge(&self) -> bool {
        let mut is_match = true;
        for y in 0..10 {
            for x in 0..10 {
                if y == 0 || x == 0 || y == 9 || x == 9 {
                    let cell = self.get_cell(x, y);
                    let edge_cell = self.edges.get_cell(x, y);
                    if cell != edge_cell {
                        is_match = false;
                    }
                }
            }
        }
        is_match
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..10 {
            for x in 0..10 {
                let c = if self.get_cell(x, y) { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

trait TileIndex {
    fn coords(&self, x: usize, y: usize) -> (usize, usize);
}

struct TileIndexIdentity;

impl TileIndex for TileIndexIdentity {
    fn coords(&self, x: usize, y: usize) -> (usize, usize) {
        (9 - x, (y as isize - 9).abs() as usize)
    }
}

struct VertFlip(Box<dyn TileIndex>);
impl TileIndex for VertFlip {
    fn coords(&self, x: usize, y: usize) -> (usize, usize) {
        let y = (y as isize - 9).abs() as usize;
        self.0.coords(x, y)
    }
}

struct Rotate(Box<dyn TileIndex>);
impl TileIndex for Rotate {
    fn coords(&self, x: usize, y: usize) -> (usize, usize) {
        let temp = y;
        let y = x;
        let x = 10 - (temp + 1);
        self.0.coords(x, y)
    }
}

struct TilePattern {
    pub width: usize,
    pub height: usize,
    pub cell_count: u64,
    cells: Vec<bool>,
    working_buf: Vec<bool>,
}

impl TilePattern {
    fn new(data: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut cell_count = 0;
        let mut cells = Vec::new();
        for line in data.lines() {
            width = 0;
            for c in line.chars() {
                let active = c == '#';
                if active {
                    cell_count += 1;
                }
                cells.push(active);
                width += 1;
            }
            height += 1;
        }
        let working_buf = Vec::with_capacity(cells.len());

        Self {
            width,
            height,
            cell_count,
            cells,
            working_buf,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.cells[y * self.width + x])
        }
    }

    fn rotate(&mut self) -> &mut Self {
        self.working_buf.clear();

        for x in 0..self.width {
            for y in 0..self.height {
                self.working_buf
                    .push(self.get_cell(self.width - x - 1, y).unwrap());
            }
        }

        let height = self.height;
        self.height = self.width;
        self.width = height;

        std::mem::swap(&mut self.cells, &mut self.working_buf);

        self
    }

    fn vert_flip(&mut self) -> &mut Self {
        self.working_buf.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                self.working_buf
                    .push(self.get_cell(x, self.height - y - 1).unwrap())
            }
        }

        std::mem::swap(&mut self.cells, &mut self.working_buf);

        self
    }
}

impl std::fmt::Display for TilePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();
                if cell {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

struct TileImage {
    tiles: HashMap<u64, Tile>,
    position_to_tile: HashMap<(usize, usize), u64>,
    tile_to_position: HashMap<u64, (usize, usize)>,
    min: (usize, usize),
    max: (usize, usize),
}

const IMAGE_CENTER: usize = usize::MAX / 2;

impl TileImage {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            position_to_tile: HashMap::new(),
            tile_to_position: HashMap::new(),
            min: (IMAGE_CENTER, IMAGE_CENTER),
            max: (IMAGE_CENTER, IMAGE_CENTER),
        }
    }

    fn process_pattern(&self, pattern: &str) -> u64 {
        let mut pattern = TilePattern::new(pattern);
        let (image_x_dim, image_y_dim) = self.borderless_dims();
        let mut cell_count = 0;
        for x in 0..image_x_dim {
            for y in 0..image_y_dim {
                if self.borderless_get_cell(x, y).unwrap() {
                    cell_count += 1;
                }
            }
        }

        let count_matches = |pattern: &TilePattern| {
            let mut count = 0;
            for x in 0..image_x_dim {
                'offset_loop: for y in 0..image_y_dim {
                    for p_x in 0..pattern.width {
                        for p_y in 0..pattern.height {
                            if let Some(image_cell) = self.borderless_get_cell(x + p_x, y + p_y) {
                                let p_cell = pattern.get_cell(p_x, p_y).unwrap();
                                if p_cell && !image_cell {
                                    continue 'offset_loop;
                                }
                            } else {
                                continue 'offset_loop;
                            }
                        }
                    }

                    count += 1;
                }
            }

            count
        };

        let mut orientation = 0;
        while orientation < 8 {
            match orientation {
                0 => (),
                1 | 2 | 3 | 5 | 6 | 7 => {
                    pattern.rotate();
                }
                4 => {
                    pattern.rotate().vert_flip();
                }
                _ => unreachable!("ran past end of loop"),
            }
            let count = count_matches(&pattern);
            if count != 0 {
                return cell_count - (count * pattern.cell_count);
            }
            orientation += 1;
        }

        panic!("pattern never found")
    }

    fn attempt_insert(&mut self, mut tile: Tile) -> Result<(), Tile> {
        let id = tile.id;
        if self.tiles.len() == 0 {
            let pos = (IMAGE_CENTER, IMAGE_CENTER);
            self.tiles.insert(id, tile);
            self.tile_to_position.insert(id, pos);
            self.position_to_tile.insert(pos, id);
            Ok(())
        } else {
            let mut found_place = None;
            for existing_tile in self.tiles.values() {
                if let Some((relative, o_code)) = existing_tile.aligns_orientation(&tile) {
                    tile.set_orientation(o_code);

                    let mut pos = self
                        .tile_to_position
                        .get(&existing_tile.id)
                        .unwrap()
                        .clone();
                    match relative {
                        RelativePosition::Left => pos.0 -= 1,
                        RelativePosition::Above => pos.1 -= 1,
                        RelativePosition::Below => pos.1 += 1,
                        RelativePosition::Right => pos.0 += 1,
                    }

                    if self.position_to_tile.contains_key(&pos) {
                        continue;
                    } else {
                        found_place = Some(pos);
                        break;
                    }
                }
            }
            if let Some(pos) = found_place {
                self.min = (self.min.0.min(pos.0), self.min.1.min(pos.1));
                self.max = (self.max.0.max(pos.0), self.max.1.max(pos.1));
                self.tiles.insert(id, tile);
                self.tile_to_position.insert(id, pos);
                self.position_to_tile.insert(pos, id);
                Ok(())
            } else {
                Err(tile)
            }
        }
    }

    fn dims(&self) -> (usize, usize) {
        (
            (self.max.0 - self.min.0 + 1) * 10,
            (self.max.1 - self.min.1 + 1) * 10,
        )
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<bool> {
        let tile_x = x % 10;
        let tile_y = y % 10;
        let image_x = (x / 10) + self.min.0;
        let image_y = (y / 10) + self.min.1;
        let tile_id = self.position_to_tile.get(&(image_x, image_y))?;
        let tile = self.tiles.get(tile_id)?;
        Some(tile.get_cell(tile_x, tile_y))
    }

    fn borderless_dims(&self) -> (usize, usize) {
        (
            (self.max.0 - self.min.0 + 1) * 8,
            (self.max.1 - self.min.1 + 1) * 8,
        )
    }

    fn borderless_get_cell(&self, x: usize, y: usize) -> Option<bool> {
        let tile_x = x % 8;
        let tile_y = y % 8;
        let image_x = (x / 8) + self.min.0;
        let image_y = (y / 8) + self.min.1;
        let tile_id = self.position_to_tile.get(&(image_x, image_y))?;
        let tile = self.tiles.get(tile_id)?;
        Some(tile.get_cell(tile_x + 1, tile_y + 1))
    }
}

#[allow(unused)]
struct Borderless<'a>(&'a TileImage);

impl<'a> std::fmt::Display for Borderless<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_dim, y_dim) = self.0.borderless_dims();
        for y in 0..=y_dim {
            for x in 0..=x_dim {
                let c = if let Some(b) = self.0.borderless_get_cell(x, y) {
                    if b {
                        '#'
                    } else {
                        '.'
                    }
                } else {
                    ' '
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for TileImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_dim, y_dim) = self.dims();
        for y in 0..=y_dim {
            for x in 0..=x_dim {
                let c = if let Some(b) = self.get_cell(x, y) {
                    if b {
                        '#'
                    } else {
                        '.'
                    }
                } else {
                    ' '
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

    run_a(i, 20899048083289);
    run_b(i, 273);
}
