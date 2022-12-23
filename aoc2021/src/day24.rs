// use crate::read_lines;
// use regex::Regex;
// use std::collections::HashSet;

// pub fn part1() {
//     let lines = read_lines("./data/day24.txt");
//     let re = Regex::new(r"^([a-z]{3}) ([wxyz]) ([-0-9wxyz]+)$").unwrap();

//     let mut ops_lst = vec![];

//     let mut iter = lines.into_iter();
//     iter.next();
//     loop {
//         let seq: Vec<String> = iter.by_ref().take_while(|line| line != "inp w").collect();
//         if seq.len() == 0 {
//             break;
//         } else {
//             ops_lst.push(seq);
//         }
//     }

//     dbg!(&ops_lst);

//     const N: usize = 14;

//     let mut machines = Vec::with_capacity(N);
//     for i in 0..N {
//         machines.push(Machine::new(&ops_lst[i]));
//     }

//     let mut res = [9; N];
//     let mut seen = Vec::with_capacity(N);
//     for _ in 0..N {
//         seen.push(HashSet::new());
//     }
//     let mut cur = 0;
//     let mut count = 0;
//     loop {
//         break;
//         count += 1;
//         // if count < 20 {
//         //     dbg!((cur, res[cur], machines[cur].wxyz));
//         // } else {
//         //     break;
//         // }
//         if res[cur] == 0 {
//             seen[cur].insert(machines[cur].get_key());
//             res[cur] = 9;
//             cur -= 1;
//             res[cur] -= 1;
//         } else {
//             let key = machines[cur].get_key();
//             if let Some(_) = seen[cur].get(&key) {
//                 res[cur] = 9;
//                 cur -= 1;
//                 res[cur] -= 1;
//             } else {
//                 match machines[cur].calc(res[cur], &re) {
//                     None => res[cur] -= 1,
//                     Some(wxyz) => {
//                         if cur == N - 1 {
//                             if wxyz[3] == 0 {
//                                 // dbg!(&res);
//                                 break;
//                             } else {
//                                 res[cur] -= 1;
//                             }
//                         } else {
//                             cur += 1;
//                             machines[cur].reset_wxyz(wxyz);
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// #[derive(Debug)]
// struct Machine<'a> {
//     seq: &'a Vec<String>,
//     wxyz: [i64; 4],
// }

// impl<'a> Machine<'a> {
//     fn new(seq: &'a Vec<String>) -> Self {
//         Self { seq, wxyz: [0; 4] }
//     }

//     fn reset_wxyz(&mut self, wxyz: [i64; 4]) {
//         self.wxyz = wxyz;
//     }

//     fn get_index(s: &str) -> usize {
//         match s {
//             "w" => 0,
//             "x" => 1,
//             "y" => 2,
//             "z" => 3,
//             _ => unreachable!(),
//         }
//     }

//     fn get_rhs(&self, s: &str) -> i64 {
//         match s {
//             "w" | "x" | "y" | "z" => self.wxyz[Self::get_index(s)],
//             _ => s.parse::<i64>().unwrap(),
//         }
//     }

//     fn calc(&mut self, inp: i64, re: &Regex) -> Option<[i64; 4]> {
//         self.wxyz[0] = inp;
//         for i in 1..self.seq.len() {
//             let caps = re.captures(&self.seq[i]).unwrap();
//             let op = caps.get(1).unwrap().as_str();
//             let i = Self::get_index(caps.get(2).unwrap().as_str());
//             let rhs = self.get_rhs(caps.get(3).unwrap().as_str());
//             match op {
//                 "add" => self.wxyz[i] += rhs,
//                 "mul" => {
//                     // if self.wxyz[i].abs() > 100 && rhs.abs() > 100 {
//                     //     dbg!((self.wxyz[i], rhs));
//                     //     panic!("");
//                     // }
//                     self.wxyz[i] *= rhs;
//                 }
//                 "div" => {
//                     if rhs == 0 {
//                         return None;
//                     }
//                     self.wxyz[i] /= rhs;
//                 }
//                 "mod" => {
//                     if self.wxyz[i] < 0 || rhs <= 0 {
//                         return None;
//                     }
//                     self.wxyz[i] %= rhs;
//                 }
//                 "eql" => self.wxyz[i] = (self.wxyz[i] == rhs) as i64,
//                 _ => unreachable!(),
//             }
//         }
//         Some(self.wxyz)
//     }

//     fn get_key(&self) -> (i64, i64, i64) {
//         (self.wxyz[1], self.wxyz[2], self.wxyz[3])
//     }
// }

pub fn part1() -> i64 {
    let mut w = [0; 14];
    for (j, i, d) in [
        (13, 0, 7), // w[13] == w[0] + 7
        (12, 1, -8),
        (11, 2, -4),
        (6, 3, 1),
        (5, 4, -7),
        (8, 7, 2),
        (10, 9, 5),
    ] {
        for k in (1..=9i64).into_iter().rev() {
            if k + d >= 1 && k + d <= 9 {
                w[i] = k;
                w[j] = k + d;
                break;
            }
        }
    }
    (w.into_iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(""))
    .parse()
    .unwrap()
}

pub fn part2() -> i64 {
    let mut w = [0; 14];
    for (j, i, d) in [
        (13, 0, 7), // w[13] == w[0] + 7
        (12, 1, -8),
        (11, 2, -4),
        (6, 3, 1),
        (5, 4, -7),
        (8, 7, 2),
        (10, 9, 5),
    ] {
        for k in 1..=9i64 {
            if k + d >= 1 && k + d <= 9 {
                w[i] = k;
                w[j] = k + d;
                break;
            }
        }
    }
    (w.into_iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(""))
    .parse()
    .unwrap()
}

// only div_z[], add_x[], add_y[] are different // see day24.py
// when add_x[i] > 0, there's no possible x == 0
// we have 26 ** 7 * (w0 + 14) (w0 > 1)
// so the other 7 rounds must have x == 0

// (w0 + 14) * 26 ** 4
// + (w1 + 2) * 26 ** 3
// + (w2 + 1) * 26 ** 2
// + (w3 + 13) * 26
// + w4 + 5 == w5 + 12 #

// (w0 + 14) * 26 ** 3
// + (w1 + 2) * 26 ** 2
// + (w2 + 1) * 26
// + (w3 + 13) == w6 + 12 #

// (w0 + 14) * 26 ** 3
// + (w1 + 2) * 26 ** 2
// + (w2 + 1) * 26
// + w7 + 9 == w8 + 7 #

// (w0 + 14) * 26 ** 3
// + (w1 + 2) * 26 ** 2
// + (w2 + 1) * 26
// + (w9 + 13) == w10 + 8 #

// (w0 + 14) * 26 ** 2
// + (w1 + 2) * 26
// + (w2 + 1) == w11 + 5 #

// (w0 + 14) * 26
// + (w1 + 2) == w12 + 10 #

// (w0 + 14) == w13 + 7 #

#[test]
fn test_24() {
    assert_eq!(29989297949519, part1());
    assert_eq!(19518121316118, part2());
}
