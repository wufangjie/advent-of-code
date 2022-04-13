use crate::read_lines;
use std::collections::HashMap;

pub fn part1() -> usize {
    let mut adapters: Vec<usize> = read_lines("./data/day10.txt")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();
    adapters.sort_unstable();
    let mut pre = 0;
    let mut diff = [0, 0, 0, 0];
    for v in adapters {
        diff[v - pre] += 1;
        pre = v;
    }
    diff[1] * (diff[3] + 1)
}

pub fn part2() -> usize {
    let mut adapters: Vec<usize> = read_lines("./data/day10.txt")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();
    adapters.sort_unstable();
    let mut counter = Counter::new(adapters);
    counter.count(0, 0)
}

pub fn part2_dp() -> usize {
    let mut adapters: Vec<usize> = read_lines("./data/day10.txt")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();
    adapters.sort_unstable();

    let n = adapters.len();
    let mut dp = vec![0; n];
    for i in 0..n {
        if adapters[i] <= 3 {
            dp[i] += 1;
        }
        let mut j = i;
        loop {
            if j == 0 {
                break;
            }
            j -= 1;
            if adapters[i] <= adapters[j] + 3 {
                dp[i] += dp[j];
            } else {
                break;
            }
        }
    }
    dp[n - 1]
}

struct Counter {
    adapters: Vec<usize>,
    cache: HashMap<(usize, usize), usize>,
    n: usize,
}

impl Counter {
    fn new(adapters: Vec<usize>) -> Self {
        let n = adapters.len() - 1;
        Counter {
            adapters,
            cache: HashMap::new(),
            n,
        }
    }

    fn count(&mut self, pre: usize, i: usize) -> usize {
        if let Some(res) = self.cache.get(&(pre, i)) {
            *res
        } else {
            let res = self._count(pre, i);
            self.cache.insert((pre, i), res);
            res
        }
    }

    fn _count(&mut self, pre: usize, i: usize) -> usize {
        if self.adapters[i] - pre > 3 {
            0
        } else if i == self.n {
            1
        } else {
            self.count(self.adapters[i], i + 1) + self.count(pre, i + 1)
        }
    }
}

#[test]
fn test_10() {
    assert_eq!(2070, part1());
    assert_eq!(24179327893504, part2());
    assert_eq!(24179327893504, part2_dp());
}
