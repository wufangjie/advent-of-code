use crate::read_lines;
use std::collections::HashMap;

pub fn helper(n: usize) -> usize {
    let line = read_lines("./data/day06.txt").next().unwrap();

    let mut last_index_map = HashMap::new();
    let mut acc = 0; // last unique char length

    for (i, c) in line.chars().enumerate() {
        let j = match last_index_map.get(&c) {
            Some(k) => *k,
            None => i,
        };

        if i != j && i - j <= acc {
            acc = i - j;
        } else {
            acc += 1;
            if acc == n {
                return i + 1;
            }
        }
        // println!("i: {}, j: {}, acc: {}", i, j, acc);
        last_index_map.insert(c, i);
    }
    usize::MAX
}

pub fn part1() -> usize {
    helper(4)
}

pub fn part2() -> usize {
    helper(14)
}

#[test]
fn test_06() {
    assert_eq!(1356, part1());
    assert_eq!(2564, part2());
}
