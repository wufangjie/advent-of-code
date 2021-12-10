use crate::read_lines;
use std::collections::HashSet;

pub fn part1() -> usize {
    get_origin_black().len()
}

fn get_origin_black() -> HashSet<(i64, i64)> {
    let lines = read_lines("./data/day24.txt");
    let mut black = HashSet::new();
    for line in lines {
        let mut x = 0;
        let mut y = 0;
        let mut pre = b'e';
        for c in line.bytes() {
            match c {
                b'e' => match pre {
                    b's' | b'n' => x += 1,
                    _ => x += 2,
                },
                b'w' => match pre {
                    b's' | b'n' => x -= 1,
                    _ => x -= 2,
                },
                b's' => y -= 1,
                b'n' => y += 1,
                _ => unreachable!(),
            }
            pre = c;
        }
        if black.contains(&(x, y)) {
            black.remove(&(x, y));
        } else {
            black.insert((x, y));
        }
    }
    black
}

pub fn part2() -> usize {
    let mut black = get_origin_black();
    let mut to_remove = vec![];

    for _ in 0..100 {
        to_remove.clear();
        let mut to_add = vec![];
        let mut white = HashSet::new();

        let black_lst: Vec<(i64, i64)> = black.iter().cloned().collect();

        for (x, y) in black_lst {
            let mut count_adj_black = 0;
            for (i, j) in [
                (x - 2, y),
                (x + 2, y),
                (x - 1, y + 1),
                (x + 1, y + 1),
                (x - 1, y - 1),
                (x + 1, y - 1),
            ] {
                if black.contains(&(i, j)) {
                    count_adj_black += 1;
                } else {
                    white.insert((i, j));
                }
            }
            if count_adj_black == 0 || count_adj_black > 2 {
                to_remove.push((x, y));
            }
        }

        for (x, y) in white {
            let mut count_adj_black = 0;
            for (i, j) in [
                (x - 2, y),
                (x + 2, y),
                (x - 1, y + 1),
                (x + 1, y + 1),
                (x - 1, y - 1),
                (x + 1, y - 1),
            ] {
                if black.contains(&(i, j)) {
                    count_adj_black += 1;
                    if count_adj_black > 2 {
                        break;
                    }
                }
            }
            if count_adj_black == 2 {
                to_add.push((x, y));
            }
        }

        for xy in &to_remove {
            black.remove(xy);
        }
        for xy in to_add {
            black.insert(xy);
        }
    }
    black.len()
}
