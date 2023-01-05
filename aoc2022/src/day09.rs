use crate::read_lines;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// return moved or not
    /// NOTE: 这里有一个非常隐蔽的点，本来只有两个点的时候下面的写法是没问题的
    /// 点多了可能会存在，x 和 y 都差 2 的情况
    fn move_toward(&mut self, other: &Self) -> bool {
        if (self.x - other.x).abs() == 2 || (self.y - other.y).abs() == 2 {
            match self.x.cmp(&other.x) {
                Greater => self.x -= 1,
                Less => self.x += 1,
                Equal => (),
            }
            match self.y.cmp(&other.y) {
                Greater => self.y -= 1,
                Less => self.y += 1,
                Equal => (),
            }
            true
        } else {
            false
        }
        // if self.x - other.x == 2 {
        //     self.x -= 1;
        //     self.y = other.y
        // } else if self.x - other.x == -2 {
        //     self.x += 1;
        //     self.y = other.y
        // } else if self.y - other.y == 2 {
        //     self.y -= 1;
        //     self.x = other.x
        // } else if self.y - other.y == -2 {
        //     self.y += 1;
        //     self.x = other.x
        // } else {
        //     return false;
        // }
        // true
    }

    fn move_direct(&mut self, direct: &str) {
        match direct {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => unreachable!(),
        }
    }
}

pub fn part1() -> usize {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    for line in read_lines("./data/day09.txt") {
        if let Some((s1, s2)) = line.split_once(' ') {
            let step = s2.parse().unwrap();
            for _ in 0..step {
                head.move_direct(s1);
                tail.move_toward(&head);
                visited.insert((tail.x, tail.y));
            }
        }
    }
    visited.len()
}

pub fn part2() -> usize {
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let n = 10;
    let mut ps = vec![Point { x: 0, y: 0 }; n];

    for line in read_lines("./data/day09.txt") {
        if let Some((s1, s2)) = line.split_once(' ') {
            let step = s2.parse().unwrap();
            for _ in 0..step {
                ps[0].move_direct(s1);
                for i in 1..n {
                    let pre = ps[i - 1];
                    if !ps[i].move_toward(&pre) {
                        break;
                    }
                }
                visited.insert((ps[n - 1].x, ps[n - 1].y));
            }
        }
    }
    visited.len()
}

#[test]
fn test_09() {
    assert_eq!(5878, part1());
    assert_eq!(2405, part2());
}
