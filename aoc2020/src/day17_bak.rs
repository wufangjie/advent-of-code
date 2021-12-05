use crate::read_lines;

const NC: usize = 6;

pub struct Cube {
    m: usize,
    n: usize,
    nz: usize,
    data: Vec<Vec<Vec<u8>>>,
}

impl Cube {
    pub fn new(m: usize) -> Self {
        let n = m - (NC << 1) - 2;
        Cube {
            m,
            n,
            nz: 2,
            data: vec![vec![vec![0u8; m]; m]; NC + 3],
        }
    }

    pub fn expand(&self) -> Self {
        let mut new = Self::new(self.m);
        new.n = self.n + 2;
        new.nz = self.nz + 1;
        let lo = (new.m - new.n) >> 1;
        for z in 1..new.nz {
            for y in lo..lo + new.n {
                for x in lo..lo + new.n {
                    new.update(x, y, z, &self);
                }
            }
        }
        new.update_z0();
        new
    }

    pub fn display_slice(&self, mut z: usize) {
        let lo = (self.m - self.n) >> 1;
        for i in lo..lo + self.n {
            for j in lo..lo + self.n {
                print!("{}", if self.data[z][i][j] == 1 { '#' } else { '.' });
            }
            println!("");
        }
        println!("");
    }

    pub fn count_active(&self) -> usize {
        let lo = (self.m - self.n) >> 1;
        let mut ret = 0;
        for k in 2..self.nz {
            ret += self.count_slice(k) << 1;
        }
        if self.n & 1 == 1 {
            ret += self.count_slice(1);
        } else {
            ret += self.count_slice(1) << 1;
        }
        ret
    }

    fn update(&mut self, x: usize, y: usize, z: usize, origin: &Cube) {
        let mut acc = 0;
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                for k in z - 1..=z + 1 {
                    acc += origin.data[k][j][i];
                }
            }
        }
        if origin.data[z][y][x] == 1 {
            if (acc == 3 || acc == 4) {
                self.data[z][y][x] = 1;
            }
        } else {
            if acc == 3 {
                self.data[z][y][x] = 1;
            }
        }
    }

    fn update_z0(&mut self) {
        let lo = (self.m - self.n) >> 1;
        if self.n & 1 == 1 {
            for x in lo..lo + self.n {
                for y in lo..lo + self.n {
                    self.data[0][y][x] = self.data[2][y][x];
                }
            }
        } else {
            for x in lo..lo + self.n {
                for y in lo..lo + self.n {
                    self.data[0][y][x] = self.data[1][y][x];
                }
            }
        }
    }

    fn count_slice(&self, k: usize) -> usize {
        let lo = (self.m - self.n) >> 1;
        let mut ret = 0;
        for j in lo..lo + self.n {
            for i in lo..lo + self.n {
                ret += self.data[k][j][i] as usize;
            }
        }
        ret
    }
}

pub fn part1() {
    let lines = read_lines("./data/day17_test.txt");
    let n = lines.len();
    let m = n + (NC << 1) + 2;

    let mut cube = Cube::new(m);

    let lines = vec![".#.", "..#", "###"];

    let lo = (m - n) >> 1;
    for i in 0..n {
        let line = lines[i].as_bytes();
        for j in 0..n {
            cube.data[1][lo + i][lo + j] = if line[j] == b'#' { 1 } else { 0 };
        }
    }

    for _ in 0..6 {
        cube = cube.expand();
    }

    for i in 1..cube.nz {
        cube.display_slice(i);
    }

    dbg!(cube.count_active());
}
