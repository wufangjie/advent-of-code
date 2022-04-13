use std::collections::HashSet;
use std::ptr;

pub fn part1() -> usize {
    //let mut cups = Cups::new(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    let mut cups = Cups::new(vec![7, 8, 4, 2, 3, 5, 9, 1, 6]);
    for _ in 0..100 {
        cups.next_move();
    }
    cups.calc_res()
}

#[derive(Debug)]
struct Cups {
    data: Vec<usize>,
    n: usize,
    destination: usize,
}

impl Cups {
    fn new(data: Vec<usize>) -> Self {
        let n = data.len();
        Self {
            data,
            n,
            destination: 0,
        }
    }

    fn find_destination(&mut self) {
        let mut v_max_less = 0;
        let mut v_max_all = 0;
        let mut i_max_less = 0;
        let mut i_max_all = 0;
        for i in 4..self.n {
            if self.data[i] < self.data[0] {
                if self.data[i] > v_max_less {
                    v_max_less = self.data[i];
                    i_max_less = i;
                }
            } else if self.data[i] > v_max_all {
                v_max_all = self.data[i];
                i_max_all = i;
            }
        }
        if v_max_less == 0 {
            self.destination = i_max_all;
        } else {
            self.destination = i_max_less;
        }
    }

    fn place_and_next(&mut self) {
        let mut new = vec![];
        for i in 4..self.n {
            new.push(self.data[i]);
            if i == self.destination {
                for j in 1..4 {
                    new.push(self.data[j]);
                }
            }
        }
        new.push(self.data[0]);
        self.data = new;
    }

    fn next_move(&mut self) {
        self.find_destination();
        self.place_and_next();
    }

    fn calc_res(&self) -> usize {
        self.data
            .iter()
            .chain(self.data.iter())
            .skip_while(|&&x| x != 1)
            .skip(1)
            .take(self.n - 1)
            .fold(0, |acc, b| acc * 10 + b)
    }
}

const N: usize = 1000000;
const M: usize = 10000000;

pub fn part2() -> usize {
    let mut cups = Vec::with_capacity(N);
    for i in 1..=N {
        cups.push(Cup::new(i));
    }
    for i in 9..N - 1 {
        cups[i].next = &mut cups[i + 1] as *mut Cup;
    }

    let left = [N, 7, 8, 4, 2, 3, 5, 9, 1, 6, 10];
    //let left = [N, 3, 8, 9, 1, 2, 5, 4, 6, 7, 10];
    for i in 0..left.len() - 1 {
        cups[left[i] - 1].next = &mut cups[left[i + 1] - 1] as *mut Cup;
    }

    let mut cur = left[1] - 1;
    for _ in 0..M {
        let (p3, picked) = cups[cur].remove_next_3();
        let mut i = cur;
        loop {
            if i == 0 {
                i = N;
            }
            i -= 1;
            if !picked.contains(&i) {
                cups[i].insert_3(p3);
                break;
            }
        }
        cur = cups[cur].find_next_cur();
    }

    unsafe { (*cups[0].next).label * (*(*cups[0].next).next).label }
}

struct Cup {
    label: usize,
    next: *mut Cup,
}

impl Cup {
    fn new(label: usize) -> Self {
        Self {
            label,
            next: ptr::null_mut(),
        }
    }

    fn remove_next_3(&mut self) -> (*mut Self, HashSet<usize>) {
        let mut picked = HashSet::new();
        let ret = self.next;
        unsafe {
            let mut next = self.next;
            for _ in 0..3 {
                picked.insert((*next).label - 1);
                next = (*next).next;
            }
            self.next = next;
        }
        (ret, picked)
    }

    fn insert_3(&mut self, p: *mut Self) {
        unsafe {
            (*(*(*p).next).next).next = self.next;
            self.next = p;
        }
    }

    fn find_next_cur(&self) -> usize {
        unsafe { (*self.next).label - 1 }
    }

    /// just for debug
    #[allow(dead_code)]
    fn print_next_n(&self, n: usize) {
        let mut p = self;
        unsafe {
            for _ in 0..n {
                print!("{} -> ", p.label);
                p = &*p.next;
            }
        }
        println!();
    }
}

#[test]
fn test_23() {
    assert_eq!(53248976, part1());
    assert_eq!(418819514477, part2());
}
