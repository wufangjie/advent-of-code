use crate::read_lines;

pub fn part1() -> usize {
    let mut lines = read_lines("./data/day5.txt");
    lines.iter().map(|x| calc_index(x)).max().unwrap()
}

fn calc_index(line: &str) -> usize {
    let mut index = 0;
    for c in line.chars() {
        index <<= 1;
        match c {
            'B' | 'R' => index |= 1,
            _ => (),
        }
    }
    index
}

#[test]
fn test_5() {
    assert_eq!(567, calc_index("BFFFBBFRRR"));
    assert_eq!(119, calc_index("FFFBBBFRRR"));
    assert_eq!(820, calc_index("BBFFBBFRLL"));
}

pub fn part2() -> usize {
    let mut lines = read_lines("./data/day5.txt");
    let mut idxs: Vec<usize> = lines.iter().map(|x| calc_index(x)).collect();
    idxs.sort();
    for i in 0..idxs.len() - 1 {
        if idxs[i + 1] - idxs[i] > 1 {
            return idxs[i] + 1;
        }
    }
    unreachable!();
}
