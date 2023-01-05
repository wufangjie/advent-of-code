use crate::read_lines;
use std::collections::HashSet;

pub fn part1() -> usize {
    let mut table = vec![];
    for line in read_lines("./data/day08.txt") {
        table.push(line.as_bytes().to_vec()); //iter().copied().collect::<Vec<u8>>());
    }

    let nrow = table.len();
    let ncol = table[0].len();

    let mut can_be_seen = HashSet::new();

    // left right
    for (i, line) in table.iter().enumerate().skip(1).take(nrow - 2) {
        let mut max = line[0];
        for (j, &v) in line.iter().enumerate().skip(1).take(ncol - 2) {
            if v > max {
                max = v;
                can_be_seen.insert((i, j));
            }
        }

        max = line[ncol - 1];
        for (j, &v) in line.iter().enumerate().skip(1).take(ncol - 2).rev() {
            if v > max {
                max = v;
                can_be_seen.insert((i, j));
            }
        }
    }

    // top down
    for j in 1..ncol - 1 {
        let mut max = table[0][j];
        for (i, row) in table.iter().enumerate().skip(1).take(nrow - 2) {
            if row[j] > max {
                max = row[j];
                can_be_seen.insert((i, j));
            }
        }

        max = table[nrow - 1][j];
        for (i, row) in table.iter().enumerate().skip(1).take(nrow - 2).rev() {
            //for i in (1..nrow - 1).into_iter().rev() {
            if row[j] > max {
                max = row[j];
                can_be_seen.insert((i, j));
            }
        }
    }

    2 * (nrow + ncol - 2) + can_be_seen.len()
}

pub fn part2() -> usize {
    let mut table = vec![];
    for line in read_lines("./data/day08.txt") {
        table.push(line.as_bytes().to_vec()); //iter().copied().collect::<Vec<u8>>());
    }

    let nrow = table.len();
    let ncol = table[0].len();

    let mut best = 0;

    for i in 1..nrow - 1 {
        for j in 1..ncol - 1 {
            let score = look_up(i, j, &table)
                * look_down(i, j, &table)
                * look_left(i, j, &table)
                * look_right(i, j, &table);
            if score > best {
                best = score;
            }
        }
    }
    best
}

fn look_up(i: usize, j: usize, table: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for row in table.iter().take(i).rev() {
        count += 1;
        if table[i][j] <= row[j] {
            break;
        }
    }
    count
}

fn look_down(i: usize, j: usize, table: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for row in table.iter().skip(i + 1) {
        count += 1;
        if table[i][j] <= row[j] {
            break;
        }
    }
    count
}

fn look_left(i: usize, j: usize, table: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for &item in table[i].iter().take(j).rev() {
        count += 1;
        if table[i][j] <= item {
            break;
        }
    }
    count
}

fn look_right(i: usize, j: usize, table: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for &item in table[i].iter().skip(j + 1) {
        count += 1;
        if table[i][j] <= item {
            break;
        }
    }
    count
}

#[test]
fn test_08() {
    assert_eq!(1794, part1());
    assert_eq!(199272, part2()); // 第二题，题目有点说的不清楚
}
