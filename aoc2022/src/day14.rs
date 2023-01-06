use crate::read_lines;
use std::collections::HashSet;

fn preprocess() -> (HashSet<(i32, i32)>, i32) {
    let mut filled = HashSet::new();
    let mut max_y = 0;
    for line in read_lines("./data/day14.txt") {
        let mut iter = line.split(" -> ");
        let temp = iter.next().unwrap().split_once(',').unwrap();
        let mut x1: i32 = temp.0.parse().unwrap();
        let mut y1: i32 = temp.1.parse().unwrap();
        max_y = max_y.max(y1);
        for pair in iter {
            let temp = pair.split_once(',').unwrap();
            let x2: i32 = temp.0.parse().unwrap();
            let y2: i32 = temp.1.parse().unwrap();
            max_y = max_y.max(y2);
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    filled.insert((x, y));
                }
            }
            x1 = x2;
            y1 = y2;
        }
    }
    (filled, max_y)
}

pub fn part1() -> usize {
    let (mut filled, max_y) = preprocess();
    let mut count = 0;
    loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y == max_y {
                return count;
            }
            y += 1;

            let mut moved = false;
            for xx in [x, x - 1, x + 1] {
                if !filled.contains(&(xx, y)) {
                    x = xx;
                    moved = true;
                    break;
                }
            }
            if !moved {
                filled.insert((x, y - 1)); // fixed
                break;
            }
        }
        count += 1;
    }
}

pub fn part2() -> usize {
    let (mut filled, mut max_y) = preprocess();
    max_y += 1;
    let mut count = 0;
    loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y == max_y {
                filled.insert((x, y)); // fixed
                break;
            }
            y += 1;

            let mut moved = false;
            for xx in [x, x - 1, x + 1] {
                if !filled.contains(&(xx, y)) {
                    x = xx;
                    moved = true;
                    break;
                }
            }
            if !moved {
                if y == 1 {
                    return count + 1;
                }
                filled.insert((x, y - 1)); // fixed
                break;
            }
        }
        count += 1;
    }
}

#[test]
fn test_14() {
    assert_eq!(1133, part1());
    assert_eq!(27566, part2());
}
