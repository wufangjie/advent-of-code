use crate::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};

const FILENAME: &str = "./data/day23.txt";

fn next_pos(i: i64, j: i64, seq: &VecDeque<char>, pos: &HashSet<(i64, i64)>) -> Option<(i64, i64)> {
    let mut count = 0;
    for ii in i - 1..=i + 1 {
        for jj in j - 1..=j + 1 {
            if pos.contains(&(ii, jj)) {
                count += 1;
            }
        }
    }
    if count == 1 {
        return None;
    }
    for c in seq {
        let (dis, djs) = match c {
            'N' => ([-1, -1, -1], [-1, 0, 1]),
            'S' => ([1, 1, 1], [-1, 0, 1]),
            'W' => ([-1, 0, 1], [-1, -1, -1]),
            'E' => ([-1, 0, 1], [1, 1, 1]),
            _ => unreachable!(),
        };

        let mut no_elf = true;
        for k in 0..3 {
            if pos.contains(&(i + dis[k], j + djs[k])) {
                no_elf = false;
                break;
            }
        }
        if no_elf {
            return Some((i + dis[1], j + djs[1]));
        }
    }
    None
}

pub fn part1() -> i64 {
    let mut lst: Vec<(i64, i64)> = vec![];
    for (i, line) in read_lines(FILENAME).enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if c == b'#' {
                lst.push((i as i64, j as i64));
            }
        }
    }

    let n = lst.len();
    let mut pos: HashSet<(i64, i64)> = lst.iter().copied().collect();
    let mut count: HashMap<(i64, i64), usize> = HashMap::with_capacity(n);
    let mut lst_nxt = Vec::with_capacity(n);

    let mut seq = VecDeque::with_capacity(4);
    for c in ['N', 'S', 'W', 'E'] {
        seq.push_back(c);
    }

    for _ in 0..10 {
        count.clear();
        lst_nxt.clear();

        for &(i, j) in &lst {
            let p = next_pos(i, j, &seq, &pos);
            lst_nxt.push(p);
            if let Some(pair) = p {
                *count.entry(pair).or_default() += 1;
            }
        }

        for i in 0..n {
            if let Some(pair) = lst_nxt[i] {
                if *count.get(&pair).unwrap() == 1 {
                    lst[i] = pair;
                }
            }
        }

        pos = lst.iter().copied().collect();
        let temp = seq.pop_front().unwrap();
        seq.push_back(temp);
    }
    let mut xmin = i64::MAX;
    let mut xmax = i64::MIN;
    let mut ymin = i64::MAX;
    let mut ymax = i64::MIN;
    for &(i, j) in &lst {
        xmin = xmin.min(j);
        xmax = xmax.max(j);
        ymin = ymin.min(i);
        ymax = ymax.max(i);
    }
    // for y in ymin..=ymax {
    //     for x in xmin..=xmax {
    //         if pos.contains(&(y, x)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    (xmax - xmin + 1) * (ymax - ymin + 1) - n as i64
}

pub fn part2() -> i64 {
    let mut lst: Vec<(i64, i64)> = vec![];
    for (i, line) in read_lines(FILENAME).enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if c == b'#' {
                lst.push((i as i64, j as i64));
            }
        }
    }

    let n = lst.len();
    let mut pos: HashSet<(i64, i64)> = lst.iter().copied().collect();
    let mut count: HashMap<(i64, i64), usize> = HashMap::with_capacity(n);
    let mut lst_nxt = Vec::with_capacity(n);

    let mut seq = VecDeque::with_capacity(4);
    for c in ['N', 'S', 'W', 'E'] {
        seq.push_back(c);
    }
    let mut changes;
    let mut round = 0;

    loop {
        count.clear();
        lst_nxt.clear();
        changes = 0;
        round += 1;

        for &(i, j) in &lst {
            let p = next_pos(i, j, &seq, &pos);
            lst_nxt.push(p);
            if let Some(pair) = p {
                *count.entry(pair).or_default() += 1;
                changes += 1;
            }
        }

        if changes == 0 {
            return round;
        }

        for i in 0..n {
            if let Some(pair) = lst_nxt[i] {
                if *count.get(&pair).unwrap() == 1 {
                    lst[i] = pair;
                }
            }
        }

        pos = lst.iter().copied().collect();
        let temp = seq.pop_front().unwrap();
        seq.push_back(temp);
    }
}

#[test]
fn test_23() {
    assert_eq!(4005, part1());
    assert_eq!(1008, part2()); // release mode 0.80s
}
