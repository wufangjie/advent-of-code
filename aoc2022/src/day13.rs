use crate::read_lines;
use std::cmp::Ord;
use std::cmp::Ordering::{self, Equal, Greater, Less};

#[derive(Debug, Clone, Copy)]
struct Lisp<'a> {
    raw: &'a str,
}

impl<'a> Lisp<'a> {
    fn new(raw: &'a str) -> Self {
        Self { raw }
    }

    /// just like flatten
    fn get_children(&self) -> Vec<Lisp<'a>> {
        if self.raw.starts_with('[') {
            let mut ret = vec![];
            let mut count = 0; // count unclosed [
            let mut i0 = 1;
            for (i, c) in self.raw.bytes().enumerate().skip(1) {
                match c {
                    b'[' => {
                        count += 1;
                    }
                    b']' => {
                        count -= 1;
                        if count < 0 {
                            // if and only if the last ]
                            ret.push(Lisp::new(&self.raw[i0..i]));
                        }
                    }
                    b',' => {
                        if count == 0 {
                            ret.push(Lisp::new(&self.raw[i0..i]));
                            i0 = i + 1;
                        }
                    }
                    _ => (),
                }
            }
            ret
        } else {
            vec![*self]
        }
    }

    #[inline]
    fn is_list(&self) -> bool {
        self.raw.starts_with('[')
    }

    /// NOTE: 空的判断必须放在最前面, 否则可能会把不该去的括号也去掉
    fn _cmp(&self, other: &Self) -> Ordering {
        if self.raw.is_empty() {
            if other.raw.is_empty() {
                Equal
            } else {
                Less
            }
        } else if other.raw.is_empty() {
            Greater
        } else if self.is_list() || other.is_list() {
            self._cmp_children(other)
        } else {
            self.raw
                .parse::<i32>()
                .unwrap()
                .cmp(&other.raw.parse::<i32>().unwrap())
        }
    }

    fn _cmp_children(&self, other: &Self) -> Ordering {
        let cl = self.get_children();
        let cr = other.get_children();
        for i in 0..cl.len().min(cr.len()) {
            match cl[i]._cmp(&cr[i]) {
                Equal => (),
                not_equal => return not_equal,
            }
        }
        cl.len().cmp(&cr.len())
    }
}

impl PartialEq for Lisp<'_> {
    fn eq(&self, other: &Self) -> bool {
        self._cmp(other) == Equal
    }
}

impl Eq for Lisp<'_> {}

impl PartialOrd for Lisp<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self._cmp(other))
    }
}

impl Ord for Lisp<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self._cmp(other)
    }
}

pub fn part1() -> usize {
    let lines = read_lines("./data/day13.txt").collect::<Vec<String>>();
    let mut iter = lines.iter();
    let mut res = 0;
    let mut count = 1;
    while let Some(left) = iter.next() {
        let cl = Lisp::new(left);
        let cr = Lisp::new(iter.next().unwrap());
        if let Less = cl.cmp(&cr) {
            res += count;
        }
        count += 1;
        iter.next(); // skip the blank line
    }
    res
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day13.txt").collect::<Vec<String>>();
    let mut iter = lines.iter();
    let mut lst = vec![Lisp::new("[[2]]"), Lisp::new("[[6]]")];
    while let Some(left) = iter.next() {
        lst.push(Lisp::new(left));
        lst.push(Lisp::new(iter.next().unwrap()));
        iter.next();
    }
    lst.sort_unstable();
    let mut res = 1;
    for (i, li) in lst.iter().enumerate() {
        if li.raw == "[[2]]" || li.raw == "[[6]]" {
            res *= i + 1;
            dbg!(li.raw, res); // suppose [[2]] and [[6]] are unqiue
        }
    }
    res
}

#[test]
fn test_13() {
    assert_eq!(5557, part1());
    assert_eq!(22425, part2());
}
