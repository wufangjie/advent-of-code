use crate::read_lines;
//use std::fmt;

struct Direction {
    d: i32,
}

impl Direction {
    fn new() -> Self {
        Self { d: 0 }
    }

    fn turn_left(&mut self, degree: i32) {
        self.d -= degree / 90;
    }

    fn turn_right(&mut self, degree: i32) {
        self.d += degree / 90;
    }

    fn to_byte(&self) -> u8 {
        match (self.d % 4 + 4) % 4 {
            0 => b'E',
            1 => b'S',
            2 => b'W',
            3 => b'N',
            _ => unreachable!(),
        }
    }
}

pub fn part1() -> i32 {
    let lines = read_lines("./data/day12.txt");
    let mut d = Direction::new();
    let mut x = 0;
    let mut y = 0;

    for line in lines {
        let mut line = line.bytes();
        let mut op = line.next().unwrap();
        let num = line
            .map(|x| (x - b'0') as i32)
            .reduce(|a, b| 10 * a + b)
            .unwrap();

        if op == b'F' {
            op = d.to_byte();
        }
        match op {
            b'E' => x += num,
            b'S' => y -= num,
            b'W' => x -= num,
            b'N' => y += num,
            b'L' => d.turn_left(num),
            b'R' => d.turn_right(num),
            _ => unreachable!(),
        }
        // dbg!((x, y));
    }
    x.abs() + y.abs()
}

pub fn part2() -> i32 {
    let lines = read_lines("./data/day12.txt");
    let mut x = 0;
    let mut y = 0;
    let mut dx = 10;
    let mut dy = 1;

    for line in lines {
        let mut line = line.bytes();
        let op = line.next().unwrap();
        let num = line
            .map(|x| (x - b'0') as i32)
            .reduce(|a, b| 10 * a + b)
            .unwrap();

        match op {
            b'E' => dx += num,
            b'S' => dy -= num,
            b'W' => dx -= num,
            b'N' => dy += num,
            b'L' => {
                // (dx, dy) = turn_left(dx, dy, num) is invalid (rust 2021)
                // let (dx, dy) = turn_left(dx, dy, num) will drop before }
                let xy = turn_left(dx, dy, num);
                dx = xy.0;
                dy = xy.1;
            }
            b'R' => {
                let xy = turn_right(dx, dy, num);
                dx = xy.0;
                dy = xy.1;
            }
            b'F' => {
                x += num * dx;
                y += num * dy;
            }
            _ => unreachable!(),
        }
        // dbg!((x, y));
    }
    x.abs() + y.abs()
}

#[inline]
fn turn_left(dx: i32, dy: i32, num: i32) -> (i32, i32) {
    match num {
        90 => (-dy, dx),
        180 => (-dx, -dy),
        270 => (dy, -dx),
        _ => (dx, dy),
    }
}

#[inline]
fn turn_right(dx: i32, dy: i32, num: i32) -> (i32, i32) {
    turn_left(dx, dy, 360 - num)
}

#[test]
fn test_12() {
    assert_eq!(2297, part1());
    assert_eq!(89984, part2());
}
