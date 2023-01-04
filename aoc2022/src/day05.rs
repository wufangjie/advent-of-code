use crate::read_lines;
use regex::Regex;

pub fn helper(reverse: bool) -> String {
    let mut iter = read_lines("./data/day05.txt");

    let mut groups = vec![];
    loop {
        let line = iter.next().unwrap();
        if &line[1..2] == "1" {
            iter.next(); // skip the blank line
            break;
        }

        for (i, c) in line.bytes().into_iter().skip(1).step_by(4).enumerate() {
            match c {
                32 => (), // ' '
                _ => {
                    while i >= groups.len() {
                        groups.push(vec![]);
                    }
                    groups[i].push(c as char);
                }
            }
        }
    }

    for g in &mut groups {
        g.reverse();
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in iter {
        let caps = re.captures(&line).unwrap();
        let triple: Vec<usize> = (1..4)
            .into_iter()
            .map(|i| caps.get(i).unwrap().as_str().parse().unwrap())
            .collect();

        let fid = triple[1] - 1;
        let tid = triple[2] - 1;
        let n1 = groups[fid].len();
        if n1 > 0 {
            let mut temp = Vec::with_capacity(triple[0]);
            for _ in 0..triple[0] {
                match groups[fid].pop() {
                    Some(c) => temp.push(c),
                    None => break,
                }
            }
            if reverse {
                groups[tid].extend(temp.into_iter().rev());
            } else {
                groups[tid].extend(temp);
            }
        }
    }

    let mut res = String::new();
    for g in &mut groups {
        if !g.is_empty() {
            res.push(g.pop().unwrap());
        }
    }
    res
}

pub fn part1() -> String {
    helper(false)
}

pub fn part2() -> String {
    helper(true)
}

#[test]
fn test_05() {
    assert_eq!("QNHWJVJZW", part1());
    assert_eq!("BPCZJLFJW", part2());
}
