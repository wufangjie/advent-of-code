use crate::read_lines;
use std::fmt;

// 看到第一个是 #, 我以为是题目错了, 不然整个空间都亮了
// 想了半天不明白, 最后发现只要最后一个不是 #, 再变一次外面的空间就会再次变暗

pub fn part1() -> usize {
    let mut iter = read_lines("./data/day20.txt").into_iter();
    let algorithm: Vec<u8> = iter
        .next()
        .unwrap()
        .bytes()
        .map(|c| u8::from(c == b'#')) //if c == b'#' { 1 } else { 0 })
        .collect();
    let rows: Vec<String> = iter
        .by_ref()
        .skip(1)
        .take_while(|line| !line.is_empty()) // != "")
        .collect();

    let nrow = rows.len();
    let ncol = rows[0].len();
    let mut data = vec![vec![0; ncol + 4]; nrow + 4];
    for i in 0..nrow {
        for j in 0..ncol {
            if &rows[i][j..j + 1] == "#" {
                data[i + 2][j + 2] = 1;
            }
        }
    }

    let mut img = Image::new(data, algorithm);
    for _ in 0..2 {
        img.enhance();
        //dbg!(&img);
    }
    img.count_lit()
}

struct Image {
    data: Vec<Vec<u8>>,
    algorithm: Vec<u8>,
    nrow: usize,
    ncol: usize,
}

impl Image {
    fn new(data: Vec<Vec<u8>>, algorithm: Vec<u8>) -> Self {
        let nrow = data.len();
        let ncol = data[0].len();
        Self {
            data,
            algorithm,
            nrow,
            ncol,
        }
    }

    fn get_next_default(&self) -> u8 {
        if self.data[0][0] == 0 {
            //dbg!(&self.algorithm[0]);
            self.algorithm[0]
        } else {
            *self.algorithm.last().unwrap()
        }
    }

    fn enhance(&mut self) {
        let mut data_new = vec![vec![self.get_next_default(); self.ncol + 2]; self.nrow + 2];
        for i in 1..self.nrow - 1 {
            for j in 1..self.ncol - 1 {
                data_new[i + 1][j + 1] = self.lookup(i, j);
            }
        }
        self.data = data_new;
        self.nrow += 2;
        self.ncol += 2;
    }

    fn lookup(&self, i: usize, j: usize) -> u8 {
        self.algorithm[((self.data[i - 1][j - 1] as usize) << 8)
            | ((self.data[i - 1][j] << 7)
                | (self.data[i - 1][j + 1] << 6)
                | (self.data[i][j - 1] << 5)
                | (self.data[i][j] << 4)
                | (self.data[i][j + 1] << 3)
                | (self.data[i + 1][j - 1] << 2)
                | (self.data[i + 1][j] << 1)
                | (self.data[i + 1][j + 1])) as usize]
    }

    fn count_lit(&self) -> usize {
        let mut count = 0;
        for i in 2..self.nrow - 2 {
            for j in 2..self.ncol - 2 {
                if self.data[i][j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for i in 2..self.nrow - 2 {
            for j in 2..self.ncol - 2 {
                write!(f, "{}", if self.data[i][j] == 1 { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

pub fn part2() -> usize {
    let mut iter = read_lines("./data/day20.txt").into_iter();
    let algorithm: Vec<u8> = iter
        .next()
        .unwrap()
        .bytes()
        .map(|c| u8::from(c == b'#')) //if c == b'#' { 1 } else { 0 })
        .collect();
    let rows: Vec<String> = iter
        .by_ref()
        .skip(1)
        .take_while(|line| !line.is_empty()) // != "")
        .collect();

    let nrow = rows.len();
    let ncol = rows[0].len();
    let mut data = vec![vec![0; ncol + 4]; nrow + 4];
    for i in 0..nrow {
        for j in 0..ncol {
            if &rows[i][j..j + 1] == "#" {
                data[i + 2][j + 2] = 1;
            }
        }
    }

    let mut img = Image::new(data, algorithm);
    for _ in 0..50 {
        img.enhance();
        //dbg!(&img);
    }
    img.count_lit()
}

#[test]
fn test_20() {
    assert_eq!(5563, part1());
    assert_eq!(19743, part2());
}
