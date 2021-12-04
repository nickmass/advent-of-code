#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Marked,
    Open(u64),
}

struct Board {
    cells: Vec<Cell>,
    winner: Option<u64>,
}

impl Board {
    fn new(data: &str) -> Self {
        let nums = data
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .map(Cell::Open);

        let mut cells = Vec::with_capacity(25);
        cells.extend(nums);

        Self {
            cells,
            winner: None,
        }
    }

    fn mark(&mut self, value: u64) {
        if self.winner() {
            return;
        }

        let mut found = None;

        'outer: for y in 0..5 {
            for x in 0..5 {
                if let Some(cell) = self.get_mut(x, y) {
                    if *cell == Cell::Open(value) {
                        *cell = Cell::Marked;
                        found = Some((x, y));
                        break 'outer;
                    }
                }
            }
        }

        if let Some((x, y)) = found {
            let mut x_winner = true;
            let mut y_winner = true;
            for n in 0..5 {
                if let Some(Cell::Open(_)) = self.get(n, y) {
                    x_winner = false;
                    if !y_winner {
                        break;
                    }
                }

                if let Some(Cell::Open(_)) = self.get(x, n) {
                    y_winner = false;
                    if !x_winner {
                        break;
                    }
                }
            }

            if x_winner || y_winner {
                self.winner = Some(value);
            }
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(y * 5 + x)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * 5 + x)
    }

    fn winner(&self) -> bool {
        self.winner.is_some()
    }

    fn score(&self) -> u64 {
        let sum = self
            .cells
            .iter()
            .map(|c| if let Cell::Open(cell) = c { *cell } else { 0 })
            .sum::<u64>();

        sum * self.winner.unwrap_or_default()
    }
}

pub fn part_one(input: &str) -> u64 {
    let mut sections = input.trim().split("\n\n");

    let draws = sections
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse::<u64>().ok());

    let mut boards: Vec<_> = sections.map(Board::new).collect();

    for draw in draws {
        for board in boards.iter_mut() {
            board.mark(draw);

            if board.winner() {
                return board.score();
            }
        }
    }

    panic!("ran out of draws")
}

pub fn part_two(input: &str) -> u64 {
    let mut sections = input.trim().split("\n\n");

    let draws = sections
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse::<u64>().ok());

    let mut boards: Vec<_> = sections.map(Board::new).collect();

    for draw in draws {
        for board in boards.iter_mut() {
            board.mark(draw);
        }

        if boards.len() == 1 && boards[0].winner() {
            return boards[0].score();
        }

        boards.retain(|b| !b.winner());
    }

    panic!("ran out of draws")
}

#[test]
fn test() {
    let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    assert_eq!(4512, part_one(input));
    assert_eq!(1924, part_two(input));
}
