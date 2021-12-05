use crate::read_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn part1() -> usize {
    let fields: HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] //, "cid"]
        .into_iter()
        .collect();

    let lines = read_lines("./data/day4.txt");
    let mut passport = HashMap::new();
    let mut count = 0usize;
    for line in lines {
        if line == "" {
            count += (passport.len() == 7) as usize;
            passport.clear();
        } else {
            for pair in line.split(' ') {
                let kv: Vec<&str> = pair.split(':').collect();
                if fields.contains(&kv[0]) {
                    passport.insert(kv[0].to_owned(), kv[1].to_owned());
                }
            }
        }
    }
    count + (passport.len() == 7) as usize
}

pub fn part2() -> usize {
    let fields: HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] //, "cid"]
        .into_iter()
        .collect();

    let lines = read_lines("./data/day4.txt");

    let reg_dct = [
        ("byr", r"^(19[2-9]\d|200[0-2])$"),
        ("iyr", r"^(201\d|2020)$"),
        ("eyr", r"^(202\d|2030)$"),
        ("hgt", r"^((1[5-8]\d|19[0-3])cm|(59|6\d|7[0-6])in)$"),
        ("hcl", r"^#[0-9a-f]{6}$"),
        ("ecl", r"^(amb|blu|brn|gr[yn]|hzl|oth)$"),
        ("pid", r"^\d{9}$"),
    ]
    .into_iter()
    .map(|(k, v)| (k, Regex::new(v).unwrap()))
    .collect();

    let mut passport = HashMap::new();
    let mut count = 0usize;
    for line in lines {
        if line == "" {
            count += is_valid(&passport, &reg_dct) as usize;
            passport.clear();
        } else {
            for pair in line.split(' ') {
                let kv: Vec<&str> = pair.split(':').collect();
                if fields.contains(&kv[0]) {
                    passport.insert(kv[0].to_owned(), kv[1].to_owned());
                }
            }
        }
    }
    count + is_valid(&passport, &reg_dct) as usize
}

fn is_valid(data: &HashMap<String, String>, reg_dct: &HashMap<&'static str, Regex>) -> bool {
    if data.len() < 7 {
        false
    } else {
        for (k, v) in data {
            if !reg_dct.get(&k as &str).unwrap().is_match(v) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let reg_dct: HashMap<&'static str, Regex> = [
        ("byr", r"^(19[2-9]\d|200[0-2])$"),
        ("iyr", r"^(201\d|2020)$"),
        ("eyr", r"^(202\d|2030)$"),
        ("hgt", r"^((1[5-8]\d|19[0-3])cm|(59|6\d|7[0-6])in)$"),
        ("hcl", r"^#[0-9a-f]{6}$"),
        ("ecl", r"^(amb|blu|brn|gr[yn]|hzl|oth)$"),
        ("pid", r"^\d{9}$"), //("pid", r"^0\d{8}$"),
    ]
    .into_iter()
    .map(|(k, v)| (k, Regex::new(v).unwrap()))
    .collect();

    let f = |k, v| reg_dct.get(k).unwrap().is_match(v);
    assert!(f("byr", "2002"));
    assert!(!f("byr", "2003"));

    assert!(f("hgt", "60in"));
    assert!(f("hgt", "190cm"));
    assert!(!f("hgt", "190in"));
    assert!(!f("hgt", "190"));

    assert!(f("hcl", "#123abc"));
    assert!(!f("hcl", "#123abz"));
    assert!(!f("hcl", "123abc"));

    assert!(f("ecl", "brn"));
    assert!(!f("ecl", "wat"));

    assert!(f("pid", "000000001"));
    assert!(!f("pid", "0123456789"));
}
