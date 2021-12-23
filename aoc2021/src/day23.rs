use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use utils::Heap;

const N: usize = 4;
const EPM: [usize; 4] = [1, 10, 100, 1000]; // energy cost per move

// NOTE: Amphipods will never stop on the space immediately outside any room
// so following change is wrong
// #############    #############
// #A........BD#    #A.......B.D#
// ###B#C#.#.### to ###B#C#.#.###
//   #D#C#B#.#	 =>   #D#C#B#.#
//   #D#B#A#C#	      #D#B#A#C#
//   #A#D#C#A#	      #A#D#C#A#
//   #########	      #########

// A* + greedy + prune(dead lock)
// NOTE: I add backtrace for debugging, remove it will save some time and space

pub fn part2() -> usize {
    let m = Machine::new([
        // [b'A', b'B'],
        // [b'D', b'C'],
        // [b'C', b'B'],
        // [b'A', b'D'],
        // [b'A', b'D', b'D', b'B'],
        // [b'D', b'B', b'C', b'C'],
        // [b'C', b'A', b'B', b'B'],
        // [b'A', b'C', b'A', b'D'],
        [b'D', b'D', b'D', b'C'],
        [b'D', b'B', b'C', b'C'],
        [b'B', b'A', b'B', b'A'],
        [b'A', b'C', b'A', b'B'],
    ]);

    let mut visited: HashMap<([[u8; N]; 4], [u8; 11]), (usize, Machine)> = HashMap::new();

    let mut heap = Heap::new();
    heap.push((m.expect(), m));

    'main: while let Some((_, mut m)) = heap.pop() {
        if m.is_finished() {
            let mut m2 = m.clone();
            dbg!(&m2);
            while m2.cost > 0 {
                let key = (m2.abcd, m2.hallway);
                let (_, tmp) = visited.get(&key).unwrap();
                m2 = tmp.clone();
                dbg!(&m2);
            }
            return m.cost;
        } else {
            for i in 0..11 {
                // greedy: if can go home, then just do it
                if let Some(mm) = m.go_home_from_hallway(i) {
                    let thres = match visited.get(&(mm.abcd, mm.hallway)) {
                        Some((v, _)) => *v,
                        None => usize::MAX,
                    };
                    if mm.cost < thres {
                        visited.insert((mm.abcd, mm.hallway), (mm.cost, m.clone()));
                        heap.push((mm.expect(), mm));
                    }
                    continue 'main;
                }
            }

            for i in 0..4 {
                // greedy: same as go_home_from hallway
                if let Some(mm) = m.go_home_from_room(i) {
                    let thres = match visited.get(&(mm.abcd, mm.hallway)) {
                        Some((v, _)) => *v,
                        None => usize::MAX,
                    };
                    if mm.cost < thres {
                        visited.insert((mm.abcd, mm.hallway), (mm.cost, m.clone()));
                        heap.push((mm.expect(), mm));
                    }
                    continue 'main;
                }
            }

            for i in 0..4 {
                for mm in m.move_out(i) {
                    let thres = match visited.get(&(mm.abcd, mm.hallway)) {
                        Some((v, _)) => *v,
                        None => usize::MAX,
                    };
                    if mm.cost < thres {
                        visited.insert((mm.abcd, mm.hallway), (mm.cost, m.clone()));
                        heap.push((mm.expect(), mm));
                    }
                }
            }
        }
    }
    panic!("Can not arrange!");
}

#[derive(Clone, Hash, PartialEq, PartialOrd)]
struct Machine {
    cost: usize,
    abcd: [[u8; N]; 4], // bottom to top
    hallway: [u8; 11],
}

impl Machine {
    fn new(abcd: [[u8; N]; 4]) -> Self {
        Self {
            abcd,
            hallway: [b' '; 11],
            cost: 0,
        }
    }

    fn is_finished(&self) -> bool {
        for i in 0..4 {
            for j in 0..N {
                if self.abcd[i][j] != b'A' + i as u8 {
                    return false;
                }
            }
        }
        true
    }

    fn move_out(&self, i: usize) -> Vec<Self> {
        let mut res = vec![];
        let jj = self.find_top_plus(i);
        if self.abcd[i][..jj].iter().all(|&c| c == b'A' + i as u8) {
            return res;
        }
        if jj > 0 {
            let i_h = (i << 1) + 2;
            if self.hallway[i_h] == b' ' {
                let mut new = self.clone();
                let cc = self.abcd[i][jj - 1]; // current char
                new.abcd[i][jj - 1] = b' ';
                let ii = (cc - b'A') as usize;
                // let ii_h = (ii << 1) + 2;
                new.cost += EPM[ii] * (N - jj + 1);

                let mut j = i_h;
                while j > 0 {
                    j -= 1;
                    if self.hallway[j] != b' ' {
                        // prune dead lock
                        if j >= ii && self.hallway[j] > cc {
                            let hi = i_h.min(self.get_hallway_door(self.hallway[j]) + 1);
                            for _ in j + 1..hi {
                                res.pop();
                            }
                        }
                        break;
                    } else if let 2 | 4 | 6 | 8 = j {
                        // j != ii_h { // if no limit
                        continue;
                    } else {
                        let mut new2 = new.clone();
                        new2.hallway[j] = cc;
                        new2.cost += EPM[ii] * (i_h - j);
                        res.push(new2);
                    }
                }
                let mut j = i_h;
                while j < 10 {
                    j += 1;
                    if self.hallway[j] != b' ' {
                        // prune dead lock
                        if j <= ii && self.hallway[j] < cc {
                            let lo = (i_h + 1).max(self.get_hallway_door(self.hallway[j]));
                            for _ in lo..j {
                                res.pop();
                            }
                        }
                        break;
                    } else if let 2 | 4 | 6 | 8 = j {
                        // j != ii_h // if no limit
                        continue;
                    } else {
                        let mut new2 = new.clone();
                        new2.hallway[j] = cc;
                        new2.cost += EPM[ii] * (j - i_h);
                        res.push(new2);
                    }
                }
            }
        }
        res
    }

    #[inline]
    fn get_hallway_door(&self, c: u8) -> usize {
        ((c - b'A' + 1) as usize) << 1
    }

    fn go_home_from_hallway(&mut self, i: usize) -> Option<Self> {
        if self.hallway[i] != b' ' {
            let ii = (self.hallway[i] - b'A') as usize;
            let ii_h = (ii << 1) + 2;
            let iter = if i < ii_h { i + 1..ii_h + 1 } else { ii_h..i };
            if iter.into_iter().all(|i| self.hallway[i] == b' ') {
                let cost_h = (i as i64 - ii_h as i64).abs() as usize;
                let jj = self.find_top_plus(ii);
                if self.abcd[ii][..jj].iter().all(|&c| c == b'A' + ii as u8) {
                    let mut new = self.clone();
                    new.hallway[i] = b' ';
                    new.abcd[ii][jj] = self.hallway[i];
                    new.cost += (cost_h + N - jj) * EPM[ii];
                    return Some(new);
                }
            }
        }
        None
    }

    fn go_home_from_room(&mut self, i: usize) -> Option<Self> {
        match self.find_top_plus(i) {
            0 => None,
            j => {
                let ii = (self.abcd[i][j - 1] - b'A') as usize;
                if i == ii {
                    None
                } else {
                    let mut new = self.clone();
                    new.abcd[i][j - 1] = b' ';
                    new.cost += (N - j + 1) * EPM[ii];
                    let i_h = (i << 1) + 2;
                    new.hallway[i_h] = self.abcd[i][j - 1];
                    new.go_home_from_hallway(i_h)
                }
            }
        }
    }

    fn find_top_plus(&self, i: usize) -> usize {
        for j in (0..N).into_iter().rev() {
            if self.abcd[i][j] != b' ' {
                return j + 1;
            }
        }
        0
    }

    fn expect(&self) -> usize {
        let mut flag = [0; 4];
        for i in 0..4 {
            for j in 0..N {
                if i as u8 + b'A' != self.abcd[i][j] {
                    flag[i] = j;
                    break;
                }
                flag[i] = N;
            }
        }
        let mut cost = [0; 4];
        let mut count = [0; 4];
        for i in 0..4 {
            let i_h = (i << 1) + 2;
            for j in flag[i]..N {
                if self.abcd[i][j] != b' ' {
                    let ii = (self.abcd[i][j] - b'A') as usize;
                    let ii_h = (ii << 1) + 2;
                    let d_h = ((ii_h as i64 - i_h as i64).abs() as usize).max(2);
                    cost[ii] += ((N - j) + d_h + (N - flag[ii] - count[ii])) * EPM[ii];
                    count[ii] += 1;
                }
            }
        }
        for i in 0..11 {
            if self.hallway[i] != b' ' {
                let ii = (self.hallway[i] - b'A') as usize;
                let ii_h = (ii << 1) + 2;
                let d_h = (ii_h as i64 - i as i64).abs() as usize;
                cost[ii] += (d_h + (N - flag[ii] - count[ii])) * EPM[ii];
                count[ii] += 1;
            }
        }
        self.cost + cost[0] + cost[1] + cost[2] + cost[3]
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cost({})\n", self.cost)?;
        write!(f, "#############\n")?;
        write!(f, "#")?;
        for c in self.hallway {
            if c == b' ' {
                write!(f, ".")?;
            } else {
                write!(f, "{}", c as char)?;
            }
        }
        write!(f, "#\n")?;
        write!(f, "###")?;
        for i in 0..4 {
            let c = self.abcd[i][N - 1];
            if c == b' ' {
                write!(f, ".")?;
            } else {
                write!(f, "{}", c as char)?;
            }
            write!(f, "#")?;
        }
        write!(f, "##\n")?;
        for j in 0..N - 1 {
            write!(f, "  #")?;
            for i in 0..4 {
                let c = self.abcd[i][N - 2 - j];
                if c == b' ' {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", c as char)?;
                }
                write!(f, "#")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  #########")
    }
}
