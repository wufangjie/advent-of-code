use crate::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::Hash;

const ORI: u8 = 24;
// At least 12 beacons that both scanners detect within the overlap
// what if there are two rotation meet this?
// greedy: only conside first overlap >= 12
// some tricks (distance) to calculate overlaps, I think is wrong

pub fn part1() -> usize {
    let lines = read_lines("./data/day19.txt");
    let mut iter = lines.into_iter();

    let mut xyz_lst = vec![];
    loop {
        let part = read_part(iter.by_ref());
        if part.len() == 0 {
            break;
        }
        xyz_lst.push(part);
    }

    let mut arranged = vec![0];
    let n = xyz_lst.len();
    let mut left: VecDeque<usize> = (0..n).into_iter().collect();

    'main: while let Some(i) = left.pop_front() {
        for &j in arranged.iter().rev() {
            //conv.keys() {
            let (r2, d2, p2) = find_abs_pos(&xyz_lst[i], &xyz_lst[j]);
            if r2 < ORI {
                xyz_lst[i] = p2;
                arranged.push(i);
                dbg!(left.len());
                continue 'main;
            }
        }
        left.push_back(i);
    }

    xyz_lst
        .into_iter()
        .flat_map(|lst| lst.into_iter().collect::<HashSet<Dim3>>())
        .collect::<HashSet<Dim3>>()
        .len()
}

// fn rotate_until(
//     mut pos: Vec<Dim3>,
//     mut i: usize,
//     conv: &HashMap<usize, (usize, u8, Dim3)>,
// ) -> Vec<Dim3> {
//     while i != 0 {
//         let (pre, which, delta) = conv.get(&i).unwrap();
//         pos = pos
//             .into_iter()
//             .map(|triple| triple.rotate(*which).add(&delta))
//             .collect();
//         i = *pre;
//     }
//     pos
// }

fn read_part(iter: &mut impl Iterator<Item = String>) -> Vec<Dim3> {
    iter.skip(1)
        .take_while(|x| x != "")
        .map(|x| Dim3::from_string(x))
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Dim3 {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Debug for Dim3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dim3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Dim3 {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
    fn from_list(lst: Vec<i32>) -> Self {
        Self {
            x: lst[0],
            y: lst[1],
            z: lst[2],
        }
    }

    fn from_string(s: String) -> Self {
        Self::from_list(s.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
    }

    fn from_triple(triple: (i32, i32, i32)) -> Self {
        Self {
            x: triple.0,
            y: triple.1,
            z: triple.2,
        }
    }

    fn rotate(&self, which: u8) -> Self {
        Self::from_triple(match which {
            0 => (self.x, self.y, self.z),
            1 => (self.x, -self.y, -self.z),
            2 => (self.x, self.z, -self.y),
            3 => (self.x, -self.z, self.y),
            4 => (self.y, -self.x, self.z),
            5 => (self.y, self.x, -self.z),
            6 => (self.y, self.z, self.x),
            7 => (self.y, -self.z, -self.x),
            8 => (-self.x, -self.y, self.z),
            9 => (-self.x, self.y, -self.z),
            10 => (-self.x, self.z, self.y),
            11 => (-self.x, -self.z, -self.y),
            12 => (-self.y, self.x, self.z),
            13 => (-self.y, -self.x, -self.z),
            14 => (-self.y, self.z, -self.x),
            15 => (-self.y, -self.z, self.x),
            16 => (self.z, self.x, self.y),
            17 => (self.z, -self.x, -self.y),
            18 => (self.z, self.y, -self.x),
            19 => (self.z, -self.y, self.x),
            20 => (-self.z, self.x, -self.y),
            21 => (-self.z, -self.x, self.y),
            22 => (-self.z, self.y, self.x),
            23 => (-self.z, -self.y, -self.x),
            _ => unreachable!(),
        })
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

fn find_abs_pos(pos: &Vec<Dim3>, lst0: &Vec<Dim3>) -> (u8, Dim3, Vec<Dim3>) {
    let set0: HashSet<Dim3> = lst0.iter().cloned().collect();
    for which in 0..ORI {
        let pos_r: Vec<Dim3> = pos.iter().map(|triple| triple.rotate(which)).collect();
        if let Some(delta) = find_delta(&pos_r, lst0, &set0) {
            return (
                which,
                delta,
                pos_r.into_iter().map(|triple| triple.add(&delta)).collect(),
            );
        }
    }
    (99, Dim3::new(), vec![])
}

fn find_delta(pos: &Vec<Dim3>, lst0: &Vec<Dim3>, set0: &HashSet<Dim3>) -> Option<Dim3> {
    let mut delta_used = HashSet::new();
    for i in 0..pos.len() {
        for j in 0..lst0.len() {
            let delta = lst0[j].sub(&pos[i]);
            if !delta_used.contains(&delta) {
                delta_used.insert(delta);
                let count = count_overlap(pos, delta, set0);
                if count >= 12 {
                    return Some(delta);
                }
            }
        }
    }
    None
}

fn count_overlap(pos: &Vec<Dim3>, delta: Dim3, set0: &HashSet<Dim3>) -> usize {
    pos.iter()
        .map(|triple| {
            if set0.contains(&(triple.add(&delta))) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn part2() -> i32 {
    let lines = read_lines("./data/day19.txt");
    let mut iter = lines.into_iter();

    let mut xyz_lst = vec![];
    loop {
        let part = read_part(iter.by_ref());
        if part.len() == 0 {
            break;
        }
        xyz_lst.push(part);
    }

    let mut arranged = vec![0];
    arranged.push(0);
    let mut pos_lst: Vec<Dim3> = vec![Dim3::new()];

    let n = xyz_lst.len();
    let mut left: VecDeque<usize> = (0..n).into_iter().collect();

    'main: while let Some(i) = left.pop_front() {
        for &j in arranged.iter().rev() {
            let (r2, d2, p2) = find_abs_pos(&xyz_lst[i], &xyz_lst[j]);
            if r2 < ORI {
                xyz_lst[i] = p2;
                arranged.push(i);
                pos_lst.push(Dim3::new().rotate(r2).add(&d2));
                dbg!(left.len());
                continue 'main;
            }
        }
        left.push_back(i);
    }

    // dbg!(&pos_lst);
    let mut max_dist = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            max_dist = max_dist.max(pos_lst[i].dist(&pos_lst[j]));
        }
    }
    max_dist
}

// fn rotate_until_single(
//     mut pos: Dim3,
//     mut i: usize,
//     conv: &HashMap<usize, (usize, u8, Dim3)>,
// ) -> Dim3 {
//     while i != 0 {
//         let (pre, which, delta) = conv.get(&i).unwrap();
//         pos = pos.rotate(*which).add(&delta);
//         i = *pre;
//     }
//     pos
// }
