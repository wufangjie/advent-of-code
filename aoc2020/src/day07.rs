use crate::read_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn part1() -> usize {
    let lines = read_lines("./data/day07.txt");

    let re = Regex::new(r"(\d+) ([a-z ]+) bag").unwrap();
    let mut dct = HashMap::new();
    for line in lines {
        let kv: Vec<&str> = line.split(" bags contain ").collect();
        for caps in re.captures_iter(kv[1]) {
            let p = dct
                .entry(caps.get(2).unwrap().as_str().to_owned())
                .or_insert(vec![]);
            (*p).push(kv[0].to_owned());
        }
    }
    let start = "shiny gold".to_owned();
    let mut stack = vec![&start];
    let mut count = 0;
    let mut visited = HashSet::new();
    while let Some(cur) = stack.pop() {
        if let Some(outers) = dct.get(cur) {
            for bag in outers {
                if !visited.contains(&bag) {
                    visited.insert(bag);
                    count += 1;
                    stack.push(bag)
                }
            }
        }
    }
    count
}

pub fn part2() -> i32 {
    let lines = read_lines("./data/day07.txt");

    let re = Regex::new(r"(\d+) ([a-z ]+) bag").unwrap();
    let mut dct: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for line in lines {
        let kv: Vec<&str> = line.split(" bags contain ").collect();
        dct.insert(
            kv[0].to_owned(),
            re.captures_iter(kv[1])
                .map(|caps| {
                    (
                        caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        caps.get(2).unwrap().as_str().to_owned(),
                    )
                })
                .collect(),
        );
    }

    let start = "shiny gold".to_owned();
    let mut stack = vec![(1, &start)];
    let mut count = 0;
    // TODO: fn with memo
    while let Some((i, cur)) = stack.pop() {
        if let Some(inners) = dct.get(cur) {
            for &(j, ref bag) in inners {
                stack.push((j * i, bag));
                count += j * i;
            }
        }
    }
    count
}

#[test]
fn test_07() {
    assert_eq!(151, part1());
    assert_eq!(41559, part2());
}
