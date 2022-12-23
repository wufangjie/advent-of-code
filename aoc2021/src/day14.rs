use crate::read_lines;
use std::collections::HashMap;

pub fn part1() -> usize {
    let lines = read_lines("./data/day14.txt");
    let mut polymer: Vec<u8> = lines[0].bytes().collect();

    let mut rules = HashMap::new();
    for line in lines.into_iter().skip(2) {
        let bytes: Vec<u8> = line.bytes().collect();
        rules.insert((bytes[0], bytes[1]), bytes[6]);
    }

    for _ in 0..10 {
        let mut polymer2 = vec![];
        let n_1 = polymer.len() - 1;
        for i in 0..n_1 {
            let val = rules.get(&(polymer[i], polymer[i + 1]));
            polymer2.push(polymer[i]);
            if let Some(&v) = val {
                polymer2.push(v);
            }
        }
        polymer2.push(polymer[n_1]);
        polymer = polymer2;
    }

    let mut count = HashMap::new();
    for c in polymer {
        *count.entry(c).or_insert(0) += 1;
    }

    count.values().max().unwrap() - count.values().min().unwrap()
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day14.txt");
    let polymer: Vec<u8> = lines[0].bytes().collect();
    let mut polymer_pair = HashMap::new();
    for i in 0..polymer.len() - 1 {
        *polymer_pair
            .entry((polymer[i], polymer[i + 1]))
            .or_insert(0) += 1;
    }

    let mut rules = HashMap::new();
    for line in lines.into_iter().skip(2) {
        let bytes: Vec<u8> = line.bytes().collect();
        rules.insert((bytes[0], bytes[1]), bytes[6]);
    }

    for _i in 0..40 {
        let mut polymer_pair2 = HashMap::new();
        for (k, v) in polymer_pair {
            let mid = rules.get(&k);
            if let Some(&c) = mid {
                *polymer_pair2.entry((k.0, c)).or_insert(0) += v;
                *polymer_pair2.entry((c, k.1)).or_insert(0) += v;
            } else {
                *polymer_pair2.entry(k).or_insert(0) += v;
            }
        }
        polymer_pair = polymer_pair2;
    }

    let mut count = HashMap::new();
    for (&k, &v) in &polymer_pair {
        *count.entry(k.1).or_insert(0) += v;
    }
    *count.entry(polymer[0]).or_insert(0) += 1; // add first one
    count.values().max().unwrap() - count.values().min().unwrap()
}

#[test]
fn test_14() {
    assert_eq!(2891, part1());
    assert_eq!(4607749009683, part2());
}
