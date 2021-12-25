use crate::read_lines;
use std::fmt;

pub fn part1() -> usize {
    let lines = read_lines("./data/day25.txt");
    let mut mat = vec![];
    for line in lines {
        let mut tmp: Vec<u8> = vec![];
        for c in line.bytes() {
            tmp.push(match c {
                b'.' => 0,
                b'>' => 1,
                b'v' => 2,
                _ => unreachable!(),
            });
        }
        mat.push(tmp);
    }
    Mat::new(mat).move_until()
}

struct Mat {
    data: Vec<Vec<u8>>,
    nrow: usize,
    ncol: usize,
}

impl Mat {
    fn new(data: Vec<Vec<u8>>) -> Self {
        let nrow = data.len();
        let ncol = data[0].len();
        Self { data, nrow, ncol }
    }

    fn move_until(&mut self) -> usize {
        let mut count = 1;
        while (self.move_east() + self.move_south()) > 0 {
            count += 1;
            // dbg!(count);
            // dbg!(&self);
        }
        count
    }

    fn move_east(&mut self) -> usize {
        let mut new = self.data.clone();
        let mut count = 0;
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                if self.data[i][j] == 1 {
                    let j2 = if j == self.ncol - 1 { 0 } else { j + 1 };
                    if self.data[i][j2] == 0 {
                        new[i][j2] = 1;
                        new[i][j] = 0;
                        count += 1
                    }
                }
            }
        }
        self.data = new;
        count
    }

    fn move_south(&mut self) -> usize {
        let mut new = self.data.clone();
        let mut count = 0;
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                if self.data[i][j] == 2 {
                    let i2 = if i == self.nrow - 1 { 0 } else { i + 1 };
                    if self.data[i2][j] == 0 {
                        new[i2][j] = 2;
                        new[i][j] = 0;
                        count += 1;
                    }
                }
            }
        }
        self.data = new;
        count
    }
}

impl fmt::Debug for Mat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                write!(
                    f,
                    "{}",
                    match self.data[i][j] {
                        0 => '.',
                        1 => '>',
                        2 => 'v',
                        _ => unreachable!(),
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}
