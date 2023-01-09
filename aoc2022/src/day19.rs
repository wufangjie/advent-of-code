use crate::read_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

const FILENAME: &str = "./data/day19.txt";
const N: usize = 4;

// NOTE: 造机器是要花一分钟的, 花的是机器人工厂 (单线程) 的时间, 机器人生产矿石的自动的

#[derive(Debug)]
struct BluePrint {
    id: usize,
    costs: [[usize; N - 1]; N],
}

impl BluePrint {
    fn new(lst: Vec<usize>) -> Self {
        Self {
            id: lst[0],
            costs: [
                [lst[1], 0, 0],
                [lst[2], 0, 0],
                [lst[3], lst[4], 0],
                [lst[5], 0, lst[6]],
            ],
        }
    }

    fn run(&mut self, minute_left: usize, having: [usize; N], robots: [usize; N]) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((minute_left, having, robots));
        let mut res = 0;
        while let Some((minute_left, having, robots)) = queue.pop_front() {
            // which robot to make
            for i in 0..N {
                let mut wait = 0;
                for j in 0..N - 1 {
                    if self.costs[i][j] > having[j] {
                        if robots[j] > 0 {
                            wait = wait.max((self.costs[i][j] - having[j] - 1) / robots[j] + 1)
                        } else {
                            wait = usize::MAX - 1; // NOTE: -1, because we will use wait + 1 later
                            break;
                        }
                    }
                }
                if wait + 1 < minute_left {
                    let mut having2 = having;
                    let mut robots2 = robots;
                    for j in 0..N - 1 {
                        having2[j] = having2[j] + robots[j] * (wait + 1) - self.costs[i][j];
                        // NOTE: overflow
                    }
                    having2[N - 1] += robots[N - 1] * (wait + 1);
                    robots2[i] += 1;

                    let ml = minute_left - wait - 1;
                    if having2[N - 1] + robots2[N - 1] * ml + ml * (ml - 1) / 2 > res {
			// 最开始有想到这点但没用, 感觉二次项有点大了, 没想到提升会这么明显
                        queue.push_back((ml, having2, robots2));
                        res = res.max(having2[N - 1] + robots2[N - 1]);
                    }
                }
            }
        }
        res
    }
}

pub fn part1() -> usize {
    let mut res = 0;
    let re = Regex::new(r"(\d+)").unwrap();
    for line in read_lines(FILENAME) {
        let mut lst = Vec::with_capacity(7);
        for caps in re.captures_iter(&line) {
            lst.push(caps.get(1).unwrap().as_str().parse().unwrap());
        }
        let mut bp = BluePrint::new(lst);
        let score = bp.run(24, [0, 0, 0, 0], [1, 0, 0, 0]);
        res += score * bp.id;
    }
    res
}

pub fn part2() -> usize {
    let mut res = 1;
    let re = Regex::new(r"(\d+)").unwrap();
    for line in read_lines(FILENAME).take(3) {
        let mut lst = Vec::with_capacity(7);
        for caps in re.captures_iter(&line) {
            lst.push(caps.get(1).unwrap().as_str().parse().unwrap());
        }
        let mut bp = BluePrint::new(lst);
        let score = bp.run(32, [0, 0, 0, 0], [1, 0, 0, 0]);
        dbg!(score);
        res *= score;
    }
    res
}

#[test]
fn test_19() {
    assert_eq!(1144, part1()); // release mode 0.21s
    assert_eq!(19980, part2()); // release mode 0.93s
}
