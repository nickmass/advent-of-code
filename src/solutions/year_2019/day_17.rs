use crate::HashMap;

use super::{intcode, Point2};

pub fn part_one(input: &str) -> i32 {
    let mut machine = intcode::Machine::new(input);
    let mut map = HashMap::new();
    let mut col = 0;
    let mut row = 0;
    let mut max_col = 0;
    let mut max_row = 0;
    loop {
        match machine.run() {
            intcode::Interrupt::Input => {}
            intcode::Interrupt::Output(10) => {
                max_col = col.max(max_col);
                col = 0;
                row += 1;
                max_row = row.max(max_row);
            }
            intcode::Interrupt::Output(c) => {
                map.insert((row, col), c);
                col += 1;
            }
            intcode::Interrupt::Halt => {
                break;
            }
        }
    }

    let mut sum = 0;

    for (&(row, col), &c) in map.iter() {
        if c == 35 {
            let top = (row - 1, col);
            let bottom = (row + 1, col);
            let left = (row, col - 1);
            let right = (row, col + 1);

            let top = map.get(&top);
            let bottom = map.get(&bottom);
            let left = map.get(&left);
            let right = map.get(&right);

            match top.zip(bottom).zip(left).zip(right) {
                Some((((35, 35), 35), 35)) => sum += row * col,
                _ => (),
            }
        }
    }

    sum
}

pub fn part_two(input: &str) -> i32 {
    let mut machine = intcode::Machine::new(input);
    machine.poke(0, 2);

    let mut input: Vec<char> = "\nn".chars().collect();

    let mut map = HashMap::new();
    let mut col = 0;
    let mut row = 0;
    let mut max_col = 0;
    let mut max_row = 0;
    let mut running = false;
    let mut mapping = true;

    let mut robot = (Point2::new(0, 0), Robot::Up);

    /*
    let print_map = |map: &HashMap<Point2<i32>, u8>, max_row, max_col| {
        for row in 0..max_row {
            for col in 0..max_col {
                if let Some(&c) = map.get(&Point2::new(col, row)) {
                    print!("{}", c as char);
                }
            }
            println!();
        }
    };
    */

    loop {
        match machine.run() {
            intcode::Interrupt::Output(n) if n > 0x7f => {
                return n;
            }
            intcode::Interrupt::Output(10) if mapping => {
                max_col = col.max(max_col);
                if col == 0 {
                    mapping = false;
                    row = 0;
                    let route = find_route(&map, &robot);
                    for c in route.chars().rev() {
                        input.push(c);
                    }
                //print_map(&map, max_row, max_col);
                } else {
                    row += 1;
                    max_row = row.max(max_row);
                }
                col = 0;
            }
            intcode::Interrupt::Output(c)
                if mapping
                    && (c as u8 == b'v'
                        || c as u8 == b'^'
                        || c as u8 == b'<'
                        || c as u8 == b'>') =>
            {
                let dir = match c as u8 {
                    b'v' => Robot::Down,
                    b'^' => Robot::Up,
                    b'<' => Robot::Left,
                    b'>' => Robot::Right,
                    _ => unreachable!(),
                };
                robot = (Point2::new(col, row), dir);
                map.insert(Point2::new(col, row), b'#');
                col += 1;
            }
            intcode::Interrupt::Output(c) if mapping => {
                map.insert(Point2::new(col, row), c as u8);
                col += 1;
            }
            intcode::Interrupt::Output(10) if running => {
                if col == 0 {
                    //println!();
                } else {
                    col = 0;
                    row += 1;

                    if row == max_row {
                        row = 0;
                        //print_map(&map, max_row, max_col);
                    }
                }
            }
            intcode::Interrupt::Output(c) if running => {
                map.insert(Point2::new(col, row), c as u8);
                col += 1;
            }
            intcode::Interrupt::Input => {
                let inp = input.pop();
                //print!("{}", inp.unwrap_or('\n'));
                machine.set_input(inp.unwrap_or('\n') as i32);
            }
            intcode::Interrupt::Output(10) => {
                if input.len() == 0 {
                    running = true;
                }
                //println!();
            }
            intcode::Interrupt::Output(_c) => {
                //print!("{}", c as u8 as char);
            }
            intcode::Interrupt::Halt => break,
        }
    }

    panic!("robot failed to find exit")
}

#[derive(Copy, Clone, Debug)]
enum Robot {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Move {
    Left,
    Right,
    Forward(u8),
    Subroutine(u8),
}

impl Move {
    fn is_sub(&self) -> bool {
        match *self {
            Move::Subroutine(_) => true,
            _ => false,
        }
    }
}

fn find_route(map: &HashMap<Point2<i32>, u8>, robot: &(Point2<i32>, Robot)) -> String {
    let mut point = robot.0;
    let mut dir = robot.1;

    let mut route = Vec::new();

    let off_left = Point2::new(-1, 0);
    let off_right = Point2::new(1, 0);
    let off_up = Point2::new(0, -1);
    let off_down = Point2::new(0, 1);

    let is_wall = |point, offset| {
        map.get(&(point + offset))
            .map(|c| *c == b'#')
            .unwrap_or(false)
    };

    loop {
        let left = is_wall(point, off_left);
        let right = is_wall(point, off_right);
        let up = is_wall(point, off_up);
        let down = is_wall(point, off_down);

        match (dir, left, right, up, down) {
            (Robot::Up, true, _, _, _) => {
                route.push(Move::Left);
                dir = Robot::Left;
            }
            (Robot::Up, _, true, _, _) => {
                route.push(Move::Right);
                dir = Robot::Right;
            }
            (Robot::Down, true, _, _, _) => {
                route.push(Move::Right);
                dir = Robot::Left;
            }
            (Robot::Down, _, true, _, _) => {
                route.push(Move::Left);
                dir = Robot::Right;
            }
            (Robot::Left, _, _, true, _) => {
                route.push(Move::Right);
                dir = Robot::Up;
            }
            (Robot::Left, _, _, _, true) => {
                route.push(Move::Left);
                dir = Robot::Down;
            }
            (Robot::Right, _, _, true, _) => {
                route.push(Move::Left);
                dir = Robot::Up;
            }
            (Robot::Right, _, _, _, true) => {
                route.push(Move::Right);
                dir = Robot::Down;
            }
            _ => break,
        }

        let move_offset = match dir {
            Robot::Left => off_left,
            Robot::Right => off_right,
            Robot::Up => off_up,
            Robot::Down => off_down,
        };

        let mut move_count = 0;
        while is_wall(point, move_offset) {
            move_count += 1;
            point = point + move_offset;
        }

        route.push(Move::Forward(move_count));
    }

    let remove_sub = |route: &mut Vec<Move>, subroutine: u8, len: usize| {
        let start = route.iter().position(|m| !m.is_sub()).unwrap_or(0);
        let start_end = start + len;
        if start_end > route.len() {
            return;
        }
        if route[start..start_end].iter().any(|m| m.is_sub()) {
            return;
        }
        let mut search_idx = start_end;
        while search_idx + len <= route.len() {
            let search_end = search_idx + len;
            if route[start..start_end] == route[search_idx..search_end] {
                for _ in search_idx..search_end {
                    route.remove(search_idx);
                }
                route.insert(search_idx, Move::Subroutine(subroutine));
            } else {
                search_idx += 1;
            }
        }

        for _ in start..start_end {
            route.remove(start);
        }
        route.insert(start, Move::Subroutine(subroutine));
    };

    let mut answer = None;
    'outer: for a in 1..10 {
        let mut route = route.clone();
        remove_sub(&mut route, 0, a);

        for b in 1..10 {
            let mut route = route.clone();
            remove_sub(&mut route, 1, b);

            for c in 1..10 {
                let mut route = route.clone();
                remove_sub(&mut route, 2, c);

                if route.iter().all(|m| m.is_sub()) {
                    answer = Some((a, b, c));
                    break 'outer;
                }
            }
        }
    }

    if let Some((a_len, b_len, c_len)) = answer {
        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();
        let mut main = String::new();

        for m in route.iter().filter(|m| !m.is_sub()).take(a_len) {
            if a.len() != 0 {
                a.push(',');
            }
            match m {
                Move::Right => a.push('R'),
                Move::Left => a.push('L'),
                Move::Forward(n) => a.push_str(n.to_string().as_str()),
                _ => (),
            }
        }
        remove_sub(&mut route, 0, a_len);

        for m in route.iter().filter(|m| !m.is_sub()).take(b_len) {
            if b.len() != 0 {
                b.push(',');
            }
            match m {
                Move::Right => b.push('R'),
                Move::Left => b.push('L'),
                Move::Forward(n) => b.push_str(n.to_string().as_str()),
                _ => (),
            }
        }
        remove_sub(&mut route, 1, b_len);

        for m in route.iter().filter(|m| !m.is_sub()).take(c_len) {
            if c.len() != 0 {
                c.push(',');
            }
            match m {
                Move::Right => c.push('R'),
                Move::Left => c.push('L'),
                Move::Forward(n) => c.push_str(n.to_string().as_str()),
                _ => (),
            }
        }
        remove_sub(&mut route, 2, c_len);

        for m in route {
            if main.len() != 0 {
                main.push(',');
            }
            match m {
                Move::Subroutine(0) => main.push('A'),
                Move::Subroutine(1) => main.push('B'),
                Move::Subroutine(2) => main.push('C'),
                _ => (),
            }
        }
        format!("{}\n{}\n{}\n{}\n", main, a, b, c)
    } else {
        panic!("route not found")
    }
}
