use crate::read_lines;
use std::collections::HashMap;

pub fn part1() -> usize {
    calc_nth_start_by(2020, vec![1, 2, 16, 19, 18, 0])
}

fn calc_nth_start_by(n: usize, start_lst: Vec<usize>) -> usize {
    let mut spoken = HashMap::new();
    let m = start_lst.len();
    for i in 0..m {
        spoken.insert(start_lst[i], i);
    }
    let mut nxt = 0;
    for i in m..n - 1 {
	let cur = nxt;
	nxt = if let Some(pre_i) = spoken.get(&cur) { i - pre_i } else { 0 };
	spoken.insert(cur, i);
    };
    nxt
}

#[test]
fn test_15_1() {
    assert_eq!(436, calc_nth_start_by(2020, vec![0, 3, 6]));
    assert_eq!(1, calc_nth_start_by(2020, vec![1, 3, 2]));
    assert_eq!(10, calc_nth_start_by(2020, vec![2, 1, 3]));
    assert_eq!(27, calc_nth_start_by(2020, vec![1, 2, 3]));
    assert_eq!(78, calc_nth_start_by(2020, vec![2, 3, 1]));
    assert_eq!(438, calc_nth_start_by(2020, vec![3, 2, 1]));
    assert_eq!(1836, calc_nth_start_by(2020, vec![3, 1, 2]));
}

pub fn part2() -> usize {
    // release mode: 2 sec
    calc_nth_start_by(30000000, vec![1, 2, 16, 19, 18, 0])
}
