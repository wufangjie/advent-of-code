use crate::read_lines;
use std::collections::HashMap;

pub fn part1() -> u64 {
    let (lines, i) = get_lines_and_invalid();
    lines[i]
}

fn get_lines_and_invalid() -> (Vec<u64>, usize) {
    let lines: Vec<u64> = read_lines("./data/day9.txt")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut pre25 = HashMap::new(); // NOTE: same value
    for i in 0..25 {
        *pre25.entry(lines[i]).or_insert(0) += 1;
    }
    'main: for i in 25..lines.len() {
        for j in 1..25 {
            if lines[i] > lines[i - j] && pre25.contains_key(&(lines[i] - lines[i - j])) {
                if pre25.get(&lines[i - 25]).unwrap() == &1 {
                    pre25.remove(&lines[i - 25]);
                } else {
                    *pre25.entry(lines[i - 25]).or_insert(0) -= 1;
                }
                *pre25.entry(lines[i]).or_insert(0) += 1;
                continue 'main;
            }
        }
        return (lines, i);
    }
    unreachable!();
}

pub fn part2() -> u64 {
    let (lines, i) = get_lines_and_invalid();
    let target = lines[i];
    let mut acc = lines.clone();
    let mut lo = 0;
    for i in 1..lines.len() {
        for j in lo..i {
            acc[j] += lines[i];
            if acc[j] == target {
                return min_plus_max(&lines, j, i);
            } else if j == lo && acc[j] > target {
                lo += 1;
            }
        }
    }
    unreachable!();
}

fn min_plus_max(lines: &Vec<u64>, lo: usize, hi: usize) -> u64 {
    assert!(hi - lo > 1);
    let mut min = lines[hi];
    let mut max = lines[hi];
    for i in lo..hi {
        if lines[i] < min {
            min = lines[i];
        } else if lines[i] > max {
            max = lines[i];
        }
    }
    min + max
}
