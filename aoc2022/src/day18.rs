use crate::read_lines;
use std::collections::{HashMap, HashSet};

const FILENAME: &str = "./data/day18.txt";

pub fn part1() -> i64 {
    count_surface_simple(read_lines(FILENAME).map(|line| {
        let lst: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        (lst[0], lst[1], lst[2])
    }))
}

fn count_surface_simple(xyz: impl Iterator<Item = (i64, i64, i64)>) -> i64 {
    let mut scanned = HashSet::new();
    let mut res = 0;
    for (x, y, z) in xyz {
        for p in [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ] {
            if !scanned.contains(&p) {
                res += 1;
            } else {
                res -= 1;
            }
        }
        scanned.insert((x, y, z));
    }
    res
}

pub fn part2() -> i64 {
    let mut scanned = HashSet::new();
    let (mut x2, mut y2, mut z2) = (usize::MIN, usize::MIN, usize::MIN);

    for line in read_lines(FILENAME) {
        let xyz: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        scanned.insert((xyz[0], xyz[1], xyz[2]));
        x2 = x2.max(xyz[0] as usize + 1);
        y2 = y2.max(xyz[1] as usize + 1);
        z2 = z2.max(xyz[2] as usize + 1);
    }

    let mut z_max = vec![vec![i64::MIN; y2]; x2];
    let mut z_min = vec![vec![i64::MAX; y2]; x2];
    let mut y_max = vec![vec![i64::MIN; z2]; x2];
    let mut y_min = vec![vec![i64::MAX; z2]; x2];
    let mut x_max = vec![vec![i64::MIN; z2]; y2];
    let mut x_min = vec![vec![i64::MAX; z2]; y2];

    for &(x, y, z) in &scanned {
        if z > z_max[x as usize][y as usize] {
            z_max[x as usize][y as usize] = z;
        }
        if z < z_min[x as usize][y as usize] {
            z_min[x as usize][y as usize] = z;
        }
        if y > y_max[x as usize][z as usize] {
            y_max[x as usize][z as usize] = y;
        }
        if y < y_min[x as usize][z as usize] {
            y_min[x as usize][z as usize] = y;
        }
        if x > x_max[y as usize][z as usize] {
            x_max[y as usize][z as usize] = x;
        }
        if x < x_min[y as usize][z as usize] {
            x_min[y as usize][z as usize] = x;
        }
    }

    // //dbg!(&z_max, &z_min, &y_max, &y_min, &x_max, &x_min);
    // dbg!(x_max[2][5], x_min[2][5]);
    // dbg!(y_max[2][5], y_min[2][5]);
    // dbg!(z_max[2][2], z_min[2][2]);
    //dbg!(y_min[10][9], y_max[10][9]);

    let mut empty = HashMap::new(); // exposed or not

    let mut res = count_surface_simple(scanned.iter().copied());
    for &(x, y, z) in &scanned {
        for p in [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ] {
            if p.0 <= 1
                || p.1 <= 1
                || p.2 <= 1
                || p.0 >= x2 as i64 - 1
                || p.1 >= y2 as i64 - 1
                || p.2 >= z2 as i64 - 1
            {
                continue;
            }
            if !scanned.contains(&p) {
                let mut stack = vec![p];
                let mut batch = HashSet::new();
                batch.insert(p);
                while let Some(p2) = stack.pop() {
                    let mut flag = 0;

                    //dbg!(p2);
                    if let Some(flag_p) = empty.get(&p) {
                        flag = *flag_p;
                        if flag == 1 {
                            res -= 1;
                        }
                    } else if p2.0 >= x_max[p2.1 as usize][p2.2 as usize]
                        || p2.0 <= x_min[p2.1 as usize][p2.2 as usize]
                        || p2.1 >= y_max[p2.0 as usize][p2.2 as usize]
                        || p2.1 <= y_min[p2.0 as usize][p2.2 as usize]
                        || p2.2 >= z_max[p2.0 as usize][p2.1 as usize]
                        || p2.2 <= z_min[p2.0 as usize][p2.1 as usize]
                    {
                        flag = 2;
                    }

                    if flag > 0 {
                        for p3 in batch.drain() {
                            empty.insert(p3, flag);
                        }
                        break;
                    }

                    for p3 in [
                        (p2.0 - 1, p2.1, p2.2),
                        (p2.0 + 1, p2.1, p2.2),
                        (p2.0, p2.1 - 1, p2.2),
                        (p2.0, p2.1 + 1, p2.2),
                        (p2.0, p2.1, p2.2 - 1),
                        (p2.0, p2.1, p2.2 + 1),
                    ] {
                        if !scanned.contains(&p3) && !batch.contains(&p3) {
                            batch.insert(p3);
                            stack.push(p3);
                        }
                    }
                }

                if !batch.is_empty() {
                    for p3 in batch.drain() {
                        empty.insert(p3, 1);
                    }
                    res -= 1;
                }
            }
        }
    }
    res
}

#[test]
fn test_18() {
    assert_eq!(4192, part1());
    assert_eq!(2520, part2());
}
