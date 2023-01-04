use crate::read_lines;
use std::ptr;

#[derive(Debug)]
struct Dir {
    name: String,
    direct_sizes: usize,
    sub_dirs: Vec<Dir>,
    parent: *mut Dir,
}

impl Dir {
    fn new(name: &str, parent: *mut Dir) -> Self {
        Self {
            name: name.to_string(),
            direct_sizes: 0,
            sub_dirs: vec![],
            parent,
        }
    }

    fn calc_size(&self) -> usize {
        self.direct_sizes + self.sub_dirs.iter().map(|x| x.calc_size()).sum::<usize>()
    }

    /// SAFETY: since mutable
    fn cd_parent(&mut self) -> &mut Self {
        debug_assert!(!self.parent.is_null());
        unsafe { &mut *self.parent }
    }

    fn cd_in(&mut self, dirname: &str) -> &mut Self {
        let n = self.sub_dirs.len();
        for i in 0..n {
            if self.sub_dirs[i].name == dirname {
                return &mut self.sub_dirs[i];
            }
        }
        self
    }

    fn sum_if_size_less_than(&self, thres: usize) -> usize {
        let mut good_lst = vec![];
        self._sum_if_size_less_than(thres, &mut good_lst);
        good_lst.into_iter().sum()
    }

    fn _sum_if_size_less_than(&self, thres: usize, good_lst: &mut Vec<usize>) -> usize {
        let mut acc = self.direct_sizes;
        for d in &self.sub_dirs {
            let size = d._sum_if_size_less_than(thres, good_lst);
            acc += size;
        }
        if acc < thres {
            good_lst.push(acc); // since we finally will sum them, we can use `+` instead of `push`
        }
        acc
    }

    fn find_if_size_greater_than(&self, thres: usize) -> usize {
        let mut good_lst = vec![];
        self._find_if_size_greater_than(thres, &mut good_lst);
        good_lst.into_iter().min().unwrap()
    }

    fn _find_if_size_greater_than(&self, thres: usize, good_lst: &mut Vec<usize>) -> usize {
        let mut acc = self.direct_sizes;
        for d in &self.sub_dirs {
            let size = d._find_if_size_greater_than(thres, good_lst);
            acc += size;
        }
        if acc >= thres {
            good_lst.push(acc); // since we finally will choose the min of them, we can use `min` instead of `push` here
        }
        acc
    }
}

pub fn part1() -> usize {
    let mut root = Dir::new("/", ptr::null_mut());
    let mut cur = &mut root;
    for line in read_lines("./data/day07.txt").skip(1) {
        if line == "$ ls" {
            // do nothing
        } else if line.starts_with("$ cd") {
            match &line[5..] {
                ".." => cur = cur.cd_parent(),
                "/" => cur = &mut root,
                _ => cur = cur.cd_in(&line[5..]),
            }
        } else if let Some(dirname) = line.strip_prefix("dir ") {
            let d = Dir::new(dirname, cur as *mut Dir);
            cur.sub_dirs.push(d);
        } else {
            cur.direct_sizes += line.split(' ').next().unwrap().parse::<usize>().unwrap();
        }
    }
    // dbg!(&root);
    // dbg!(root.calc_size());
    root.sum_if_size_less_than(100000)
}

pub fn part2() -> usize {
    let mut root = Dir::new("/", ptr::null_mut());
    let mut cur = &mut root;
    for line in read_lines("./data/day07.txt").skip(1) {
        if line == "$ ls" {
            // do nothing
        } else if line.starts_with("$ cd") {
            match &line[5..] {
                ".." => cur = cur.cd_parent(),
                "/" => cur = &mut root,
                _ => cur = cur.cd_in(&line[5..]),
            }
        } else if let Some(dirname) = line.strip_prefix("dir ") {
            let d = Dir::new(dirname, cur as *mut Dir);
            cur.sub_dirs.push(d);
        } else {
            cur.direct_sizes += line.split(' ').next().unwrap().parse::<usize>().unwrap();
        }
    }
    let current_size = 70000000 - root.calc_size();
    let thres = 30000000 - current_size;
    root.find_if_size_greater_than(thres) // + current_size
}

#[test]
fn test_07() {
    assert_eq!(2104783, part1());
    assert_eq!(5883165, part2());
}
