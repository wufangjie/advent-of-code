//use crate::read_lines;
use std::collections::HashMap;

pub fn part1() -> usize {
    calc_nth_start_by(2020, vec![1, 2, 16, 19, 18, 0])
}

fn calc_nth_start_by(n: usize, start_lst: Vec<usize>) -> usize {
    // let mut spoken: HashMap<usize, usize> =
    //     start_lst.iter().enumerate().map(|(x, &y)| (y, x)).collect();
    let m = start_lst.len();
    let mut spoken: HashMap<usize, usize> = start_lst.into_iter().zip(0..m).collect();

    // let mut spoken = HashMap::new();
    // let m = start_lst.len();
    // for i in 0..m {
    //     spoken.insert(start_lst[i], i);
    // }

    let mut nxt = 0;
    for i in m..n - 1 {
        let pre = spoken.insert(nxt, i);
        nxt = if let Some(pre_i) = pre { i - pre_i } else { 0 };
    }
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
    // release mode: <2 sec
    calc_nth_start_by(30000000, vec![1, 2, 16, 19, 18, 0])
}

#[test]
fn test_15() {
    assert_eq!(536, part1());
    assert_eq!(24065124, part2());
}
