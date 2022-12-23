use crate::read_lines;
use regex::Regex;
//use std::cmp::Ordering::{Equal, Greater, Less};
//use std::collections::HashSet;

type T = i64;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cube {
    on: bool,
    x1: T,
    x2: T,
    y1: T,
    y2: T,
    z1: T,
    z2: T,
}

impl Cube {
    pub fn new(on: bool, x1: T, x2: T, y1: T, y2: T, z1: T, z2: T) -> Self {
        Self {
            on,
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        }
    }

    pub fn from_sp(vec: Vec<T>, on: bool) -> Self {
        Self {
            on,
            x1: vec[0],
            x2: vec[1] + 1,
            y1: vec[2],
            y2: vec[3] + 1,
            z1: vec[4],
            z2: vec[5] + 1,
        }
    }

    pub fn count(&self) -> T {
        (self.x2 - self.x1) * (self.y2 - self.y1) * (self.z2 - self.z1)
    }

    // self 是后来的, 算交集需要的话拆分, 返回值是(明暗变化, 拆分后的)
    pub fn intersect(&self, other: &Self) -> (T, Vec<Cube>) {
        let xmin = self.x1.max(other.x1);
        let xmax = self.x2.min(other.x2);
        if xmin >= xmax {
            return (0, vec![*self]);
        }
        let ymin = self.y1.max(other.y1);
        let ymax = self.y2.min(other.y2);
        if ymin >= ymax {
            return (0, vec![*self]);
        }
        let zmin = self.z1.max(other.z1);
        let zmax = self.z2.min(other.z2);
        if zmin >= zmax {
            return (0, vec![*self]);
        }

        let mut left_lst = vec![];
        if self.x1 < xmin {
            left_lst.push(Self::new(
                self.on, self.x1, xmin, self.y1, self.y2, self.z1, self.z2,
            ));
        }
        if self.x2 > xmax {
            left_lst.push(Self::new(
                self.on, xmax, self.x2, self.y1, self.y2, self.z1, self.z2,
            ));
        }
        if self.y1 < ymin {
            left_lst.push(Self::new(
                self.on, xmin, xmax, self.y1, ymin, self.z1, self.z2,
            ));
        }
        if self.y2 > ymax {
            left_lst.push(Self::new(
                self.on, xmin, xmax, ymax, self.y2, self.z1, self.z2,
            ));
        }
        if self.z1 < zmin {
            left_lst.push(Self::new(self.on, xmin, xmax, ymin, ymax, self.z1, zmin));
        }
        if self.z2 > zmax {
            left_lst.push(Self::new(self.on, xmin, xmax, ymin, ymax, zmax, self.z2));
        }

        if self.on == other.on {
            (0, left_lst)
        } else if self.on {
            ((xmax - xmin) * (ymax - ymin) * (zmax - zmin), left_lst)
        } else {
            (-(xmax - xmin) * (ymax - ymin) * (zmax - zmin), left_lst)
        }
    }
}

pub fn part1() -> usize {
    let lines = read_lines("./data/day22.txt");
    let re = Regex::new(
        r"^(on|off) x=([-0-9]+)\.\.([-0-9]+),y=([-0-9]+)\.\.([-0-9]+),z=([-0-9]+)\.\.([-0-9]+)",
    )
    .unwrap();
    let n = lines.len();
    let mut cube_lst: Vec<Cube> = Vec::with_capacity(n);
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let state = caps.get(1).unwrap().as_str() == "on";
        let mut pos: Vec<T> = (2..8)
            .into_iter()
            .map(|i| caps.get(i).unwrap().as_str().parse::<T>().unwrap())
            .collect();
        let mut need = true;
        for i in [0, 2, 4] {
            pos[i] = pos[i].max(-50);
            pos[i + 1] = pos[i + 1].min(50);
            if pos[i + 1] < pos[i] {
                need = false;
                break;
            }
        }
        if need {
            cube_lst.push(Cube::from_sp(pos, state));
        }
    }

    let mut count = 0;
    for (i, cube) in cube_lst.iter().enumerate() {
        let mut left = vec![*cube];
        let mut j = i;
        while j > 0 {
            j -= 1;
            let mut left2 = vec![];
            for temp in left.iter() {
                let (delta, other) = temp.intersect(&cube_lst[j]);
                left2.extend(other);
                count += delta;
            }
            left = left2;
            if left.is_empty() {
                break;
            }
        }
        if cube.on {
            for cube in left {
                count += cube.count();
            }
        }
    }
    count as usize
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day22.txt");
    let re = Regex::new(
        r"^(on|off) x=([-0-9]+)\.\.([-0-9]+),y=([-0-9]+)\.\.([-0-9]+),z=([-0-9]+)\.\.([-0-9]+)",
    )
    .unwrap();
    let n = lines.len();
    let mut cube_lst: Vec<Cube> = Vec::with_capacity(n);
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let state = caps.get(1).unwrap().as_str() == "on";
        let pos: Vec<T> = (2..8)
            .into_iter()
            .map(|i| caps.get(i).unwrap().as_str().parse::<T>().unwrap())
            .collect();
        cube_lst.push(Cube::from_sp(pos, state));
    }

    let mut count = 0;
    for (i, cube) in cube_lst.iter().enumerate() {
        let mut left = vec![*cube];
        let mut j = i;
        while j > 0 {
            j -= 1;
            let mut left2 = vec![];
            for temp in left.iter() {
                let (delta, other) = temp.intersect(&cube_lst[j]);
                left2.extend(other);
                count += delta;
            }
            left = left2;
            if left.is_empty() {
                break;
            }
        }
        if cube.on {
            for cube in left {
                count += cube.count();
            }
        }
    }
    count as usize
}

#[test]
fn test_22() {
    assert_eq!(607657, part1());
    assert_eq!(1187742789778677, part2());
}
