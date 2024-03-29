use crate::read_lines;
use regex::Regex;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashSet;

pub fn part1() -> usize {
    let mut iter = read_lines("./data/day13.txt").into_iter();
    let mut dots = HashSet::new();
    for line in iter.by_ref().take_while(|x| !x.is_empty()) {
        let xy: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
        dots.insert((xy[0], xy[1]));
    }

    let re_fold = Regex::new(r"^fold along ([xy])=(\d+)$").unwrap();
    if let Some(line) = iter.next() {
        let mut dots2 = HashSet::new();
        let caps = re_fold.captures(&line).unwrap();
        let num: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        match caps.get(1).unwrap().as_str() {
            "x" => {
                for &(x, y) in &dots {
                    match x.cmp(&num) {
                        Greater => dots2.insert((2 * num - x, y)),
                        Less => dots2.insert((x, y)),
                        _ => false,
                    };
                    // if x > num {
                    //     dots2.insert((2 * num - x, y));
                    // } else if x < num {
                    //     dots2.insert((x, y));
                    // }
                }
            }
            "y" => {
                for &(x, y) in &dots {
                    match y.cmp(&num) {
                        Greater => dots2.insert((x, 2 * num - y)),
                        Less => dots2.insert((x, y)),
                        _ => false,
                    };
                    // if y > num {
                    //     dots2.insert((x, 2 * num - y));
                    // } else if y < num {
                    //     dots2.insert((x, y));
                    // }
                }
            }
            _ => unreachable!(),
        }
        dots = dots2;
    }
    dots.len()
}

pub fn part2() {
    let mut iter = read_lines("./data/day13.txt").into_iter();
    let mut dots = HashSet::new();
    for line in iter.by_ref().take_while(|x| !x.is_empty()) {
        let xy: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
        dots.insert((xy[0], xy[1]));
    }

    let re_fold = Regex::new(r"^fold along ([xy])=(\d+)$").unwrap();
    for line in iter {
        let mut dots2 = HashSet::new();
        let caps = re_fold.captures(&line).unwrap();
        let num: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        match caps.get(1).unwrap().as_str() {
            "x" => {
                for &(x, y) in &dots {
                    match x.cmp(&num) {
                        Greater => dots2.insert((2 * num - x, y)),
                        Less => dots2.insert((x, y)),
                        _ => false,
                    };
                    //     if x > num {
                    //         dots2.insert((2 * num - x, y));
                    //     } else if x < num {
                    //         dots2.insert((x, y));
                    //     }
                }
            }
            "y" => {
                for &(x, y) in &dots {
                    match y.cmp(&num) {
                        Greater => dots2.insert((x, 2 * num - y)),
                        Less => dots2.insert((x, y)),
                        _ => false,
                    };
                    // if y > num {
                    //     dots2.insert((x, 2 * num - y));
                    // } else if y < num {
                    //     dots2.insert((x, y));
                    // }
                }
            }
            _ => unreachable!(),
        }
        dots = dots2;
    }

    // print
    let mut xmax = 0;
    let mut ymax = 0;
    for &(x, y) in &dots {
        xmax = xmax.max(x);
        ymax = ymax.max(y)
    }
    let mut to_print = vec![vec![false; xmax + 1]; ymax + 1];
    for (x, y) in dots {
        to_print[y][x] = true;
    }
    for row in to_print.iter() {
        for &item in row.iter() {
            print!("{}", if item { "#" } else { " " });
        }
        // for j in 0..=ymax {
        //     for i in 0..=xmax {
        //         print!("{}", if to_print[j][i] { "#" } else { " " });
        //     }
        println!();
    }
    println!();
}

#[test]
fn test_13() {
    assert_eq!(827, part1());
    dbg!(part2());
}
