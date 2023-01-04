use crate::read_lines;
use std::collections::HashSet;

pub fn part1() -> usize {
    let mut res = 0;
    for line in read_lines("./data/day03.txt") {
        let n = line.len() >> 1;
        let s1: HashSet<&u8> = HashSet::from_iter(&line.as_bytes()[..n]);
        for &&c in s1.intersection(&HashSet::from_iter(&line.as_bytes()[n..])) {
            match c {
                x if x > 96 => {
                    res += (c - 96) as usize;
                }
                _ => {
                    res += (c - 65 + 27) as usize;
                }
            }
        }
    }
    res
}

pub fn part2() -> usize {
    let mut res = 0;
    let lines: Vec<String> = read_lines("./data/day03.txt").collect();
    for triple in lines.chunks(3) {
        let mut s1: HashSet<u8> = HashSet::from_iter(triple[0].bytes());
        for line in &triple[1..] {
            let s2: HashSet<u8> = HashSet::from_iter(line.bytes());
            s1.retain(|k| s2.contains(k));
        }

        for c in s1 {
            match c {
                x if x > 96 => {
                    res += (c - 96) as usize;
                }
                _ => {
                    res += (c - 65 + 27) as usize;
                }
            }
        }
    }
    res
}

#[test]
fn test_03() {
    assert_eq!(7597, part1());
    assert_eq!(2607, part2());
}
