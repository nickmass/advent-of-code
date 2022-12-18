use crate::HashSet;

pub fn part_one(input: &str) -> i64 {
    let mut map = HashSet::new();

    let mut faces = 0;
    for point in parse(input) {
        let matched_faces = point
            .neighbors()
            .into_iter()
            .filter(|p| map.contains(p))
            .count() as i64;
        map.insert(point);
        faces += 6 - (matched_faces * 2);
    }

    faces
}

pub fn part_two(input: &str) -> i64 {
    let mut map = HashSet::new();
    let mut possibly_trapped = HashSet::new();
    let mut trapped = HashSet::new();

    let mut max = Point(i32::MIN, i32::MIN, i32::MIN);
    let mut min = Point(i32::MAX, i32::MAX, i32::MAX);

    let mut faces = 0;
    for point in parse(input) {
        let matched_faces = point
            .neighbors()
            .into_iter()
            .filter(|p| map.contains(p))
            .count() as i64;

        map.insert(point);
        possibly_trapped.extend(point.neighbors());

        max = max.max(point);
        min = min.min(point);

        faces += 6 - (matched_faces * 2);
    }

    possibly_trapped = possibly_trapped
        .symmetric_difference(&map)
        .copied()
        .collect();

    for &Point(x, y, z) in possibly_trapped.iter() {
        if search_range(&map, min.0, x, max.0, |n| Point(n, y, z))
            && search_range(&map, min.1, y, max.1, |n| Point(x, n, z))
            && search_range(&map, min.2, z, max.2, |n| Point(x, y, n))
        {
            trapped.insert(Point(x, y, z));
        }
    }

    let trapped_faces = map
        .into_iter()
        .flat_map(|p| p.neighbors())
        .filter(|p| trapped.contains(p))
        .count() as i64;

    faces - trapped_faces
}

fn parse(input: &str) -> impl Iterator<Item = Point> + '_ {
    input.trim().lines().filter_map(|l| {
        let mut p = l.split(",").filter_map(|n| n.parse::<i32>().ok());
        let x = p.next()?;
        let y = p.next()?;
        let z = p.next()?;
        Some(Point(x, y, z))
    })
}

fn search_range<F: Fn(i32) -> Point>(
    map: &HashSet<Point>,
    min: i32,
    mid: i32,
    max: i32,
    f: F,
) -> bool {
    let mut found = false;
    for n in min..=mid {
        let p = f(n);
        if map.contains(&p) {
            found = true;
        }
    }

    if !found {
        return false;
    }

    for n in mid..=max {
        let p = f(n);
        if map.contains(&p) {
            return true;
        }
    }

    false
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point(i32, i32, i32);

impl Point {
    fn neighbors(&self) -> [Point; 6] {
        let Point(x, y, z) = self;

        [
            Point(x + 1, y + 0, z + 0),
            Point(x - 1, y + 0, z + 0),
            Point(x + 0, y + 1, z + 0),
            Point(x + 0, y - 1, z + 0),
            Point(x + 0, y + 0, z + 1),
            Point(x + 0, y + 0, z - 1),
        ]
    }

    fn max(&self, other: Point) -> Point {
        let x = self.0.max(other.0);
        let y = self.1.max(other.1);
        let z = self.2.max(other.2);

        Point(x, y, z)
    }

    fn min(&self, other: Point) -> Point {
        let x = self.0.min(other.0);
        let y = self.1.min(other.1);
        let z = self.2.min(other.2);

        Point(x, y, z)
    }
}

#[test]
fn test() {
    let input = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    assert_eq!(64, part_one(input));
    assert_eq!(58, part_two(input));
}
