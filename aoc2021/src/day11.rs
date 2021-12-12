use crate::read_lines;

pub fn part1() -> usize {
    let mut lines: Vec<Vec<u8>> = read_lines("./data/day11.txt")
        .into_iter()
        .map(|row| row.bytes().map(|x| x - b'0').collect())
        .collect();
    let nrow = lines.len();
    let ncol = lines[0].len();
    let mut count = 0;

    for step in 0..100 {
        let mut stack = vec![];
        for i in 0..nrow {
            for j in 0..ncol {
                if lines[i][j] == 10 {
                    lines[i][j] = 1;
                } else {
                    lines[i][j] += 1;
                    if lines[i][j] == 10 {
                        count += 1;
                        stack.push((i, j));
                    }
                }
            }
        }
        while let Some((i, j)) = stack.pop() {
            // assert nrow, ncol >> 2
            let range_i = if i == 0 {
                i..=i + 1
            } else if i == nrow - 1 {
                i - 1..=i
            } else {
                i - 1..=i + 1
            };
            let range_j = if j == 0 {
                j..=j + 1
            } else if j == ncol - 1 {
                j - 1..=j
            } else {
                j - 1..=j + 1
            };

            for ii in range_i {
                for jj in range_j.clone() {
                    if lines[ii][jj] != 10 {
                        lines[ii][jj] += 1;
                        if lines[ii][jj] == 10 {
                            count += 1;
                            stack.push((ii, jj));
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn part2() -> usize {
    let mut lines: Vec<Vec<u8>> = read_lines("./data/day11.txt")
        .into_iter()
        .map(|row| row.bytes().map(|x| x - b'0').collect())
        .collect();
    let nrow = lines.len();
    let ncol = lines[0].len();
    let mut step = 0;
    loop {
        step += 1;
        let mut step_flash_count = 0;
        let mut stack = vec![];
        for i in 0..nrow {
            for j in 0..ncol {
                if lines[i][j] == 10 {
                    lines[i][j] = 1;
                } else {
                    lines[i][j] += 1;
                    if lines[i][j] == 10 {
                        step_flash_count += 1;
                        stack.push((i, j));
                    }
                }
            }
        }
        while let Some((i, j)) = stack.pop() {
            // assert nrow, ncol >> 2
            let range_i = if i == 0 {
                i..=i + 1
            } else if i == nrow - 1 {
                i - 1..=i
            } else {
                i - 1..=i + 1
            };
            let range_j = if j == 0 {
                j..=j + 1
            } else if j == ncol - 1 {
                j - 1..=j
            } else {
                j - 1..=j + 1
            };

            for ii in range_i {
                for jj in range_j.clone() {
                    if lines[ii][jj] != 10 {
                        lines[ii][jj] += 1;
                        if lines[ii][jj] == 10 {
                            step_flash_count += 1;
                            stack.push((ii, jj));
                        }
                    }
                }
            }
        }
        if step_flash_count == nrow * ncol {
            return step;
        }
    }
}
