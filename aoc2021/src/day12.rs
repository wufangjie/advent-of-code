use crate::read_lines;
use std::collections::{HashMap, HashSet};

pub fn part1() -> usize {
    let lines = read_lines("./data/day12.txt");
    let mut connect: HashMap<&str, HashSet<&str>> = HashMap::new();
    for row in lines.iter() {
        let pair: Vec<&str> = row.split('-').collect();
        (*connect.entry(pair[0]).or_insert_with(HashSet::new)).insert(pair[1]);
        (*connect.entry(pair[1]).or_insert_with(HashSet::new)).insert(pair[0]);
    }

    let mut visited = HashSet::new();
    visited.insert("start");
    dfs("start", visited, &connect)
}

fn dfs(p: &str, visited: HashSet<&str>, connect: &HashMap<&str, HashSet<&str>>) -> usize {
    if p == "end" {
        1
    } else {
        let mut acc = 0;
        for p2 in connect.get(&p).unwrap() {
            if !visited.contains(p2) {
                let mut visited2 = visited.clone();
                if p2 >= &"a" {
                    visited2.insert(*p2);
                }
                acc += dfs(p2, visited2, connect);
            }
        }
        acc
    }
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day12.txt");
    let mut connect: HashMap<&str, HashSet<&str>> = HashMap::new();
    for row in lines.iter() {
        let pair: Vec<&str> = row.split('-').collect();
        (*connect.entry(pair[0]).or_insert_with(HashSet::new)).insert(pair[1]);
        (*connect.entry(pair[1]).or_insert_with(HashSet::new)).insert(pair[0]);
    }

    let mut visited = HashSet::new();
    visited.insert("start");
    dfs2("start", "", visited, &connect)
}

fn dfs2(
    p: &str,
    single_cave: &str,
    visited: HashSet<&str>,
    connect: &HashMap<&str, HashSet<&str>>,
) -> usize {
    if p == "end" {
        usize::from(single_cave.is_empty() || single_cave == "#")
        // if single_cave.is_empty() || single_cave == "#" {
        //     1
        // } else {
        //     0
        // }
    } else {
        let mut acc = 0;
        for p2 in connect.get(&p).unwrap() {
            if !visited.contains(p2) {
                let mut visited2 = visited.clone();
                if *p2 >= "a" {
                    visited2.insert(*p2);
                }
                if *p2 == single_cave {
                    acc += dfs2(p2, "#", visited2, connect);
                } else {
                    acc += dfs2(p2, single_cave, visited2, connect);
                }
                if single_cave.is_empty() && *p2 >= "a" && *p2 != "end" {
                    acc += dfs2(p2, p2, visited.clone(), connect);
                }
            }
        }
        acc
    }
}

#[test]
fn test_12() {
    // dbg!(part1());
    // dbg!(part2());
    assert_eq!(4773, part1());
    assert_eq!(116985, part2());
}
