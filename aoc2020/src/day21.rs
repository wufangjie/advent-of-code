use crate::read_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn part1() -> usize {
    arranged().0
}

fn arranged() -> (usize, String) {
    let lines = read_lines("./data/day21.txt");
    let re = Regex::new(r"^([a-z ]+) \(contains ([a-z ,]+)\)").unwrap();
    let mut set_lst: Vec<HashSet<&str>> = vec![];
    let mut allergen_dct: HashMap<&str, Vec<usize>> = HashMap::new();
    for i in 0..lines.len() {
        let caps = re.captures(&lines[i]).unwrap();
        set_lst.push(caps.get(1).unwrap().as_str().split(' ').collect());
        for s in caps.get(2).unwrap().as_str().split(", ") {
            (*allergen_dct.entry(s).or_insert(vec![])).push(i);
        }
    }

    let mut poss = HashMap::new();
    for (key, lst) in &allergen_dct {
        let mut left = set_lst[lst[0]].clone();
        for i in 1..lst.len() {
            left.retain(|x| set_lst[lst[i]].contains(x));
        }
        poss.insert(key, left);
    }

    let mut arranged = HashMap::new();
    let mut left: HashMap<&str, usize> = poss.iter().map(|(&&k, v)| (k, v.len())).collect();
    loop {
        for (&key, set) in &poss {
            if 1 == *left.get(*key).unwrap() {
                let mut val = "";
                for v in set {
                    if !arranged.contains_key(v) {
                        val = v;
                        break;
                    }
                }
                if val != "" {
                    arranged.insert(val, *key);
                    for (k, v) in &mut left {
                        if poss.get(&k).unwrap().contains(&val) {
                            *v -= 1;
                        }
                    }
                }
            }
        }
        if poss.len() == arranged.len() {
            break;
        }
    }

    let mut acc = 0;
    for set in set_lst {
        acc += set.iter().filter(|&x| !arranged.contains_key(x)).count();
    }

    let mut temp: Vec<(&str, &str)> = arranged.into_iter().map(|(x, y)| (y, x)).collect();
    temp.sort_unstable();

    (
        acc,
        temp.into_iter()
            .map(|(x, y)| y)
            .collect::<Vec<&str>>()
            .join(","),
    )
}

pub fn part2() -> String {
    arranged().1
}
