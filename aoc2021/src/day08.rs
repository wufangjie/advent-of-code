use crate::read_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn part1() -> usize {
    let valid: HashSet<usize> = [2, 3, 4, 7].into_iter().collect();
    read_lines("./data/day8.txt")
        .into_iter()
        .map(|line| {
            line.split(' ')
                .rev()
                .take(4)
                .map(|x| valid.contains(&x.len()) as usize)
                .sum::<usize>()
        })
        .sum()
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day8.txt");
    let re = Regex::new(r"([a-g]+)").unwrap();
    let mut acc = 0;
    for line in lines {
        let mut caps = re.captures_iter(&line);
        let mut nums: Vec<&str> = caps
            .by_ref()
            .take(10)
            .map(|x| x.get(1).unwrap().as_str())
            .collect();
        nums.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut nums_b = [0u8; 10];
        for (i, s) in nums.iter().enumerate() {
            for d in s.as_bytes() {
                nums_b[i] |= 1 << (d - b'a');
            }
        }

        let mut seg = vec![0u8; 7]; // seg[seg(ith)] = 2 ** (input - 'a')
        seg[0] = nums_b[1] - nums_b[0]; // a

        let b_d = nums_b[2] - nums_b[0]; // b | d
        let n8069 = nums_b[9] & nums_b[8] & nums_b[7] & nums_b[6]; // 8 & 069
        let b_g = n8069 & (!nums_b[1]);
        seg[1] = b_d & b_g;
        seg[3] = b_d & (!seg[1]);
        seg[6] = b_g & (!seg[1]);
        seg[2] = nums_b[0] & (!n8069);
        seg[5] = nums_b[0] & (!seg[2]);
        seg[4] = nums_b[9] - nums_b[2] - seg[0] - seg[6];

        let dct: HashMap<u8, usize> = vec![
            127 - seg[3],
            seg[2] | seg[5],
            127 - seg[1] - seg[5],
            127 - seg[1] - seg[4],
            seg[1] | seg[2] | seg[3] | seg[5],
            127 - seg[2] - seg[4],
            127 - seg[2],
            seg[0] | seg[2] | seg[5],
            127,
            127 - seg[4],
        ]
        .into_iter()
        .zip(0..10)
        // .enumerate()
        // .map(|(i, v)| (v, i))
        .collect();

        let mut res = 0;
        for x in caps {
            let mut tmp = 0;
            for c in x.get(1).unwrap().as_str().as_bytes() {
                tmp |= 1 << (c - b'a');
            }
            res *= 10;
            res += dct.get(&tmp).unwrap();
        }
        acc += res;
    }
    acc
}
