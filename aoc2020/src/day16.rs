use crate::read_string;
use regex::Regex;
use std::collections::HashSet;

pub fn part1() -> u32 {
    let s = read_string("./data/day16.txt").unwrap();
    let parts: Vec<&str> = s.split("\n\n").collect();
    let bounds = merge_bound(parts[0]);

    let mut res = 0;
    for line in parts[2].trim().split('\n').skip(1) {
        res += line
            .split(',')
            .map(|x| x.parse().unwrap())
            .filter(|&x| !is_valid(&bounds, x))
            .sum::<u32>()
    }
    res
}

fn merge_bound(part: &str) -> Vec<(u32, u32)> {
    let mut pairs = vec![];
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    for caps in re.captures_iter(part) {
        pairs.push((
            caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        ));
    }
    pairs.sort_unstable();
    let mut bounds = vec![pairs[0]];
    let mut count = 0;
    for (lo, hi) in pairs.into_iter().skip(1) {
        if lo > bounds[count].1 {
            bounds.push((lo, hi));
            count += 1;
        } else if hi > bounds[count].1 {
            bounds[count].1 = hi;
        }
    }
    bounds
}

fn is_valid(bounds: &[(u32, u32)], v: u32) -> bool {
    let mut lo = 0;
    let mut hi = bounds.len() - 1;
    while hi >= lo {
        let mid = (lo + hi) >> 1;
        if bounds[mid].0 <= v && bounds[mid].1 >= v {
            return true;
        } else if bounds[mid].0 > v {
            if mid == 0 {
                return false;
            }
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    false
}

fn get_valid_rows(parts: &[&str]) -> Vec<Vec<u32>> {
    let bounds = merge_bound(parts[0]);
    let mut valid_rows = vec![];
    for line in parts[2].trim().split('\n').skip(1) {
        let row: Vec<u32> = line.split(',').map(|x| x.parse().unwrap()).collect();
        if row.iter().all(|&x| is_valid(&bounds, x)) {
            valid_rows.push(row);
        }
    }
    valid_rows
}

pub fn part2() -> usize {
    let s = read_string("./data/day16.txt").unwrap();
    let parts: Vec<&str> = s.split("\n\n").collect();

    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut bounds_lst = vec![];
    for line in parts[0].trim().split('\n') {
        let mut bounds = vec![];
        for caps in re.captures_iter(line) {
            bounds.push((
                caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            ));
        }
        bounds_lst.push(bounds); // NOTE: bounds is already sorted
    }

    let mine = parts[1]
        .trim()
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut valid_rows = get_valid_rows(&parts);
    valid_rows.push(mine.clone());

    let n = bounds_lst.len();
    let mut poss = Vec::with_capacity(n);
    for _ in 0..n {
        poss.push((0..n).into_iter().collect::<HashSet<usize>>());
    }

    for row in valid_rows {
        for i in 0..n {
            for j in poss[i].clone() {
                if !is_valid(&bounds_lst[i], row[j]) {
                    poss[i].remove(&j);
                }
            }
        }
    }

    get_match_lst(&mut poss)
        .into_iter()
        .take(6) // fields starts with `departure` is 1-6
        .map(|i| mine[i] as usize)
        .product::<usize>()
}

fn get_match_lst(poss: &mut Vec<HashSet<usize>>) -> Vec<usize> {
    let n = poss.len();
    let mut used = vec![false; n];
    let mut match_lst = vec![0; n];
    let mut i = 0;
    let mut count = 0;
    loop {
        if !used[i] && poss[i].len() == 1 {
            used[i] = true;
            match_lst[i] = *poss[i].iter().next().unwrap();
            count += 1;
            if count == n {
                return match_lst;
            }
            for j in 0..n {
                if !used[j] {
                    poss[j].remove(&match_lst[i]);
                }
            }
        }
        i += 1;
        i %= n;
    }
}

#[test]
fn test_16() {
    assert_eq!(19060, part1());
    assert_eq!(953713095011, part2());
}
