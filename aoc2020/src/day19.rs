use crate::read_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn part1() -> usize {
    let (rules, tests) = get_rules_and_tests();
    let size_hints = calc_size_hints(&rules);

    let re_chr: Regex = Regex::new(r"^[a-z]$").unwrap();
    let re_num: Regex = Regex::new(r"^\d+$").unwrap();
    let mut ck = Checker::new(rules, re_chr, re_num, size_hints);
    tests
        .iter()
        .map(|x| ck.is_valid(x, "0".to_owned()) as usize)
        .sum()
}

fn calc_size_hints(rules: &[Vec<String>]) -> Vec<HashSet<usize>> {
    let re_chr: Regex = Regex::new("^[a-z]$").unwrap();
    let n = rules.len();
    let mut size_hints = vec![HashSet::new(); n];
    let mut left_unknown = Vec::with_capacity(n);
    let mut find_parent = vec![HashSet::new(); n];
    let mut known_stack = vec![];
    for i in 0..n {
        let mut unknown = HashSet::new();
        let mut known = true;
        for seq in &rules[i] {
            if re_chr.is_match(seq) {
                size_hints[i].insert(1);
            } else {
                known = false;
                for d in seq.split(' ') {
                    let d: usize = d.parse().unwrap();
                    unknown.insert(d);
                    find_parent[d].insert(i);
                }
            }
        }
        if known {
            known_stack.push(i);
        }
        left_unknown.push(unknown);
    }

    while let Some(i) = known_stack.pop() {
        for seq in &rules[i] {
            if !re_chr.is_match(seq) {
                let mut sz_set = HashSet::new();
                sz_set.insert(0usize);
                for d in seq.split(' ') {
                    let d: usize = d.parse().unwrap();
                    sz_set = sz_set
                        .into_iter()
                        .flat_map(|x| {
                            std::iter::repeat(x)
                                .zip(size_hints[d].iter())
                                .map(|(a, b)| a + b)
                        })
                        .collect();
                }
                size_hints[i].extend(sz_set);
            }
        }
        for &p in &find_parent[i] {
            left_unknown[p].remove(&i);
            if left_unknown[p].is_empty() {
                known_stack.push(p);
            }
        }
    }
    size_hints
}

struct Checker<'a> {
    rules: Vec<Vec<String>>,
    cache: HashMap<(&'a str, String), bool>,
    re_chr: Regex,
    re_num: Regex,
    size_hints: Vec<HashSet<usize>>,
}

impl<'a> Checker<'a> {
    fn new(
        rules: Vec<Vec<String>>,
        re_chr: Regex,
        re_num: Regex,
        size_hints: Vec<HashSet<usize>>,
    ) -> Self {
        Checker {
            rules,
            cache: HashMap::new(),
            re_chr,
            re_num,
            size_hints,
        }
    }

    fn is_valid(&mut self, s: &'a str, r: String) -> bool {
        if self.re_chr.is_match(&r) {
            s == r
        } else if s.is_empty() {
            r.is_empty()
        } else if r.is_empty() {
            false
        } else if let Some(res) = self.cache.get(&(s, r.clone())) {
            *res
        } else {
            let res = self._is_valid(s, r.clone());
            self.cache.insert((s, r), res);
            res
        }
    }

    fn _is_valid(&mut self, s: &'a str, r: String) -> bool {
        if self.re_num.is_match(&r) {
            let i: usize = r.parse().unwrap();
            let nseq = self.rules[i].len();
            for iseq in 0..nseq {
                if self.is_valid(s, self.rules[i][iseq].clone()) {
                    return true;
                }
            }
        } else {
            let mut i = 0;
            for c in r.as_bytes() {
                if c == &b' ' {
                    break;
                }
                i += 1
            }
            let first: usize = (&r[..i]).parse().unwrap();
            let lst: Vec<usize> = self.size_hints[first]
                .iter()
                .cloned()
                .filter(|&x| x < s.len())
                .collect();
            for sz in lst {
                if self.is_valid(&s[..sz], (&r[..i]).to_owned())
                    && self.is_valid(&s[sz..], (&r[i + 1..]).to_owned())
                {
                    return true;
                }
            }
        }
        false
    }
}

fn get_rules_and_tests() -> (Vec<Vec<String>>, Vec<String>) {
    let mut lines = read_lines("./data/day19.txt").into_iter();
    let mut rules_rows = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        rules_rows.push(line);
    }

    let mut rules = vec![vec![]; rules_rows.len()];
    let re_sep: Regex = Regex::new(r"[:|]").unwrap();
    for s in rules_rows {
        let mut iter = re_sep.split(&s);
        let i: usize = iter.next().unwrap().parse().unwrap();
        for ds in iter {
            rules[i].push(ds.trim().replace('"', ""));
        }
    }
    (rules, lines.collect())
}

pub fn part2() -> usize {
    let (mut rules, tests) = get_rules_and_tests();
    let mut size_hints = calc_size_hints(&rules);

    for i in 2..12 {
        size_hints[8].insert(i * 8);
        size_hints[11].insert(i * 8);
    }
    rules[8] = vec!["42".to_owned(), "42 8".to_owned()];
    rules[11] = vec!["42 31".to_owned(), "42 11 31".to_owned()];

    let re_chr: Regex = Regex::new(r"^[a-z]$").unwrap();
    let re_num: Regex = Regex::new(r"^\d+$").unwrap();
    let mut ck = Checker::new(rules, re_chr, re_num, size_hints);
    tests
        .iter()
        .map(|x| ck.is_valid(x, "0".to_owned()) as usize)
        .sum()
}

#[test]
fn test_19() {
    assert_eq!(299, part1());
    assert_eq!(414, part2());
}
