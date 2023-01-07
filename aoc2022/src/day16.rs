use crate::read_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

const FILENAME: &str = "./data/day16.txt";

struct DpM {
    rate_dct: HashMap<String, i64>,
    next_dct: HashMap<String, Vec<String>>,
    cache: HashMap<(i64, i64, String), i64>,
}

impl DpM {
    fn new(rate_dct: HashMap<String, i64>, next_dct: HashMap<String, Vec<String>>) -> Self {
        Self {
            rate_dct,
            next_dct,
            cache: HashMap::new(),
        }
    }

    fn shortest_path(&self, from: &str, to: &str) -> i64 {
        let mut queue = VecDeque::new();
	queue.push_back((from, 0));
        let mut visited = HashSet::new();
	visited.insert(from);
        while let Some((u, mut d)) = queue.pop_front() {
            if u == to {
                return d;
            }
            d += 1;
            for v in self.next_dct.get(u).unwrap() {
                if !visited.contains(&v[..]) {
		    queue.push_back((v, d));
		    visited.insert(v);
                }
            }
        }
        unreachable!()
    }

    fn solve(&mut self, left_minute: i64, mut selected: i64, cur: &str) -> i64 {
        if left_minute < 2 {
            0
        } else {
            if let Some(res) = self.cache.get(&(left_minute, selected, cur.to_string())) {
                return *res;
            }

            let mut best = 0;
            let seq = self
                .rate_dct
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .enumerate()
                .collect::<Vec<(usize, (String, i64))>>();

            for (i, (name, rate)) in seq {
                // go to valve by name and open it
                let flag = 2_i64.pow(i as u32);
                if flag & selected == 0 {
                    let lm2 = left_minute - 1 - self.shortest_path(cur, &name);
                    if lm2 > 0 {
                        let mut score = self.solve(lm2, selected | flag, &name);
                        self.cache.insert((lm2, selected | flag, name), score);
                        score += lm2 * rate;
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


struct DpM2 {
    rate_dct: HashMap<String, i64>,
    next_dct: HashMap<String, Vec<String>>,
    cache: HashMap<(i64, i64, i64, String, String), i64>,
}

impl DpM2 {
    fn new(rate_dct: HashMap<String, i64>, next_dct: HashMap<String, Vec<String>>) -> Self {
        Self {
            rate_dct,
            next_dct,
            cache: HashMap::new(),
        }
    }

    fn shortest_path(&self, from: &str, to: &str) -> i64 {
        let mut queue = VecDeque::new();
	queue.push_back((from, 0));
        let mut visited = HashSet::new();
	visited.insert(from);
        while let Some((u, mut d)) = queue.pop_front() {
            if u == to {
                return d;
            }
            d += 1;
            for v in self.next_dct.get(u).unwrap() {
                if !visited.contains(&v[..]) {
		    queue.push_back((v, d));
		    visited.insert(v);
                }
            }
        }
        unreachable!()
    }

    fn solve(&mut self, left_minute: i64, left_minute2: i64, mut selected: i64, cur: &str, cur2: &str) -> i64 {
        if left_minute < 2 && left_minute2 < 2 {
            0
        } else {
            if let Some(res) = self.cache.get(&(left_minute, left_minute2, selected, cur.to_string(), cur2.to_string())) {
                return *res;
            }

	    let mut best = 0;
	    let seq = self
                .rate_dct
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .enumerate()
                .collect::<Vec<(usize, (String, i64))>>();


	    if left_minute >= left_minute2 {

		for (i, (name, rate)) in seq {
                    // go to valve by name and open it
                    let flag = 2_i64.pow(i as u32);
                    if flag & selected == 0 {
			let lm2 = left_minute - 1 - self.shortest_path(cur, &name);
			if lm2 > 0 {
                            let mut score = self.solve(lm2, left_minute2, selected | flag, &name, cur2);
                            self.cache.insert((lm2, left_minute2, selected | flag, name, cur2.to_string()), score);
                            score += lm2 * rate;
                            if score > best {
				best = score
                            }
			} // else {
                        //     selected |= flag; // never
			// } // NOTE: we can not do early stop here, because we can not promise the other one can not reach the value
                    }
		}
	    } else {

		for (i, (name, rate)) in seq {
                    // go to valve by name and open it
                    let flag = 2_i64.pow(i as u32);
                    if flag & selected == 0 {
			let lm2 = left_minute2 - 1 - self.shortest_path(cur2, &name);
			if lm2 > 0 {
                            let mut score = self.solve(left_minute, lm2, selected | flag, cur, &name);
                            self.cache.insert((left_minute, lm2, selected | flag, cur.to_string(), name), score);
                            score += lm2 * rate;
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

    dp.solve(30, 0, "AA")
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
    dp.solve(26, 26, 0, "AA", "AA")
}


#[test]
fn test_15() {
    // dbg!(part1());
    // assert_eq!(2253, part1()); // release mode cost 5.42s (use cache 1.42s)
    // assert_eq!(2838, part2()); // finished in 645.82s
}
