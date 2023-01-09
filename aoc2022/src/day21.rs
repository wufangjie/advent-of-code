use crate::read_lines;
use std::collections::{HashMap, VecDeque};

const FILENAME: &str = "./data/day21.txt";

pub fn part1() -> i64 {
    let origin: Vec<String> = read_lines(FILENAME).collect();
    let mut unknown = HashMap::new();
    let mut known = HashMap::new();
    let mut to_make: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut queue = VecDeque::new();
    for line in &origin {
        let cur = &line[..4];
        if line.len() == 17 && &line[10..11] == " " && &line[12..13] == " " {
            let first = &line[6..10];
            let second = &line[13..];
            unknown.insert(cur.to_string(), (2, first, &line[11..12], second));
            to_make.entry(first).or_default().push(cur);
            to_make.entry(second).or_default().push(cur);
        } else {
            known.insert(cur, line[6..].parse::<i64>().unwrap());
            queue.push_back(cur);
        }
    }
    while let Some(monkey) = queue.pop_front() {
        if !known.contains_key(monkey) {
            let (_, first, op, second) = unknown.get(monkey).unwrap();
            let v1 = known.get(first).unwrap();
            let v2 = known.get(second).unwrap();
            let res = match *op {
                "+" => v1 + v2,
                "-" => v1 - v2,
                "*" => v1 * v2,
                "/" => v1 / v2,
                _ => unreachable!(),
            };
            if monkey == "root" {
                return res;
            }
            known.insert(monkey, res);
        }
        for m2 in to_make.get(monkey).unwrap() {
            let v4 = unknown.get_mut(*m2).unwrap();
            v4.0 -= 1;
            if v4.0 == 0 {
                queue.push_back(*m2);
            }
        }
    }
    unreachable!()
}

pub fn part2() -> i64 {
    let origin: Vec<String> = read_lines(FILENAME).collect();
    let mut unknown = HashMap::new();
    let mut known = HashMap::new();
    let mut to_make: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut queue = VecDeque::new();
    for line in &origin {
        let cur = &line[..4];
        if cur == "humn" {
            continue;
        }

        if line.len() == 17 && &line[10..11] == " " && &line[12..13] == " " {
            let first = &line[6..10];
            let second = &line[13..];
            let op = if cur == "root" { "-" } else { &line[11..12] };
            unknown.insert(cur.to_string(), (2, first, op, second));
            to_make.entry(first).or_default().push(cur);
            to_make.entry(second).or_default().push(cur);
        } else {
            known.insert(cur, line[6..].parse::<i64>().unwrap());
            queue.push_back(cur);
        }
    }
    while let Some(monkey) = queue.pop_front() {
        if !known.contains_key(monkey) {
            let (_, first, op, second) = unknown.get(monkey).unwrap();
            let v1 = known.get(first).unwrap();
            let v2 = known.get(second).unwrap();
            let res = match *op {
                "+" => v1 + v2,
                "-" => v1 - v2,
                "*" => v1 * v2,
                "/" => v1 / v2,
                _ => unreachable!(),
            };
            if monkey == "root" {
                return res;
            }
            known.insert(monkey, res);
        }
        for m2 in to_make.get(monkey).unwrap() {
            let v4 = unknown.get_mut(*m2).unwrap();
            v4.0 -= 1;
            if v4.0 == 0 {
                queue.push_back(*m2);
            }
        }
    }

    // dbg!(&to_make); // NOTE: it is easy because it is linear
    queue.push_back("root");
    known.insert("root", 0);
    while let Some(monkey) = queue.pop_front() {
        let v1 = known.get(monkey).unwrap();
        if monkey == "humn" {
            return *v1;
        }

        let (_, first, op, second) = unknown.get_mut(monkey).unwrap();
        if let Some(v2) = known.get(first) {
            let res = match *op {
                "+" => v1 - v2,
                "-" => v2 - v1,
                "*" => v1 / v2,
                "/" => v2 / v1,
                _ => unreachable!(),
            };
            known.insert(second, res);
            queue.push_back(second);
        } else {
            let v2 = known.get(second).unwrap();
            let res = match *op {
                "+" => v1 - v2,
                "-" => v2 + v1,
                "*" => v1 / v2,
                "/" => v2 * v1,
                _ => unreachable!(),
            };
            known.insert(first, res);
            queue.push_back(first);
        }
    }
    unreachable!()
}

#[test]
fn test_21() {
    assert_eq!(169525884255464, part1());
    assert_eq!(3247317268284, part2());
}
