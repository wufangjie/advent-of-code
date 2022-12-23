use crate::read_lines;
use std::collections::HashMap;

const N: usize = 5;

fn parse_lines() -> (Vec<i32>, Vec<HashMap<i32, (usize, usize)>>, Vec<i32>) {
    let lines = read_lines("./data/day4.txt");
    let seq: Vec<i32> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    let n = (lines.len() - 1) / 6;
    let mut dct_lst = Vec::with_capacity(n);
    let mut sum_lst = Vec::with_capacity(n);
    let mut dct = HashMap::new();
    for i0 in (2..lines.len()).into_iter().step_by(N + 1) {
        let mut sum = 0;
        for i in 0..N {
            for j in 0..N {
                let v = lines[i0 + i][j * 3..j * 3 + 2]
                    .trim()
                    .parse::<i32>()
                    .unwrap();
                sum += v;
                dct.insert(v, (i, j));
            }
        }
        sum_lst.push(sum);
        dct_lst.push(dct);
        dct = HashMap::new();
    }
    (seq, dct_lst, sum_lst)
}

pub fn part1() -> i32 {
    let (seq, dct_lst, mut sum_lst) = parse_lines();
    let n = sum_lst.len();
    let mut record = vec![[0; 10]; n];
    for k in seq {
        for (idx, dct) in dct_lst.iter().enumerate() {
            if let Some(&(i, j)) = dct.get(&k) {
                record[idx][i] += 1;
                record[idx][N + j] += 1;
                sum_lst[idx] -= k;
                if record[idx][i] == N || record[idx][N + j] == N {
                    return sum_lst[idx] * k;
                }
            }
        }
    }
    unreachable!();
}

pub fn part2() -> i32 {
    let (seq, dct_lst, mut sum_lst) = parse_lines();
    let mut count = sum_lst.len();
    let mut record = vec![[0; 10]; count];
    let mut finished = vec![false; count];
    for k in seq {
        for (idx, dct) in dct_lst.iter().enumerate() {
            if finished[idx] {
                continue;
            }
            if let Some(&(i, j)) = dct.get(&k) {
                record[idx][i] += 1;
                record[idx][N + j] += 1;
                sum_lst[idx] -= k;
                if record[idx][i] == N || record[idx][N + j] == N {
                    if count == 1 {
                        return sum_lst[idx] * k;
                    }
                    finished[idx] = true;
                    count -= 1;
                }
            }
        }
    }
    unreachable!();
}
