use crate::read_lines;
use std::collections::HashSet; //{HashMap, HashSet, VecDeque};
use utils::Heap;

const FILENAME: &str = "./data/day24.txt";

pub fn part1() -> usize {
    let mut dct: HashSet<(usize, usize, u8)> = HashSet::new();
    let mut lst = vec![]; // u8 is enough, at most 4
    for (i, line) in read_lines(FILENAME).enumerate() {
        lst.push(
            line.bytes()
                .map(|c| u8::from(c != b'.'))
                .collect::<Vec<u8>>(),
        );
        for (j, c) in line.bytes().enumerate() {
            if c != b'#' && c != b'.' {
                dct.insert((i, j, c));
            }
        }
    }
    let nrow = lst.len();
    let ncol = lst[0].len();
    // debug_assert!(lst[0][1] == 0);
    // debug_assert!(lst[nrow - 1][ncol - 2] == 0);

    one_pass(nrow - 1, ncol - 2, 0, 1, 0, &dct)
}

fn one_pass(
    yy: usize,
    xx: usize,
    i: usize,
    j: usize,
    step0: usize,
    dct: &HashSet<(usize, usize, u8)>,
) -> usize {
    //let (yy, xx) = (nrow - 1, ncol - 2); // target point
    let mut snap_lst = vec![]; //lst];
    let nrow = yy.max(i) + 1;
    let ncol = xx.max(j) + 2;

    let mut heap: Heap<(usize, usize, usize, usize)> = Heap::new();
    if yy > i {
        heap.push((yy - i + xx - j, i, j, step0)); // start point: E
    } else {
        heap.push((i - yy + j - xx, i, j, step0)); // start point: E
    }

    let mut seen = HashSet::new(); // NOTE: important

    while let Some((h, i, j, step)) = heap.pop() {
        if seen.contains(&(i, j, step)) {
            continue;
        } else {
            seen.insert((i, j, step));
        }
        if step - step0 == snap_lst.len() {
            // println!("{}", step);
            snap_lst.push(add_snap(&dct, step + 1, nrow, ncol));
            // print_snap(&snap_lst[step]);
        }

        if i == 0 {
            heap.push((h + 1, 0, 1, step + 1)); // just wait
            if snap_lst[step - step0][1][1] == 0 {
                heap.push((h, 1, 1, step + 1));
            }
        } else if i == nrow - 1 {
            heap.push((h + 1, nrow - 2, ncol - 2, step + 1)); // just wait
            if snap_lst[step - step0][nrow - 2][ncol - 2] == 0 {
                heap.push((h, nrow - 2, ncol - 2, step + 1));
            }
        } else {
            for (ii, jj) in [(i, j), (i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if ii == yy && jj == xx {
                    return step + 1;
                }
                if ii == 0
                    || jj == 0
                    || ii == yy
                    || jj == ncol - 1
                    || snap_lst[step - step0][ii][jj] != 0
                {
                    continue;
                }
                if yy > ii {
                    heap.push((yy - ii + xx - jj + step + 1, ii, jj, step + 1));
                } else {
                    heap.push((ii - yy + jj - xx + step + 1, ii, jj, step + 1));
                }
            }
        }
    }
    unreachable!()
}

fn add_snap(
    dct: &HashSet<(usize, usize, u8)>,
    step: usize,
    nrow: usize,
    ncol: usize,
) -> Vec<Vec<u8>> {
    let mut lst = vec![vec![0; ncol]; nrow];
    for &(i, j, f) in dct {
        let (i2, j2) = match f {
            b'<' => (i, (j - 1 + ncol - 2 - step % (ncol - 2)) % (ncol - 2) + 1),
            b'>' => (i, (j - 1 + step) % (ncol - 2) + 1),
            b'v' => ((i - 1 + step) % (nrow - 2) + 1, j),
            b'^' => ((i - 1 + nrow - 2 - step % (nrow - 2)) % (nrow - 2) + 1, j),
            _ => unreachable!(),
        };
        lst[i2][j2] += 1;
    }
    lst
}

fn print_snap(lst: &[Vec<u8>]) {
    for line in lst {
        for c in line {
            if *c > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part2() -> usize {
    let mut dct: HashSet<(usize, usize, u8)> = HashSet::new();
    let mut lst = vec![]; // u8 is enough, at most 4
    for (i, line) in read_lines(FILENAME).enumerate() {
        lst.push(
            line.bytes()
                .map(|c| u8::from(c != b'.'))
                .collect::<Vec<u8>>(),
        );
        for (j, c) in line.bytes().enumerate() {
            if c != b'#' && c != b'.' {
                dct.insert((i, j, c));
            }
        }
    }
    let nrow = lst.len();
    let ncol = lst[0].len();
    // debug_assert!(lst[0][1] == 0);
    // debug_assert!(lst[nrow - 1][ncol - 2] == 0);

    let mut step = one_pass(nrow - 1, ncol - 2, 0, 1, 0, &dct);
    step = one_pass(0, 1, nrow - 1, ncol - 2, step, &dct);
    one_pass(nrow - 1, ncol - 2, 0, 1, step, &dct)
}

#[test]
fn test_24() {
    assert_eq!(314, part1()); // release mode
    assert_eq!(896, part2()); // release mode 0.80s
}
