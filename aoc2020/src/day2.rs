use crate::read_lines;
use regex::Regex;

pub fn part1() -> usize {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]{1,})$").unwrap();
    let lines = read_lines("./data/day2.txt");

    let mut count_valid = 0;
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let c = caps.get(3).unwrap().as_str().bytes().next().unwrap();
        let count = caps
            .get(4)
            .unwrap()
            .as_str()
            .as_bytes()
            .into_iter()
            .filter(|&&x| x == c)
            .count();
        if count >= caps.get(1).unwrap().as_str().parse().unwrap()
            && count <= caps.get(2).unwrap().as_str().parse().unwrap()
        {
            count_valid += 1;
        }
    }
    count_valid
}

pub fn part2() -> usize {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]{1,})$").unwrap();
    let lines = read_lines("./data/day2.txt");

    let mut count_valid = 0;
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let c = caps.get(3).unwrap().as_str().bytes().next().unwrap();
        let s = caps.get(4).unwrap().as_str();
        let lo = caps.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let hi = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let c1 = s.bytes().nth(lo).unwrap();
        let c2 = s.bytes().nth(hi).unwrap();
        if c1 != c2 && (c1 == c || c2 == c) {
            count_valid += 1;
        }
    }
    count_valid
}
