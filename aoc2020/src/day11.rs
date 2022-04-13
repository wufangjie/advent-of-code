use crate::read_lines;

pub fn part1() -> usize {
    let lines = read_lines("./data/day11.txt");
    let nrow = lines.len();
    let ncol = lines[0].len();

    let mut layout: Vec<Vec<i8>> = vec![vec![-1i8; ncol + 2]; nrow + 2];
    for i in 0..nrow {
        for (j, c) in lines[i].bytes().into_iter().enumerate() {
            match c {
                b'L' => layout[i + 1][j + 1] = 0,
                b'#' => layout[i + 1][j + 1] = 1,
                _ => (),
            }
        }
    }

    let mut change: Vec<(usize, usize)> = vec![];
    let mut changing = true;
    while changing {
        while let Some((i, j)) = change.pop() {
            layout[i][j] ^= 1i8;
        }
        for i in 1..nrow + 1 {
            for j in 1..ncol + 1 {
                match layout[i][j] {
                    0 => {
                        if count_adj(&layout, i, j) == 0 {
                            change.push((i, j));
                        }
                    }
                    1 => {
                        if count_adj(&layout, i, j) > 3 {
                            change.push((i, j));
                        }
                    }
                    _ => (),
                }
            }
        }
        changing = !change.is_empty();
    }

    layout
        .iter()
        .map(|line| line.iter().filter(|&&x| x == 1).count())
        .sum()
}

#[inline]
fn count_adj(layout: &[Vec<i8>], i: usize, j: usize) -> usize {
    [
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
    .into_iter()
    .filter(|&(x, y)| layout[x][y] == 1)
    .count()
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day11.txt");
    let nrow = lines.len();
    let ncol = lines[0].len();

    let mut layout: Vec<Vec<i8>> = vec![vec![-1i8; ncol]; nrow];
    for i in 0..nrow {
        for (j, c) in lines[i].bytes().into_iter().enumerate() {
            match c {
                b'L' => layout[i][j] = 0,
                b'#' => layout[i][j] = 1,
                _ => (),
            }
        }
    }

    let mut change: Vec<(usize, usize)> = vec![];
    let mut changing = true;
    while changing {
        while let Some((i, j)) = change.pop() {
            layout[i][j] ^= 1i8;
        }
        for i in 0..nrow {
            for j in 0..ncol {
                match layout[i][j] {
                    0 => {
                        if count_first_look(&layout, i, j) == 0 {
                            change.push((i, j));
                        }
                    }
                    1 => {
                        if count_first_look(&layout, i, j) > 4 {
                            change.push((i, j));
                        }
                    }
                    _ => (),
                }
            }
        }
        changing = !change.is_empty();
        // pretty_print(&layout);
    }

    layout
        .iter()
        .map(|line| line.iter().filter(|&&x| x == 1).count())
        .sum()
}

fn count_first_look(layout: &[Vec<i8>], i: usize, j: usize) -> usize {
    let nrow = layout.len() as i32;
    let ncol = layout[0].len() as i32;

    let mut acc = 0;
    for (dx, dy) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let mut ii = i as i32;
        let mut jj = j as i32;
        loop {
            ii += dx;
            jj += dy;
            if ii >= 0 && ii < nrow && jj >= 0 && jj < ncol {
                match layout[ii as usize][jj as usize] {
                    1 => {
                        acc += 1;
                        break;
                    }
                    0 => break,
                    _ => (),
                }
            } else {
                break;
            }
        }
    }
    acc
}

#[allow(dead_code)]
fn pretty_print(layout: &[Vec<i8>]) {
    //let nrow = layout.len();
    //let ncol = layout[0].len();
    for row in layout.iter() {
        for item in row.iter() {
            print!(
                "{}",
                match *item {
                    1 => "#",
                    0 => "L",
                    _ => ".",
                }
            );
        }
        println!();
    }

    // for i in 0..nrow {
    //     for j in 0..ncol {
    //         print!(
    //             "{}",
    //             match layout[i][j] {
    //                 1 => "#",
    //                 0 => "L",
    //                 _ => ".",
    //             }
    //         );
    //     }
    //     println!();
    // }
    println!();
}

#[test]
fn test_11() {
    assert_eq!(2247, part1());
    assert_eq!(2011, part2());
}
