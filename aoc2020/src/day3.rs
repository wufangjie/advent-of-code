use crate::read_lines;

pub fn part1() -> usize {
    let lines = read_lines("./data/day3.txt");
    count_tree(&lines, 3, 1)
}

fn count_tree(lines: &Vec<String>, dx: usize, dy: usize) -> usize {
    let nrow = lines.len();
    let ncol = lines[0].len();
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;
    while i < nrow {
        if lines[i].bytes().nth(j).unwrap() == b'#' {
            count += 1;
        }
        j += dx;
        j %= ncol;
        i += dy;
    }
    count
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day3.txt");
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| count_tree(&lines, dx, dy))
        .product::<usize>()
}
