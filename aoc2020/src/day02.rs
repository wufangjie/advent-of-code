use crate::read_lines;
use regex::Regex;

pub fn part1() -> usize {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]{1,})$").unwrap();
    let lines = read_lines("./data/day02.txt");

    let mut count_valid = 0;
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let c = caps.get(3).unwrap().as_str().parse::<char>().unwrap() as u8;
        let count = caps
            .get(4)
            .unwrap()
            .as_str()
            .as_bytes() //.chars()
            .iter()
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
    let lines = read_lines("./data/day02.txt");

    let mut count_valid = 0;
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let c = caps.get(3).unwrap().as_str().parse::<char>().unwrap() as u8;
        let s = caps.get(4).unwrap().as_str();
        let lo = caps.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let hi = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let c1 = s.as_bytes()[lo];
        let c2 = s.as_bytes()[hi];
        if c1 != c2 && (c1 == c || c2 == c) {
            count_valid += 1;
        }
    }
    count_valid
}

#[test]
fn test_day02() {
    assert_eq!(493, part1());
    assert_eq!(593, part2());
}
