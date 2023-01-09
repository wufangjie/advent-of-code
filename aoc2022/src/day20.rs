use crate::read_lines;
use std::collections::HashMap; //, HashSet, VecDeque};

const FILENAME: &str = "./data/day20.txt";

fn move_prev(mut i: usize, mut prev: usize, next: usize, lst: &mut [(usize, usize)]) {
    i %= lst.len() - 1;
    if i > 0 {
        debug_assert!(i > 0);
        let cur = lst[prev].1;
        lst[prev].1 = next;
        lst[next].0 = prev;

        for _ in 0..i - 1 {
            prev = lst[prev].0;
        }
        let prev2 = lst[prev].0;
        lst[prev2].1 = cur;
        lst[cur].0 = prev2;
        lst[cur].1 = prev;
        lst[prev].0 = cur;
    }
}

fn move_next(mut i: usize, prev: usize, mut next: usize, lst: &mut [(usize, usize)]) {
    i %= lst.len() - 1;
    if i > 0 {
        debug_assert!(i > 0);
        let cur = lst[prev].1;
        lst[prev].1 = next;
        lst[next].0 = prev;

        for _ in 0..i - 1 {
            next = lst[next].1;
        }

        let next2 = lst[next].1;
        lst[next2].0 = cur;
        lst[cur].1 = next2;
        lst[cur].0 = next;
        lst[next].1 = cur;
    }
}

/// just for visualization
fn get_lst_from(mut i: usize, count: usize, lst: &[(usize, usize)], item_lst: &[i64]) -> Vec<i64> {
    let mut res = Vec::with_capacity(count);
    res.push(item_lst[i]);
    for _ in 1..count {
        i = lst[i].1;
        res.push(item_lst[i]);
    }
    res
}

fn get_score(mut i: usize, lst: &[(usize, usize)], item_lst: &[i64]) -> i64 {
    let mut res = 0;
    for j in 1..3001 {
        i = lst[i].1;
        if j % 1000 == 0 {
            dbg!(item_lst[i]);
            res += item_lst[i];
        }
    }
    res
}

pub fn part1() -> i64 {
    let item_lst: Vec<i64> = read_lines(FILENAME)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let item_dct: HashMap<i64, usize> = item_lst
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    let n = item_lst.len();
    let mut lst: Vec<(usize, usize)> = (0..n)
        .into_iter()
        .map(|i| {
            (
                if i == 0 { n - 1 } else { i - 1 },
                if i == n - 1 { 0 } else { i + 1 },
            )
        })
        .collect();

    for i in 0..n {
        let (prev, next) = lst[i];
        let v = item_lst[i];
        if v > 0 {
            move_next(v as usize, prev, next, &mut lst);
        } else if v < 0 {
            move_prev(-v as usize, prev, next, &mut lst);
        }
    }
    get_score(*item_dct.get(&0).unwrap(), &lst, &item_lst)
}

pub fn part2() -> i64 {
    let item_lst: Vec<i64> = read_lines(FILENAME)
        .map(|x| x.parse::<i64>().unwrap() * 811589153)
        .collect();
    let item_dct: HashMap<i64, usize> = item_lst
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    let n = item_lst.len();
    let mut lst: Vec<(usize, usize)> = (0..n)
        .into_iter()
        .map(|i| {
            (
                if i == 0 { n - 1 } else { i - 1 },
                if i == n - 1 { 0 } else { i + 1 },
            )
        })
        .collect();

    for _ in 0..10 {
        for i in 0..n {
            let (prev, next) = lst[i];
            let v = item_lst[i];
            if v > 0 {
                move_next(v as usize, prev, next, &mut lst);
            } else if v < 0 {
                move_prev(-v as usize, prev, next, &mut lst);
            }
        }
        // dbg!(get_lst_from(*item_dct.get(&0).unwrap(), n, &lst, &item_lst));
    }
    get_score(*item_dct.get(&0).unwrap(), &lst, &item_lst)
}

#[test]
fn test_20() {
    assert_eq!(2203, part1());
    assert_eq!(6641234038999, part2());
}
