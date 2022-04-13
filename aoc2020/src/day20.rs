use crate::read_lines;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

const N: usize = 10; // single edge's pixels
const M: usize = 12; // image edge's tiles (count)
const MN: usize = M * (N - 2); // image edge's pixels
const CW: [Pos; 4] = [Pos::Up, Pos::Right, Pos::Down, Pos::Left];

type F1_2 = fn(usize) -> (usize, usize);
//type f2_2 = fn(usize, usize) -> (usize, usize);

// NOTE: The borders of each tile are not part of the actual image;
// start by removing them.
// the problem is hard, but the data is easy enough to solve (greedy)

#[derive(Debug, Clone, Copy)]
enum Pos {
    Up,
    Right,
    Down,
    Left,
}

// impl Pos {
//     fn from_i64(i: i64) -> Self {
//         match match i % 4 {
//             x if x >= 0 => x,
//             x => x + 4,
//         } {
//             0 => Self::Up,
//             1 => Self::Right,
//             2 => Self::Down,
//             3 => Self::Left,
//             _ => unreachable!(),
//         }
//     }
// }

pub fn part1() -> usize {
    let lines = read_lines("./data/day20.txt");
    let (tile_ids, urdls) = get_tiles_and_urdls(&lines);

    let mut count = HashMap::new();
    for lst in &urdls {
        for &d in lst.iter() {
            let d2 = get_reverse_edge(d);
            *count.entry(d).or_insert(0) += 1;
            *count.entry(d2).or_insert(0) += 1;
        }
    }

    let mut res = 1;
    for i in 0..urdls.len() {
        let mut count_edge = 0;
        for d in urdls[i] {
            if *count.get(&d).unwrap() == 1 {
                count_edge += 1;
            }
        }
        if count_edge == 2 {
            // dbg!(i);
            res *= tile_ids[i];
        }
    }
    res
}

fn get_tiles_and_urdls(lines: &[String]) -> (Vec<usize>, Vec<[u32; 4]>) {
    // only for part1
    let re_num = Regex::new(r"(\d+)").unwrap();
    let mut tile_ids: Vec<usize> = vec![];
    let mut urdls = vec![]; // (up, right, down, left)
    for i in (0..lines.len()).into_iter().step_by(N + 2) {
        tile_ids.push(
            re_num
                .captures(&lines[i])
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
        );

        let mut up = 0;
        for c in lines[i + 1].as_bytes() {
            up <<= 1;
            if let b'#' = c {
                up |= 1;
            }
        }

        let mut down = 0;
        for c in lines[i + N].as_bytes() {
            down <<= 1;
            if let b'#' = c {
                down |= 1;
            }
        }

        let mut left = 0;
        let mut right = 0;
        for j in 1..N + 1 {
            left <<= 1;
            right <<= 1;
            if let b'#' = lines[i + j].as_bytes()[0] {
                left |= 1;
            }
            if let b'#' = lines[i + j].as_bytes()[N - 1] {
                right |= 1;
            }
        }

        urdls.push([up, right, down, left]);
    }
    (tile_ids, urdls)
}

fn get_reverse_edge(mut d: u32) -> u32 {
    // only for part1
    let mut ret = 0;
    for i in 0..N {
        ret |= (d & 1) << (N - i - 1);
        d >>= 1;
    }
    ret as u32
}

// --------------------

pub fn part2() -> usize {
    let lines = read_lines("./data/day20.txt");
    let mut tiles = get_tiles(&lines);
    let n = tiles.len();

    let mut count = HashMap::new();
    let mut edge_tid_dct = HashMap::new();

    for (i, tile) in tiles.iter().enumerate() {
        for p in CW {
            for d in [tile.calc_edge_code(p), tile.calc_edge_code_rev(p)] {
                (*edge_tid_dct.entry(d).or_insert(vec![])).push(i);
                *count.entry(d).or_insert(0) += 1;
            }
        }
    }

    // for i in 0..n {
    //     for p in CW {
    //         for d in [tiles[i].calc_edge_code(p), tiles[i].calc_edge_code_rev(p)] {
    //             (*edge_tid_dct.entry(d).or_insert(vec![])).push(i);
    //             *count.entry(d).or_insert(0) += 1;
    //         }
    //     }
    // }

    // find left-top one
    let mut lo = 0usize; // NOTE: not local
    let rotate_map = [[0, 3, 0, 0], [3, 0, 2, 0], [0, 2, 0, 1], [0, 0, 1, 0]];
    let mut rotate_ij = vec![];

    for (i, tile) in tiles.iter().enumerate() {
        rotate_ij.clear();
        for (j, p) in CW.into_iter().enumerate() {
            if *count.get(&tile.calc_edge_code(p)).unwrap() == 1
            // && *count.get(&tiles[i].calc_edge_code_rev(p)).unwrap() == 1
            {
                rotate_ij.push(j);
            }
        }
        if rotate_ij.len() == 2 {
            lo = i;
            break;
        }
    }

    // for i in 0..n {
    //     rotate_ij.clear();
    //     for (j, p) in CW.into_iter().enumerate() {
    //         if *count.get(&tiles[i].calc_edge_code(p)).unwrap() == 1
    //         // && *count.get(&tiles[i].calc_edge_code_rev(p)).unwrap() == 1
    //         {
    //             rotate_ij.push(j);
    //         }
    //     }
    //     if rotate_ij.len() == 2 {
    //         lo = i;
    //         break;
    //     }
    // }
    let mut used = vec![false; n];
    used[lo] = true;
    tiles[lo].rotate(rotate_map[rotate_ij[0]][rotate_ij[1]]);

    // let mut pre = Tile::new();

    // find layout
    let mut layout = [[999; M]; M];
    for i in 0..M {
        if i == 0 {
            layout[0][0] = lo;
        } else {
            //std::mem::swap(&mut pre, &mut tiles[layout[i - 1][0]]);
            let pre = &tiles[layout[i - 1][0]].clone();
            for &idx in edge_tid_dct.get(&pre.calc_edge_code(Pos::Down)).unwrap() {
                if !used[idx] {
                    tiles.get_mut(idx).unwrap().attach_up(pre);
                    layout[i][0] = idx;
                    used[idx] = true;
                    break;
                }
            }
            //std::mem::swap(&mut pre, &mut tiles[layout[i - 1][0]]);
        }
        for j in 1..M {
            //std::mem::swap(&mut pre, &mut tiles[layout[i][j - 1]]);
            let pre = &tiles[layout[i][j - 1]].clone();
            for &idx in edge_tid_dct.get(&pre.calc_edge_code(Pos::Right)).unwrap() {
                if !used[idx] {
                    tiles.get_mut(idx).unwrap().attach_left(pre);
                    layout[i][j] = idx;
                    used[idx] = true;
                    if i > 0 {
                        assert_eq!(
                            tiles[idx].calc_edge_code(Pos::Up),
                            tiles[layout[i - 1][j]].calc_edge_code(Pos::Down)
                        );
                    }
                    break;
                }
            }
            //std::mem::swap(&mut pre, &mut tiles[layout[i][j - 1]]);
        }
    }

    // find monster
    let mut image = Image::from_layout(&tiles, &layout);
    for flip_or_not in [false, true] {
        image.flip(flip_or_not);
        for i in 0..4 {
            image.rotate(i);
            let m = image.count_monster();
            if m > 0 {
                return image.count_water(m);
            }
        }
    }
    unreachable!()
}

#[derive(Debug, Clone)]
struct Tile {
    data: [[bool; N]; N],
    id: usize,
}

impl Tile {
    // fn new() -> Self {
    //     Self {
    //         data: [[false; N]; N],
    //         id: 0,
    //     }
    // }

    fn from_iter<'a>(lines: impl Iterator<Item = &'a String>, id: usize) -> Self {
        let mut data = [[false; N]; N];
        // let nc = lines[0].len();
        for (i, line) in lines.enumerate() {
            //.iter()
            for (j, &c) in line.as_bytes().iter().enumerate() {
                if c == b'#' {
                    data[i][j] = true;
                }
            }
        }

        // for i in 0..lines.len() {
        //     let bts = lines[i].as_bytes();
        //     for j in 0..nc {
        //         if bts[j] == b'#' {
        //             data[i][j] = true;
        //         }
        //     }
        // }
        Self { data, id }
    }

    fn rotate(&mut self, mut i: usize) {
        // right rotate
        i %= 4;
        if i == 0 {
            return;
        }
        let mut new = [[false; N]; N];
        match i {
            1 => {
                for (i, row) in new.iter_mut().enumerate() {
                    for (j, item) in row.iter_mut().enumerate() {
                        *item = self.data[N - 1 - j][i];
                    }
                }
                // for i in 0..N {
                //     for j in 0..N {
                //         new[j][N - i - 1] = self.data[i][j];
                //     }
                // }
            }
            2 => {
                for (i, row) in new.iter_mut().enumerate() {
                    for (j, item) in row.iter_mut().enumerate() {
                        *item = self.data[N - 1 - i][N - 1 - j];
                    }
                }
                // for i in 0..N {
                //     for j in 0..N {
                //         new[N - i - 1][N - j - 1] = self.data[i][j];
                //     }
                // }
            }
            _ => {
                for (i, row) in new.iter_mut().enumerate() {
                    for (j, item) in row.iter_mut().enumerate() {
                        *item = self.data[j][N - 1 - i];
                    }
                }
                // for i in 0..N {
                //     for j in 0..N {
                //         new[N - j - 1][i] = self.data[i][j];
                //     }
                // }
            }
        }
        self.data = new;
    }

    fn flip(&mut self, flip_or_not: bool) {
        // left <-> right is enough
        if flip_or_not {
            let mut new = [[false; N]; N];
            for (i, row) in new.iter_mut().enumerate() {
                for (j, item) in row.iter_mut().enumerate() {
                    *item = self.data[i][N - 1 - j];
                }
            }

            // for i in 0..N {
            //     for j in 0..N {
            //         new[i][N - j - 1] = self.data[i][j];
            //     }
            // }
            self.data = new;
        }
    }

    fn _calc_edge_code_by_iter<I>(&self, iter: I) -> u32
    where
        I: Iterator<Item = (usize, usize)>,
    {
        let mut ret = 0u32;
        for (i, j) in iter {
            ret <<= 1;
            if self.data[i][j] {
                ret |= 1;
            }
        }
        ret
    }

    fn edge_iter(p: Pos) -> F1_2 {
        match p {
            Pos::Up => (|x| (0, x)) as F1_2,
            Pos::Right => (|x| (x, N - 1)) as F1_2,
            Pos::Down => (|x| (N - 1, x)) as F1_2,
            Pos::Left => (|x| (x, 0)) as F1_2,
        }
    }

    fn calc_edge_code(&self, p: Pos) -> u32 {
        self._calc_edge_code_by_iter((0..N).into_iter().map(Self::edge_iter(p)))
    }

    fn calc_edge_code_rev(&self, p: Pos) -> u32 {
        self._calc_edge_code_by_iter((0..N).into_iter().rev().map(Self::edge_iter(p)))
    }

    fn attach_left(&mut self, left: &Self) {
        let to_attach = left.calc_edge_code(Pos::Right);
        for flip_or_not in [false, true] {
            self.flip(flip_or_not);
            for (i, p) in [Pos::Left, Pos::Down, Pos::Right, Pos::Up]
                .into_iter()
                .enumerate()
            {
                if if i < 2 {
                    self.calc_edge_code(p)
                } else {
                    self.calc_edge_code_rev(p)
                } == to_attach
                {
                    self.rotate(i);
                    return;
                }
            }
        }
    }

    fn attach_up(&mut self, up: &Self) {
        let to_attach = up.calc_edge_code(Pos::Down);
        for flip_or_not in [false, true] {
            self.flip(flip_or_not);
            for (i, p) in [Pos::Up, Pos::Left, Pos::Down, Pos::Right]
                .into_iter()
                .enumerate()
            {
                if if i == 0 || i == 3 {
                    self.calc_edge_code(p)
                } else {
                    self.calc_edge_code_rev(p)
                } == to_attach
                {
                    self.rotate(i);
                    return;
                }
            }
        }
    }
}

struct Image {
    data: [[bool; MN]; MN],
}

impl Image {
    fn from_layout(tiles: &[Tile], layout: &[[usize; M]; M]) -> Self {
        let mut data = [[false; MN]; MN];
        let mut i0 = 0;
        let mut j0; // = 0;
        for i in 0..M {
            j0 = 0;
            for j in 0..M {
                let temp = &tiles[layout[i][j]].data;
                for ii in 1..N - 1 {
                    for jj in 1..N - 1 {
                        data[i0 + ii - 1][j0 + jj - 1] = temp[ii][jj];
                    }
                }
                j0 += N - 2;
            }
            i0 += N - 2;
        }
        Self { data }
    }

    fn rotate(&mut self, mut i: usize) {
        // right rotate
        i %= 4;
        if i == 0 {
            return;
        }
        let mut new = [[false; MN]; MN];
        match i {
            1 => {
                for (i, row) in new.iter_mut().enumerate() {
                    for (j, item) in row.iter_mut().enumerate() {
                        *item = self.data[MN - 1 - j][i];
                    }
                }
            }
            2 => {
                for (i, row) in new.iter_mut().enumerate() {
                    for (j, item) in row.iter_mut().enumerate() {
                        *item = self.data[MN - 1 - i][MN - 1 - j];
                    }
                }
            }
            3 => {
                for (i, row) in new.iter_mut().enumerate() {
                    for (j, item) in row.iter_mut().enumerate() {
                        *item = self.data[j][MN - 1 - i];
                    }
                }
            }
            _ => unreachable!(), // i: usize
        }
        self.data = new;
    }

    fn flip(&mut self, flip_or_not: bool) {
        if flip_or_not {
            let mut new = [[false; MN]; MN];
            for (i, row) in new.iter_mut().enumerate() {
                for (j, item) in row.iter_mut().enumerate() {
                    *item = self.data[i][MN - 1 - j];
                }
            }
            // for i in 0..MN {
            //     for j in 0..MN {
            //         new[i][MN - j - 1] = self.data[i][j];
            //     }
            // }
            self.data = new;
        }
    }

    fn count_monster(&self) -> usize {
        //  01234567890123456789
        // 0                  #
        // 1#    ##    ##    ###
        // 2 #  #  #  #  #  #
        let delta = [
            (0, 18),
            (1, 0),
            (1, 5),
            (1, 6),
            (1, 11),
            (1, 12),
            (1, 17),
            (1, 18),
            (1, 19),
            (2, 1),
            (2, 4),
            (2, 7),
            (2, 10),
            (2, 13),
            (2, 16),
        ];
        let mut count = 0;
        for i0 in 0..MN - 3 {
            'next: for j0 in 0..MN - 20 {
                for (di, dj) in delta.iter() {
                    if !self.data[i0 + di][j0 + dj] {
                        continue 'next;
                    }
                }
                count += 1;
            }
        }
        count
    }

    fn count_water(&self, monster: usize) -> usize {
        self.data
            .iter()
            .map(|sub| sub.iter().map(|&x| x as usize).sum::<usize>())
            .sum::<usize>()
            - monster * 15
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for i in 0..N {
            for j in 0..N {
                write!(f, "{}", if self.data[i][j] { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..MN {
            for j in 0..MN {
                write!(f, "{}", if self.data[i][j] { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn get_tiles(lines: &[String]) -> Vec<Tile> {
    let re_num = Regex::new(r"(\d+)").unwrap();
    let mut iter = lines.iter();
    let mut ret = vec![];
    for _ in 0..M * M {
        let tid = re_num
            .captures(iter.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        //ret.push(Tile::from_iter(&iter.by_ref().take(N).collect(), tid));
        ret.push(Tile::from_iter(iter.by_ref().take(N), tid));
        iter.next();
    }
    ret
}

// //奇怪的语法增加了:
// match match i % 4 {
//     x if x >= 0 => x,
//     x => x + 4,
// } {

// if if i < 2 {
//     self.calc_edge_code(p)
// } else {
//     self.calc_edge_code_rev(p)
// } == to_attach

#[test]
fn test_20() {
    assert_eq!(60145080587029, part1());
    assert_eq!(1901, part2());
}
