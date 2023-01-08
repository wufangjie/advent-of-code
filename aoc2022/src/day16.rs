use crate::read_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

const FILENAME: &str = "./data/day16.txt";

fn calc_shortest_path(
    from: &str,
    valve_lst: &HashMap<String, usize>,
    next_dct: &HashMap<String, Vec<String>>,
) -> Vec<i64> {
    let mut res = vec![0; valve_lst.len()];

    let mut queue = VecDeque::new();
    queue.push_back((from, 0));
    let mut visited = HashSet::new();
    visited.insert(from);
    while let Some((u, mut d)) = queue.pop_front() {
        if let Some(i) = valve_lst.get(u) {
            res[*i] = d;
        }
        d += 1;
        for v in next_dct.get(u).unwrap() {
            if !visited.contains(&v[..]) {
                queue.push_back((v, d));
                visited.insert(v);
            }
        }
    }
    res
}

#[derive(Debug)]
struct DpM {
    rate_lst: Vec<i64>,
    sp_lst: Vec<Vec<i64>>,
    cache: HashMap<(i64, i64, usize), i64>,
}

impl DpM {
    fn new(rate_dct: HashMap<String, i64>, next_dct: HashMap<String, Vec<String>>) -> Self {
        let n = rate_dct.len();
        let mut temp: Vec<(String, i64)> = rate_dct.into_iter().collect();
        temp.sort_unstable();
        let mut valve_lst = Vec::with_capacity(n + 1);
        let mut rate_lst = Vec::with_capacity(n + 1);
        valve_lst.push("AA".to_string());
        rate_lst.push(0);
        for (valve, rate) in temp {
            valve_lst.push(valve);
            rate_lst.push(rate);
        }
        let index_dct: HashMap<String, usize> = valve_lst
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect();

        let mut sp_lst = Vec::with_capacity(n);
        for valve in valve_lst.iter() {
            sp_lst.push(calc_shortest_path(valve, &index_dct, &next_dct));
        }

        Self {
            rate_lst,
            sp_lst,
            cache: HashMap::new(),
        }
    }

    fn solve(&mut self, left_minute: i64, mut selected: i64, cur: usize) -> i64 {
        if left_minute < 2 {
            0
        } else {
            if let Some(res) = self.cache.get(&(left_minute, selected, cur)) {
                return *res;
            }

            let mut best = 0;
            let n = self.rate_lst.len();
            for i in 0..n {
                let flag = 2_i64.pow(i as u32);
                if flag & selected == 0 {
                    let lm2 = left_minute - 1 - self.sp_lst[cur][i];
                    if lm2 > 0 {
                        let mut score = self.solve(lm2, selected | flag, i);
                        self.cache.insert((lm2, selected | flag, i), score);
                        score += lm2 * self.rate_lst[i];
                        if score > best {
                            best = score
                        }
                    } else {
                        selected |= flag; // never
                    }
                }
            }
            best
        }
    }
}

pub fn part1() -> i64 {
    let mut rate_dct = HashMap::new();
    let mut next_dct = HashMap::new();

    // NOTE: 最后的除了 valves 还可能有 valve (单个的情况)
    let re =
        Regex::new(r"Valve (?P<name>[A-Z]{2}) .*? rate=(?P<rate>\d+); .*? (?P<neighbor>[A-Z, ]+)$")
            .unwrap();
    for line in read_lines(FILENAME) {
        let caps = re.captures(&line).unwrap();
        let name = caps["name"].to_string();
        let rate = caps["rate"].parse::<i64>().unwrap();
        if rate != 0 {
            rate_dct.insert(name.clone(), rate);
        }
        next_dct.insert(
            name,
            caps["neighbor"]
                .split(", ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }

    // dbg!(&rate_dct); // 15! cases
    let mut dp = DpM::new(rate_dct, next_dct);
    dp.solve(30, 1, 0) // "AA"
}

struct DpM2 {
    rate_lst: Vec<i64>,
    sp_lst: Vec<Vec<i64>>,
    cache: HashMap<(i64, i64, i64, usize, usize), i64>,
}

impl DpM2 {
    fn new(rate_dct: HashMap<String, i64>, next_dct: HashMap<String, Vec<String>>) -> Self {
        let n = rate_dct.len();
        let mut temp: Vec<(String, i64)> = rate_dct.into_iter().collect();
        temp.sort_unstable();
        let mut valve_lst = Vec::with_capacity(n + 1);
        let mut rate_lst = Vec::with_capacity(n + 1);
        valve_lst.push("AA".to_string());
        rate_lst.push(0);
        for (valve, rate) in temp {
            valve_lst.push(valve);
            rate_lst.push(rate);
        }
        let index_dct: HashMap<String, usize> = valve_lst
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect();

        let mut sp_lst = Vec::with_capacity(n);
        for valve in valve_lst.iter() {
            sp_lst.push(calc_shortest_path(valve, &index_dct, &next_dct));
        }

        Self {
            rate_lst,
            sp_lst,
            cache: HashMap::new(),
        }
    }

    fn solve(
        &mut self,
        left_minute: i64,
        left_minute2: i64,
        selected: i64,
        cur: usize,
        cur2: usize,
    ) -> i64 {
        if left_minute < 2 && left_minute2 < 2 {
            0
        } else {
            if let Some(res) = self
                .cache
                .get(&(left_minute, left_minute2, selected, cur, cur2))
            {
                return *res;
            }

            let mut best = 0;
            let n = self.rate_lst.len();

            if left_minute >= left_minute2 {
                for i in 0..n {
                    let flag = 2_i64.pow(i as u32);
                    if flag & selected == 0 {
                        let lm2 = left_minute - 1 - self.sp_lst[cur][i];
                        if lm2 > 0 {
                            let mut score = self.solve(lm2, left_minute2, selected | flag, i, cur2);
                            self.cache
                                .insert((lm2, left_minute2, selected | flag, i, cur2), score);
                            score += lm2 * self.rate_lst[i];
                            if score > best {
                                best = score
                            }
                        } // else {
                          //     selected |= flag; // never
                          // } // NOTE: we can not do early stop here, because we can not promise the other one can not reach the value
                    }
                }
            } else {
                for i in 0..n {
                    let flag = 2_i64.pow(i as u32);
                    if flag & selected == 0 {
                        let lm2 = left_minute2 - 1 - self.sp_lst[cur2][i];
                        if lm2 > 0 {
                            let mut score = self.solve(left_minute, lm2, selected | flag, cur, i);
                            self.cache
                                .insert((left_minute, lm2, selected | flag, cur, i), score);
                            score += lm2 * self.rate_lst[i];
                            if score > best {
                                best = score
                            }
                        } // else {
                          //     selected |= flag; // never
                          // }
                    }
                }
            }
            best
        }
    }
}

pub fn part2() -> i64 {
    let mut rate_dct = HashMap::new();
    let mut next_dct = HashMap::new();

    // NOTE: 最后的除了 valves 还可能有 valve (单个的情况)
    let re =
        Regex::new(r"Valve (?P<name>[A-Z]{2}) .*? rate=(?P<rate>\d+); .*? (?P<neighbor>[A-Z, ]+)$")
            .unwrap();
    for line in read_lines(FILENAME) {
        let caps = re.captures(&line).unwrap();
        let name = caps["name"].to_string();
        let rate = caps["rate"].parse::<i64>().unwrap();
        if rate != 0 {
            rate_dct.insert(name.clone(), rate);
        }
        next_dct.insert(
            name,
            caps["neighbor"]
                .split(", ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }

    // dbg!(&rate_dct); // 15! cases
    let mut dp = DpM2::new(rate_dct, next_dct);
    dp.solve(26, 26, 1, 0, 0)
}

#[test]
fn test_15() {
    assert_eq!(2253, part1());
    // assert_eq!(2838, part2()); // NOTE: use release mode cost 8.94s
}
