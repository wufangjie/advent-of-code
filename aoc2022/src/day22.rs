use crate::read_lines;
use regex::Regex;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;

const FILENAME: &str = "./data/day22.txt";

fn turn_face(facing: i8, rotate: &str) -> i8 {
    match rotate {
        "R" => (facing + 1) % 4,
        "L" => {
            if facing == 0 {
                3
            } else {
                facing - 1
            }
        }
        _ => unreachable!(),
    }
}

fn move_next_n(
    n: usize,
    table: &[Vec<u8>],
    i: usize,
    j: usize,
    facing: i8,
    jump: &mut HashMap<(usize, usize, i8), usize>,
) -> (usize, usize) {
    let (i2, j2) = find_next(table, i, j, facing, jump);
    if table[i2][j2] == b'#' {
        (i, j)
    } else if n == 1 {
        (i2, j2)
    } else {
        move_next_n(n - 1, table, i2, j2, facing, jump)
    }
}

fn find_next(
    table: &[Vec<u8>],
    i: usize,
    j: usize,
    facing: i8,
    jump: &mut HashMap<(usize, usize, i8), usize>,
) -> (usize, usize) {
    let (i2, j2) = match facing {
        0 => (i, j + 1),
        1 => (i + 1, j),
        2 => (i, j - 1),
        3 => (i - 1, j),
        _ => unreachable!(),
    };

    if table[i2][j2] != b' ' {
        (i2, j2)
    } else if let Some(k) = jump.get(&(i2, j2, facing)) {
        match facing {
            0 | 2 => (i, *k),
            _ => (*k, j),
        }
    } else {
        let mut k = 0;
        match facing {
            0 => {
                while table[i][k] == b' ' {
                    k += 1;
                }
            }
            1 => {
                while table[k][j] == b' ' {
                    k += 1;
                }
            }
            2 => {
                k = table[0].len() - 1;
                while table[i][k] == b' ' {
                    k -= 1;
                }
            }
            3 => {
                k = table.len() - 1;
                while table[k][j] == b' ' {
                    k -= 1;
                }
            }
            _ => unreachable!(),
        }
        jump.insert((i2, j2, facing), k);
        match facing {
            0 | 2 => (i, k),
            _ => (k, j),
        }
    }
}

pub fn part1() -> usize {
    let mut origin: Vec<String> = read_lines(FILENAME).collect();
    let path = origin.pop().unwrap();
    let n = origin.len();

    let ncol = origin
        .iter()
        .take(n - 1)
        .map(|line| line.len())
        .max()
        .unwrap()
        + 2;
    let mut table = Vec::with_capacity(n);
    table.push(vec![b' '; ncol]);
    for line in &origin {
        let mut row = vec![b' '; ncol];
        for (i, c) in line.bytes().enumerate() {
            row[i + 1] = c;
        }
        table.push(row);
    }

    let mut jump = HashMap::new();
    let mut facing = 0;
    let mut x = 0;
    let mut y = 1;
    while table[y][x] == b' ' {
        x += 1;
    }

    let re = Regex::new(r"(\d+|[LR])").unwrap();
    let mut iter = re.captures_iter(&path);
    let mut step = iter
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    (y, x) = move_next_n(step, &table, y, x, facing, &mut jump);
    while let Some(caps) = iter.next() {
        let rotate = caps.get(1).unwrap().as_str();
        facing = turn_face(facing, rotate);
        step = iter
            .next()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        (y, x) = move_next_n(step, &table, y, x, facing, &mut jump);
    }

    1000 * y + 4 * x + facing as usize
}

fn move_next_n2(
    n: usize,
    table: &[Vec<u8>],
    i: usize,
    j: usize,
    facing: i8,
    jump: &HashMap<(usize, usize, i8), (usize, usize, i8)>,
) -> (usize, usize, i8) {
    let (i2, j2, facing2) = find_next2(table, i, j, facing, jump);
    if table[i2][j2] == b'#' {
        (i, j, facing)
    } else if n == 1 {
        (i2, j2, facing2)
    } else {
        move_next_n2(n - 1, table, i2, j2, facing2, jump)
    }
}

fn find_next2(
    table: &[Vec<u8>],
    i: usize,
    j: usize,
    facing: i8,
    jump: &HashMap<(usize, usize, i8), (usize, usize, i8)>,
) -> (usize, usize, i8) {
    let (i2, j2) = match facing {
        0 => (i, j + 1),
        1 => (i + 1, j),
        2 => (i, j - 1),
        3 => (i - 1, j),
        _ => unreachable!(),
    };

    if table[i2][j2] != b' ' {
        (i2, j2, facing)
    } else if let Some(res) = jump.get(&(i2, j2, facing)) {
        *res
    } else {
        dbg!(i, j, i2, j2, facing);
        unreachable!()
    }
}

fn get_facing(table: &[Vec<u8>], x1: usize, y1: usize, x2: usize) -> i8 {
    if x1 == x2 {
        if table[y1][x1 - 1] == b' ' {
            0
        } else {
            2
        }
    } else if table[y1 - 1][x1] == b' ' {
        1
    } else {
        3
    }
}

fn make_jump(table: &[Vec<u8>]) -> HashMap<(usize, usize, i8), (usize, usize, i8)> {
    // make it by hand
    let n = 50;
    let mut jump = HashMap::with_capacity(n * 7 * 2);
    for ([mut y1, mut x1, y2, x2], [mut n1, mut m1, n2, m2]) in [
        ([1, n + 1, 1, n * 2 + 1], [n * 3 + 1, 1, n * 4 + 1, 1]),
        ([1, n * 2 + 1, 1, n * 3 + 1], [n * 4, 1, n * 4, n + 1]),
        ([1, n + 1, n + 1, n + 1], [n * 3, 1, n * 2, 1]),
        ([1, n * 3, n + 1, n * 3], [n * 3, n * 2, n * 2, n * 2]),
        (
            [n + 1, n + 1, n * 2 + 1, n + 1],
            [n * 2 + 1, 1, n * 2 + 1, n + 1],
        ),
        (
            [n + 1, n * 2, n * 2 + 1, n * 2],
            [n, n * 2 + 1, n, n * 3 + 1],
        ),
        (
            [n * 3 + 1, n, n * 4 + 1, n],
            [n * 3, n + 1, n * 3, n * 2 + 1],
        ),
    ] {
        let f1 = get_facing(table, x1, y1, x2);
        let f2 = get_facing(table, m1, n1, m2);

        for _ in 0..n {
            match f1 {
                0 => jump.insert((y1, x1 - 1, 2), (n1, m1, f2)),
                1 => jump.insert((y1 - 1, x1, 3), (n1, m1, f2)),
                2 => jump.insert((y1, x1 + 1, 0), (n1, m1, f2)),
                3 => jump.insert((y1 + 1, x1, 1), (n1, m1, f2)),
                _ => unreachable!(),
            };
            match f2 {
                0 => jump.insert((n1, m1 - 1, 2), (y1, x1, f1)),
                1 => jump.insert((n1 - 1, m1, 3), (y1, x1, f1)),
                2 => jump.insert((n1, m1 + 1, 0), (y1, x1, f1)),
                3 => jump.insert((n1 + 1, m1, 1), (y1, x1, f1)),
                _ => unreachable!(),
            };

            match x1.cmp(&x2) {
                Less => x1 += 1,
                Greater => x1 -= 1,
                _ => (),
            }
            match y1.cmp(&y2) {
                Less => y1 += 1,
                Greater => y1 -= 1,
                _ => (),
            }
            match m1.cmp(&m2) {
                Less => m1 += 1,
                Greater => m1 -= 1,
                _ => (),
            }
            match n1.cmp(&n2) {
                Less => n1 += 1,
                Greater => n1 -= 1,
                _ => (),
            }
        }
    }
    jump
}

pub fn part2() -> usize {
    let mut origin: Vec<String> = read_lines(FILENAME).collect();
    let path = origin.pop().unwrap();
    let n = origin.len();

    let ncol = origin
        .iter()
        .take(n - 1)
        .map(|line| line.len())
        .max()
        .unwrap()
        + 2;
    let mut table = Vec::with_capacity(n);
    table.push(vec![b' '; ncol]);
    for line in &origin {
        let mut row = vec![b' '; ncol];
        for (i, c) in line.bytes().enumerate() {
            row[i + 1] = c;
        }
        table.push(row);
    }
    let jump = make_jump(&table);

    let mut facing = 0;
    let mut x = 0;
    let mut y = 1;
    while table[y][x] == b' ' {
        x += 1;
    }

    let re = Regex::new(r"(\d+|[LR])").unwrap();
    let mut iter = re.captures_iter(&path);
    let mut step = iter
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    (y, x, facing) = move_next_n2(step, &table, y, x, facing, &jump);
    while let Some(caps) = iter.next() {
        let rotate = caps.get(1).unwrap().as_str();
        facing = turn_face(facing, rotate);
        step = iter
            .next()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        (y, x, facing) = move_next_n2(step, &table, y, x, facing, &jump);
    }

    1000 * y + 4 * x + facing as usize
}

#[test]
fn test_22() {
    assert_eq!(106094, part1());
    assert_eq!(162038, part2());
}
