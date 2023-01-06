use crate::read_lines;
use std::collections::VecDeque;

pub fn part1() -> usize {
    let mut table = vec![];
    for line in read_lines("./data/day12.txt") {
        table.push(line.as_bytes().to_vec());
    }
    let nrow = table.len();
    let ncol = table[0].len();
    let mut dp = vec![vec![usize::MAX; ncol]; nrow];

    let mut queue = VecDeque::new();
    for (i, row) in table.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            if *v == b'S' {
                *v = b'a';
                queue.push_back((i, j, 0));
                break;
            }
        }
        if !queue.is_empty() {
            break;
        }
    }

    while let Some((i, j, mut v)) = queue.pop_front() {
        let old = table[i][j];
        if old == b'E' {
            return v;
        }
        v += 1;

        if i > 0 {
            try_move(i - 1, j, old, v, &mut dp, &mut queue, &table);
        }
        if i + 1 < nrow {
            try_move(i + 1, j, old, v, &mut dp, &mut queue, &table);
        }
        if j > 0 {
            try_move(i, j - 1, old, v, &mut dp, &mut queue, &table);
        }
        if j + 1 < ncol {
            try_move(i, j + 1, old, v, &mut dp, &mut queue, &table);
        }
    }
    usize::MAX
}

fn try_move(
    ii: usize,
    jj: usize,
    old: u8,
    vv: usize,
    dp: &mut [Vec<usize>],
    queue: &mut VecDeque<(usize, usize, usize)>,
    table: &[Vec<u8>],
) {
    if dp[ii][jj] > vv
        && ((table[ii][jj] != b'E' && old + 1 >= table[ii][jj])
            || (table[ii][jj] == b'E' && old > b'x'))
    {
        dp[ii][jj] = vv;
        queue.push_back((ii, jj, vv));
    }
}

/// since we can start at any 'a' but only one target 'E', just do it reversely
pub fn part2() -> usize {
    let mut table = vec![];
    for line in read_lines("./data/day12_test.txt") {
        table.push(line.as_bytes().to_vec());
    }
    let nrow = table.len();
    let ncol = table[0].len();
    let mut dp = vec![vec![usize::MAX; ncol]; nrow];

    let mut queue = VecDeque::new();
    for (i, row) in table.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            if *v == b'S' {
                *v = b'a';
            } else if *v == b'E' {
                queue.push_back((i, j, 0));
            }
        }
    }

    while let Some((i, j, mut v)) = queue.pop_front() {
        let old = table[i][j];
        if old == b'a' {
            return v;
        }
        v += 1;

        if i > 0 {
            try_move2(i - 1, j, old, v, &mut dp, &mut queue, &table);
        }
        if i + 1 < nrow {
            try_move2(i + 1, j, old, v, &mut dp, &mut queue, &table);
        }
        if j > 0 {
            try_move2(i, j - 1, old, v, &mut dp, &mut queue, &table);
        }
        if j + 1 < ncol {
            try_move2(i, j + 1, old, v, &mut dp, &mut queue, &table);
        }
    }
    usize::MAX
}

fn try_move2(
    ii: usize,
    jj: usize,
    old: u8,
    vv: usize,
    dp: &mut [Vec<usize>],
    queue: &mut VecDeque<(usize, usize, usize)>,
    table: &[Vec<u8>],
) {
    if dp[ii][jj] > vv
        && ((old != b'E' && table[ii][jj] + 1 >= old) || (old == b'E' && table[ii][jj] > b'x'))
    {
        dp[ii][jj] = vv;
        queue.push_back((ii, jj, vv));
    }
}

#[test]
fn test_12() {
    assert_eq!(361, part1());
    assert_eq!(354, part2());
}
