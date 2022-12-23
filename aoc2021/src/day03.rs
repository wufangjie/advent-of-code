use crate::read_lines;

pub fn part1() -> usize {
    let lines = read_lines("./data/day3.txt");
    let nrow = lines.len();
    let ncol = lines[0].len();

    let mut count = vec![0; ncol];
    for line in lines {
        for (j, c) in line.chars().enumerate() {
            if c == '1' {
                count[j] += 1;
            }
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mid = nrow >> 1;
    for v in count {
        x <<= 1;
        y <<= 1;
        if v > mid {
            x += 1;
        } else {
            y += 1;
        }
    }
    x * y
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day3.txt");
    let ncol = lines[0].len();

    let mut trie = Trie::new();
    for line in lines {
        trie.insert(line.as_bytes());
    }

    let x = calc_most_common(&trie, ncol);
    let y = calc_least_common(&trie, ncol);
    x * y
}

fn calc_most_common(trie: &Trie, count: usize) -> usize {
    let mut acc = 0;
    let mut node = trie;
    for _ in 0..count {
        acc <<= 1;
        if node.count[1] >= node.count[0] {
            acc += 1;
            node = &node.child[1].as_ref().unwrap();
        } else {
            node = &node.child[0].as_ref().unwrap();
        }
    }
    acc
}

fn calc_least_common(trie: &Trie, count: usize) -> usize {
    let mut acc = 0;
    let mut node = trie;
    for _ in 0..count {
        acc <<= 1;
        let mut j = if node.count[1] < node.count[0] { 1 } else { 0 };
        if node.count[j] == 0 {
            j ^= 1;
        }
        acc += j;
        node = &node.child[j].as_ref().unwrap();
    }
    acc
}

#[derive(Debug)]
struct Trie {
    child: [Option<Box<Trie>>; 2],
    count: [usize; 2],
}

impl Trie {
    fn new() -> Self {
        Trie {
            child: [None, None],
            count: [0, 0],
        }
    }

    fn insert(&mut self, seq: &[u8]) {
        let mut node = self;
        for c in seq {
            let j = (c - b'0') as usize;
            node.count[j] += 1;
            if node.child[j].is_none() {
                node.child[j] = Some(Box::new(Trie::new()));
            }
            node = node.child[j].as_mut().unwrap();
        }
    }
}
