use crate::read_lines;

pub fn part1() -> usize {
    let lines = read_lines("./data/day5.txt");
    lines.iter().map(|x| calc_index(x)).max().unwrap()
}

fn calc_index(line: &str) -> usize {
    let mut index = 0;
    for c in line.as_bytes() {
        index <<= 1;
        match *c {
            b'B' | b'R' => index |= 1,
            _ => (),
        }
    }
    index
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day5.txt");
    let mut idxs: Vec<usize> = lines.iter().map(|x| calc_index(x)).collect();
    idxs.sort_unstable();
    for i in 0..idxs.len() - 1 {
        if idxs[i + 1] - idxs[i] > 1 {
            return idxs[i] + 1;
        }
    }
    unreachable!();
}

#[test]
fn test_05() {
    assert_eq!(567, calc_index("BFFFBBFRRR"));
    assert_eq!(119, calc_index("FFFBBBFRRR"));
    assert_eq!(820, calc_index("BBFFBBFRLL"));

    assert_eq!(842, part1());
    assert_eq!(617, part2());
}
