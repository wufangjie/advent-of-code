use crate::read_lines;
use std::fmt;

pub fn part1() -> i32 {
    let lines = read_lines("./data/day18.txt");
    let mut num = SFNumber::from_str(&lines[0]);
    for i in 1..lines.len() {
        num.add_by(SFNumber::from_str(&lines[i]));
        num.reduce();
        //println!("{}", num);
    }
    //println!("{}", num);
    num.calc_magnitude()
}

#[derive(Debug, Clone)]
enum Basic {
    Start,
    End,
    Regular(u8),
}

impl Basic {
    fn add_by(&mut self, num: u8) {
        if let Self::Regular(x) = self {
            *x += num;
        }
    }

    fn get(&self) -> u8 {
        if let Self::Regular(x) = self {
            *x
        } else {
            0
        }
    }
}

#[derive(Clone)]
struct SFNumber {
    data: Vec<Basic>,
}

impl SFNumber {
    fn from_str(s: &str) -> Self {
        let mut data = vec![];
        for c in s.bytes() {
            match c {
                b'[' => data.push(Basic::Start),
                b']' => data.push(Basic::End),
                b'0'..=b'9' => data.push(Basic::Regular(c - b'0')), // no split!
                _ => (),
            }
        }
        Self { data }
    }

    fn add_by(&mut self, other: Self) {
        self.data.push(Basic::Start);
        self.data.rotate_right(1);
        self.data.extend(other.data);
        self.data.push(Basic::End);
    }

    fn add(&self, other: &Self) -> Self {
        let mut new = vec![];
        new.push(Basic::Start);
        new.extend(self.data.clone());
        new.extend(other.data.clone());
        new.push(Basic::End);
        Self { data: new }
    }

    fn explode(&mut self) -> bool {
        // [m, n] -> x (4 -> 1)
        let mut count_nest = 0;
        let mut pre_num_i = 0;
        for i in 0..self.data.len() {
            match self.data[i] {
                Basic::Start => {
                    count_nest += 1;
                    if count_nest == 5 {
                        if pre_num_i != 0 {
                            let to_add = self.data[i + 1].get();
                            self.data[pre_num_i].add_by(to_add);
                        }
                        let nxt_num_i = self.next_regular(i + 4);
                        if nxt_num_i != 0 {
                            let to_add = self.data[i + 2].get();
                            self.data[nxt_num_i].add_by(to_add);
                        }
                        self.data[i] = Basic::Regular(0);
                        drop(self.data.drain(i + 1..i + 4));
                        return true;
                    }
                }
                Basic::End => count_nest -= 1,
                _ => pre_num_i = i,
            }
        }
        false
    }

    fn next_regular(&self, from: usize) -> usize {
        for i in from..self.data.len() {
            if let Basic::Regular(_) = self.data[i] {
                return i;
            }
        }
        0
    }

    fn find_splits(&self) -> Vec<(usize, usize)> {
        // (index, nest)
        let mut count_nest = 0;
        let mut res = vec![];
        for i in 0..self.data.len() {
            match self.data[i] {
                Basic::Start => count_nest += 1,
                Basic::End => count_nest -= 1,
                Basic::Regular(v) => {
                    if v > 9 {
                        res.push((i, count_nest));
                    }
                }
            }
        }
        res
    }

    fn split_at(&mut self, i: usize) {
        let n = self.data[i].get();
        self.data[i] = Basic::Start;
        self.data.extend(vec![
            Basic::Regular(n >> 1),
            Basic::Regular(n - (n >> 1)),
            Basic::End,
        ]);
        let (_, r) = self.data.split_at_mut(i + 1);
        r.rotate_right(3);
    }

    fn reduce(&mut self) {
        while self.explode() {}

        loop {
            let splits = self.find_splits();
            if splits.len() == 0 {
                break;
            } else {
                let mut to_split = splits[0];
                if splits[0].1 < 4 {
                    for i in 1..splits.len() {
                        if splits[i].1 > 4 {
                            to_split = splits[i];
                            break;
                        }
                    }
                }
                self.split_at(to_split.0);
                if to_split.1 > 3 {
                    self.explode();
                }
            }
        }
    }

    fn calc_magnitude(&self) -> i32 {
        let mut stack = vec![];
        for i in 0..self.data.len() {
            match self.data[i] {
                Basic::Start => (),
                Basic::End => {
                    let right = stack.pop().unwrap();
                    let left = stack.last_mut().unwrap();
                    *left *= 3;
                    *left += right << 1;
                }
                Basic::Regular(v) => stack.push(v as i32),
            }
        }
        stack[0]
    }
}

impl fmt::Display for SFNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 1..self.data.len() {
            match self.data[i] {
                Basic::Start => {
                    if let Basic::Start = self.data[i - 1] {
                        write!(f, "[")?
                    } else {
                        write!(f, ", [")?
                    }
                }
                Basic::End => write!(f, "]")?,
                Basic::Regular(v) => {
                    if let Basic::Start = self.data[i - 1] {
                        write!(f, "{}", v)?;
                    } else {
                        write!(f, ", {}", v)?;
                    }
                }
            }
        }
        write!(f, "")
    }
}

#[test]
fn test_18() {
    let mut n1 = SFNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    n1.reduce();
    assert_eq!(
        "[[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]",
        format!("{}", &n1)
    );

    assert_eq!(
        143,
        SFNumber::from_str("[[1,2],[[3,4],5]]").calc_magnitude()
    );
}

pub fn part2() -> i32 {
    let lines = read_lines("./data/day18.txt");
    let n = lines.len();
    let mut best = 0;
    for i in 0..n - 1 {
        let num = SFNumber::from_str(&lines[i]);
        for j in i + 1..n {
            let num2 = SFNumber::from_str(&lines[j]);
            for mut tmp in [num.add(&num2), num2.add(&num)] {
                tmp.reduce();
                best = best.max(tmp.calc_magnitude());
            }
        }
    }
    best
}
