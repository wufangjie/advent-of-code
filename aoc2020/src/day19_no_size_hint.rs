use crate::read_lines;
use itertools::Itertools; // (1..ns).combinations(nc - 1);
use regex::Regex;
use std::collections::{HashMap, HashSet};

// part1: use generating way (slow too, not support loop)
// part2: is a generic but slow implemention, support circle (no just self loop)

pub fn part1() -> usize {
    let (mut rules, tests) = get_rules_and_tests();
    let re_chr: Regex = Regex::new("^[a-z]$").unwrap();

    let mut messages = vec![HashSet::new(); rules.len()];
    let mut stack = vec![0];
    while !stack.is_empty() {
        let i = *stack.last().unwrap();
        if rules[i].is_empty() {
            stack.pop();
            continue;
        }
        let mut to_push = vec![];
        let mut to_concat = vec![];
        for s in &rules[i] {
            if re_chr.is_match(s) {
                messages[i].insert(s.clone());
                continue;
            }
            to_concat.push(vec![]);
            for j in s.split(' ').map(|d| d.parse::<usize>().unwrap()) {
                if rules[j].is_empty() {
                    to_concat.last_mut().unwrap().push(j);
                } else {
                    to_push.push(j);
                }
            }
        }
        if to_push.is_empty() {
            for lst in to_concat {
                if lst.len() > 0 {
                    update_messages(&mut messages, lst, i);
                }
            }
            rules[i].clear();
        } else {
            stack.extend(to_push);
        }
    }
    tests.iter().map(|s| messages[0].contains(s) as usize).sum()
}

fn update_messages(messages: &mut Vec<HashSet<String>>, lst: Vec<usize>, i0: usize) {
    let mut vec1 = vec![String::new()];
    let mut vec2 = Vec::with_capacity(messages[lst[0]].len());
    let n = lst.len();
    for i in 0..lst.len() {
        for s in vec1 {
            for s2 in &messages[lst[i]] {
                vec2.push(s.clone() + s2);
            }
        }
        vec1 = vec2;
        if i < n - 1 {
            vec2 = Vec::with_capacity(vec1.len() * messages[lst[i + 1]].len());
        } else {
            break;
        }
    }
    messages[i0].extend(vec1);
}

pub fn part2() -> usize {
    let (mut rules, tests) = get_rules_and_tests();
    rules[8] = vec!["42".to_owned(), "42 8".to_owned()];
    rules[11] = vec!["42 31".to_owned(), "42 11 31".to_owned()];
    let re_chr: Regex = Regex::new("^[a-z]$").unwrap();
    let mut ck = Checker::new(rules, re_chr);
    tests.iter().map(|x| ck.is_valid(x, 0) as usize).sum()
}

struct Checker<'a> {
    rules: Vec<Vec<String>>,
    cache: HashMap<(&'a str, usize), bool>,
    re_chr: Regex,
}

impl<'a> Checker<'a> {
    fn new(rules: Vec<Vec<String>>, re_chr: Regex) -> Self {
        Checker {
            rules,
            cache: HashMap::new(),
            re_chr,
        }
    }

    fn is_valid(&mut self, s: &'a str, i: usize) -> bool {
        if let Some(res) = self.cache.get(&(s, i)) {
            return *res;
        } else {
            // I tried to calc least chars for i, but did not work
            let res = self._is_valid(s, i);
            self.cache.insert((s, i), res);
            res
        }
    }

    fn _is_valid(&mut self, s: &'a str, i: usize) -> bool {
        let ns = s.len();
        let nseq = self.rules[i].len();
        for iseq in 0..nseq {
            if self.re_chr.is_match(&self.rules[i][iseq]) {
                if s == self.rules[i][iseq] {
                    return true;
                }
                continue;
            }

            let children: Vec<usize> = self.rules[i][iseq]
                .split(' ')
                .map(|d| d.parse().unwrap())
                .collect();
            let nc = children.len();
            let comb_iter = (1..ns).combinations(nc - 1);
            'comb: for comb in comb_iter {
                let mut lo = 0;
                for j in 0..nc - 1 {
                    if !self.is_valid(&s[lo..comb[j]], children[j]) {
                        continue 'comb;
                    }
                    lo = comb[j];
                }
                if self.is_valid(&s[lo..ns], children[nc - 1]) {
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
        if line == "" {
            break;
        }
        rules_rows.push(line);
    }

    let mut rules = vec![vec![]; rules_rows.len()];
    let re_sep: Regex = Regex::new(r"[:|]").unwrap();
    for s in rules_rows {
        let mut iter = re_sep.split(&s).into_iter();
        let i: usize = iter.next().unwrap().parse().unwrap();
        for ds in iter {
            rules[i].push(ds.trim().replace("\"", ""));
        }
    }
    (rules, lines.collect())
}
