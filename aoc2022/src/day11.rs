use crate::read_lines;

struct Monkey {
    lst: Vec<i64>,
    ops: fn(i64) -> i64,
    div: i64,
    to: (usize, usize),
}

impl Monkey {
    fn new(lst: Vec<i64>, ops: fn(i64) -> i64, div: i64, to: (usize, usize)) -> Self {
        Self { lst, ops, div, to }
    }

    fn process(&mut self) -> Vec<(usize, i64)> {
        let mut res = Vec::with_capacity(self.lst.len());
        for &item in &self.lst {
            let new = (self.ops)(item) / 3;
            let to = if new % self.div == 0 {
                self.to.0
            } else {
                self.to.1
            };
            res.push((to, new));
        }
        self.lst.clear();
        res
    }

    fn process2(&mut self, lcm: i64) -> Vec<(usize, i64)> {
        let mut res = Vec::with_capacity(self.lst.len());
        for &item in &self.lst {
            let new = (self.ops)(item) % lcm;
            let to = if new % self.div == 0 {
                self.to.0
            } else {
                self.to.1
            };
            res.push((to, new));
        }
        self.lst.clear();
        res
    }
}

// fn make_monkeys() -> Vec<Monkey> {
//     vec![
//         Monkey::new(vec![79, 98], |x| x * 19, 23, (2, 3)),
//         Monkey::new(vec![54, 65, 75, 74], |x| x + 6, 19, (2, 0)),
//         Monkey::new(vec![79, 60, 97], |x| x * x, 13, (1, 3)),
//         Monkey::new(vec![74], |x| x + 3, 17, (0, 1)),
//     ]
// }

fn make_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(vec![85, 77, 77], |x| x * 7, 19, (6, 7)),
        Monkey::new(vec![80, 99], |x| x * 11, 3, (3, 5)),
        Monkey::new(vec![74, 60, 74, 63, 86, 92, 80], |x| x + 8, 13, (0, 6)),
        Monkey::new(vec![71, 58, 93, 65, 80, 68, 54, 71], |x| x + 7, 7, (2, 4)),
        Monkey::new(vec![97, 56, 79, 65, 58], |x| x + 5, 5, (2, 0)),
        Monkey::new(vec![77], |x| x + 4, 11, (4, 3)),
        Monkey::new(vec![99, 90, 84, 50], |x| x * x, 17, (7, 1)),
        Monkey::new(vec![50, 66, 61, 92, 64, 78], |x| x + 3, 2, (5, 1)),
    ]
}

pub fn part1() -> usize {
    let mut monkeys = make_monkeys();
    let n = monkeys.len();
    let mut count = vec![0; n];
    for _ in 0..20 {
        for i in 0..n {
            count[i] += monkeys[i].lst.len();
            for (j, v) in monkeys[i].process() {
                monkeys[j].lst.push(v);
            }
        }
    }
    count.sort_unstable();
    count[n - 2] * count[n - 1]
}

fn calc_lcm(v1: i64, v2: i64) -> i64 {
    v1 * v2 / calc_gcd(v1, v2)
}

fn calc_gcd(v1: i64, v2: i64) -> i64 {
    if v2 == 0 {
        v1
    } else if v2 == 1 {
        1
    } else {
        calc_gcd(v2, v1 % v2)
    }
}

pub fn part2() -> usize {
    let mut monkeys = make_monkeys();
    let mut lcm = 1;
    for m in &monkeys {
        lcm = calc_lcm(lcm, m.div);
    }

    let n = monkeys.len();
    let mut count = vec![0; n];
    for _ in 0..10000 {
        for i in 0..n {
            count[i] += monkeys[i].lst.len();
            for (j, v) in monkeys[i].process2(lcm) {
                monkeys[j].lst.push(v);
            }
        }
    }
    count.sort_unstable();
    count[n - 2] * count[n - 1]
}

#[test]
fn test_11() {
    assert_eq!(54752, part1());
    assert_eq!(13606755504, part2());
}
