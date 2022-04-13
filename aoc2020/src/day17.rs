use crate::read_lines;
use std::fmt;

const NC: usize = 6;

pub struct Cube {
    is_odd: bool,
    n_xyz: [usize; 3],
    d1_xyz: [usize; 3],
    dn_xyz: [usize; 3],
    data: Vec<Vec<Vec<u8>>>,
}

impl Cube {
    pub fn new(is_odd: bool, nx: usize, ny: usize, nz: usize) -> Self {
        Cube {
            is_odd,
            n_xyz: [nx, ny, nz],
            d1_xyz: [0; 3],
            dn_xyz: [0; 3],
            data: vec![vec![vec![0u8; nx + 4]; ny + 4]; nz + 3],
        }
    }

    pub fn init(&mut self, lines: &[String]) {
        for (j, line) in lines.iter().enumerate() {
            let line = line.as_bytes();
            for (i, &c) in line.iter().enumerate() {
                self.data[1][j + 2][i + 2] = if c == b'#' { 1 } else { 0 };
            }
        }
        // let n = lines.len();
        // for j in 0..n {
        //     let line = lines[j].as_bytes();
        //     for i in 0..n {
        //         self.data[1][j + 2][i + 2] = if line[i] == b'#' { 1 } else { 0 };
        //     }
        // }
    }

    pub fn next_cycle(&self) -> Self {
        let mut new = Self::new(
            self.is_odd,
            2 + self.n_xyz[0] - self.d1_xyz[0] - self.dn_xyz[0],
            2 + self.n_xyz[1] - self.d1_xyz[1] - self.dn_xyz[1],
            1 + self.n_xyz[2] - self.d1_xyz[2] - self.dn_xyz[2],
        );
        for z in new.gen_range('z') {
            for y in new.gen_range('y') {
                for x in new.gen_range('x') {
                    new.update(x, y, z, self);
                }
            }
        }
        new.update_delta();
        new.update_z0();
        new
    }

    pub fn count_active(&self) -> usize {
        let mut ret = 0;
        for k in self.gen_range('z') {
            ret += self.count_slice('z', k) << 1;
        }
        if self.is_odd {
            ret -= self.count_slice('z', 1);
        }
        ret
    }

    #[inline]
    fn gen_range(&self, c: char) -> std::ops::Range<usize> {
        match c {
            'x' => 2 + self.d1_xyz[0]..self.n_xyz[0] - self.dn_xyz[0] + 2,
            'y' => 2 + self.d1_xyz[1]..self.n_xyz[1] - self.dn_xyz[1] + 2,
            'z' => 1 + self.d1_xyz[2]..self.n_xyz[2] - self.dn_xyz[2] + 1,
            _ => unreachable!(),
        }
    }

    fn update(&mut self, x: usize, y: usize, z: usize, origin: &Cube) {
        let mut acc = 0;
        let x0 = origin.d1_xyz[0] + x - 2;
        let y0 = origin.d1_xyz[1] + y - 2;
        let z0 = origin.d1_xyz[2] + z - 1;
        for k in z0..z0 + 3 {
            for j in y0..y0 + 3 {
                for i in x0..x0 + 3 {
                    acc += origin.data[k][j][i];
                }
            }
        }
        if origin.data[z0 + 1][y0 + 1][x0 + 1] == 1 {
            if acc == 3 || acc == 4 {
                self.data[z][y][x] = 1;
            }
        } else if acc == 3 {
            self.data[z][y][x] = 1;
        }
    }

    fn update_delta(&mut self) {
        for (i, c) in ['x', 'y'].into_iter().enumerate() {
            while self.count_slice(c, self.d1_xyz[i] + 2) == 0 {
                self.d1_xyz[i] += 1;
            }
            while self.count_slice(c, self.n_xyz[i] - self.dn_xyz[i] + 1) == 0 {
                self.dn_xyz[i] += 1;
            }
        }
        let i = 2;
        while self.count_slice('z', self.n_xyz[i] - self.dn_xyz[i]) == 0 {
            self.dn_xyz[i] += 1;
        }
    }

    fn update_z0(&mut self) {
        if self.is_odd {
            for x in self.gen_range('x') {
                for y in self.gen_range('y') {
                    self.data[0][y][x] = self.data[2][y][x];
                }
            }
        } else {
            for x in self.gen_range('x') {
                for y in self.gen_range('y') {
                    self.data[0][y][x] = self.data[1][y][x];
                }
            }
        }
    }

    fn count_slice(&self, c: char, idx: usize) -> usize {
        let mut ret = 0;
        match c {
            'x' => {
                for k in self.gen_range('z') {
                    for j in self.gen_range('y') {
                        ret += self.data[k][j][idx] as usize;
                    }
                }
            }
            'y' => {
                for k in self.gen_range('z') {
                    for i in self.gen_range('x') {
                        ret += self.data[k][idx][i] as usize;
                    }
                }
            }
            'z' => {
                for j in self.gen_range('y') {
                    for i in self.gen_range('x') {
                        ret += self.data[idx][j][i] as usize;
                    }
                }
            }
            _ => unreachable!(),
        }
        ret
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for k in self.gen_range('z') {
            for j in self.gen_range('y') {
                for i in self.gen_range('x') {
                    write!(f, "{}", if self.data[k][j][i] == 1 { '#' } else { '.' })?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

pub fn part1() -> usize {
    let lines = read_lines("./data/day17.txt");
    let mut cube = Cube::new(true, lines[0].len(), lines.len(), 1);
    cube.init(&lines);

    for _ in 0..NC {
        cube = cube.next_cycle();
    }

    // println!("{}", &cube);
    cube.count_active()
}

pub struct Dim4 {
    n_xyz: [usize; 4],
    d1_xyz: [usize; 4],
    dn_xyz: [usize; 4],
    data: Vec<Vec<Vec<Vec<u8>>>>,
}

impl Dim4 {
    pub fn new(nx: usize, ny: usize, nz: usize, nw: usize) -> Self {
        Dim4 {
            n_xyz: [nx, ny, nz, nw],
            d1_xyz: [0; 4],
            dn_xyz: [0; 4],
            data: vec![vec![vec![vec![0u8; nx + 4]; ny + 4]; nz + 4]; nw + 4],
        }
    }

    pub fn init(&mut self, lines: &[String]) {
        for (j, line) in lines.iter().map(|line| line.as_bytes()).enumerate() {
            //let line = line.as_bytes();
            for (i, &c) in line.iter().enumerate() {
                self.data[2][2][j + 2][i + 2] = if c == b'#' { 1 } else { 0 };
            }
        }
        // let n = lines.len();
        // for j in 0..n {
        //     let line = lines[j].as_bytes();
        //     for i in 0..n {
        //         self.data[2][2][j + 2][i + 2] = if line[i] == b'#' { 1 } else { 0 };
        //     }
        // }
    }

    pub fn next_cycle(&self) -> Self {
        let mut new = Self::new(
            2 + self.n_xyz[0] - self.d1_xyz[0] - self.dn_xyz[0],
            2 + self.n_xyz[1] - self.d1_xyz[1] - self.dn_xyz[1],
            2 + self.n_xyz[2] - self.d1_xyz[2] - self.dn_xyz[2],
            2 + self.n_xyz[3] - self.d1_xyz[3] - self.dn_xyz[3],
        );
        for w in new.gen_range('w') {
            for z in new.gen_range('z') {
                for y in new.gen_range('y') {
                    for x in new.gen_range('x') {
                        new.update(x, y, z, w, self);
                    }
                }
            }
        }
        new.update_delta();
        new
    }

    pub fn count_active(&self) -> usize {
        self.gen_range('w').map(|k| self.count_slice('w', k)).sum()
    }

    fn get_idx(&self, c: char) -> usize {
        match c {
            'x' => 0,
            'y' => 1,
            'z' => 2,
            'w' => 3,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn gen_range(&self, c: char) -> std::ops::Range<usize> {
        let i = self.get_idx(c);
        2 + self.d1_xyz[i]..self.n_xyz[i] - self.dn_xyz[i] + 2
    }

    fn update(&mut self, x: usize, y: usize, z: usize, w: usize, origin: &Dim4) {
        let mut acc = 0;
        let x0 = origin.d1_xyz[0] + x - 2;
        let y0 = origin.d1_xyz[1] + y - 2;
        let z0 = origin.d1_xyz[2] + z - 2;
        let w0 = origin.d1_xyz[3] + w - 2;
        for l in w0..w0 + 3 {
            for k in z0..z0 + 3 {
                for j in y0..y0 + 3 {
                    for i in x0..x0 + 3 {
                        acc += origin.data[l][k][j][i];
                    }
                }
            }
        }
        if origin.data[w0 + 1][z0 + 1][y0 + 1][x0 + 1] == 1 {
            if acc == 3 || acc == 4 {
                self.data[w][z][y][x] = 1;
            }
        } else if acc == 3 {
            self.data[w][z][y][x] = 1;
        }
    }

    fn update_delta(&mut self) {
        for (i, c) in ['x', 'y', 'z', 'w'].into_iter().enumerate() {
            while self.count_slice(c, self.d1_xyz[i] + 2) == 0 {
                self.d1_xyz[i] += 1;
            }
            while self.count_slice(c, self.n_xyz[i] - self.dn_xyz[i] + 1) == 0 {
                self.dn_xyz[i] += 1;
            }
        }
    }

    fn count_slice(&self, c: char, i: usize) -> usize {
        // TODO: simplify code
        let mut ret = 0;
        match c {
            'x' => {
                for y in self.gen_range('y') {
                    for z in self.gen_range('z') {
                        for w in self.gen_range('w') {
                            ret += self.data[w][z][y][i] as usize;
                        }
                    }
                }
            }
            'y' => {
                for x in self.gen_range('x') {
                    for z in self.gen_range('z') {
                        for w in self.gen_range('w') {
                            ret += self.data[w][z][i][x] as usize;
                        }
                    }
                }
            }
            'z' => {
                for x in self.gen_range('x') {
                    for y in self.gen_range('y') {
                        for w in self.gen_range('w') {
                            ret += self.data[w][i][y][x] as usize;
                        }
                    }
                }
            }
            'w' => {
                for x in self.gen_range('x') {
                    for y in self.gen_range('y') {
                        for z in self.gen_range('z') {
                            ret += self.data[i][z][y][x] as usize;
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        ret
    }
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day17.txt");
    let mut dim4 = Dim4::new(lines[0].len(), lines.len(), 1, 1);
    dim4.init(&lines);

    for _ in 0..NC {
        dim4 = dim4.next_cycle();
    }
    dim4.count_active()
}

#[test]
fn test_17() {
    assert_eq!(304, part1());
    assert_eq!(1868, part2());
}
