use crate::read_lines;
use regex::Regex;
use std::collections::HashMap;

const N: usize = 36;

pub fn part1() -> u64 {
    let lines = read_lines("./data/day14.txt");
    let re_mask = Regex::new(r"^mask = ([01X]+)$").unwrap();
    let re_nums = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut record = HashMap::new();
    let mut caps = re_mask.captures(&lines[0]).unwrap();
    let mut bt = BitN::new(caps.get(1).unwrap().as_str());
    for line in lines.iter().skip(1) {
        if re_mask.is_match(line) {
            caps = re_mask.captures(line).unwrap();
            bt = BitN::new(caps.get(1).unwrap().as_str());
        } else {
            caps = re_nums.captures(line).unwrap();
            let key: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let val: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            record.insert(key, bt.write(val));
        }
    }
    record.values().into_iter().sum()
}

#[derive(Debug)]
struct BitN {
    data: [u8; N],
    mask: [bool; N],
}

impl BitN {
    fn new(s: &str) -> Self {
        let mut data = [0; N];
        let mut mask = [false; N];
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if c == b'X' {
                mask[i] = true;
            } else if c == b'1' {
                data[i] = 1;
            }
        }
        BitN { data, mask }
    }

    fn write(&mut self, n: u64) -> u64 {
        let mut ret = 0;
        for i in 0..N {
            if self.mask[i] {
                ret |= n & (1 << (N - i - 1));
            } else if self.data[i] > 0 {
                ret |= 1 << (N - i - 1);
            }
        }
        ret
    }
}

#[test]
fn test_14_1() {
    let mut bt = BitN::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    assert_eq!(73, bt.write(11));
    assert_eq!(101, bt.write(101));
    assert_eq!(64, bt.write(0));
}

pub fn part2() -> u64 {
    let lines = read_lines("./data/day14.txt");
    let re_mask = Regex::new(r"^mask = ([01X]+)$").unwrap();
    let re_nums = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut record = HashMap::new();
    let mut caps = re_mask.captures(&lines[0]).unwrap();
    let mut bt = BitN2::new(caps.get(1).unwrap().as_str());
    for line in lines.iter().skip(1) {
        if re_mask.is_match(line) {
            caps = re_mask.captures(line).unwrap();
            bt = BitN2::new(caps.get(1).unwrap().as_str());
        } else {
            caps = re_nums.captures(line).unwrap();
            let key: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let val: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            for k in bt.write(key) {
                record.insert(k, val);
            }
        }
    }
    record.values().into_iter().sum()
}

#[derive(Debug)]
struct BitN2 {
    data: [u8; N],
    mask: [bool; N],
    poss: Vec<u64>,
}

impl BitN2 {
    fn new(s: &str) -> Self {
        let mut data = [0; N];
        let mut mask = [false; N];
        let mut poss = vec![0u64];
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if c == b'X' {
                mask[i] = true;
                poss = poss
                    .into_iter()
                    .flat_map(|x| [x, x | (1 << (N - i - 1))].into_iter())
                    .collect();
            } else if c == b'1' {
                data[i] = 1;
            }
        }
        BitN2 { data, mask, poss }
    }

    fn write(&mut self, n: u64) -> Vec<u64> {
        let mut ret = 0u64;
        for i in 0..N {
            if !self.mask[i] {
                let cur = 1 << (N - i - 1);
                if self.data[i] > 0 || n & cur > 0 {
                    ret |= cur;
                }
            }
        }
        self.poss.iter().map(|x| x | ret).collect()
    }
}

#[test]
fn test_14_2() {
    let mut bt = BitN2::new("000000000000000000000000000000X1001X");
    dbg!(&bt.poss);
    dbg!(&bt.data);
    dbg!(&bt.write(42));
}

#[test]
fn test_14() {
    assert_eq!(10717676595607, part1());
    assert_eq!(3974538275659, part2());
}
