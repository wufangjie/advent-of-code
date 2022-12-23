use crate::read_lines;

#[inline]
fn calc_shape_score(c: char) -> usize {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => unreachable!(),
    }
}

fn calc_round_score(c1: char, c2: char) -> usize {
    match c2 as u8 - c1 as u8 {
        23 => 3,
        24 | 21 => 6,
        _ => 0,
    }
}

pub fn part1() -> usize {
    let mut res = 0;
    for line in read_lines("./data/day02.txt") {
        let c1 = line.as_bytes()[0] as char;
        let c2 = line.as_bytes()[2] as char;
        res += calc_shape_score(c2) + calc_round_score(c1, c2);
    }
    res
}

fn calc_shape_score_2(c1: char, c2: char) -> usize {
    let s = calc_shape_score(c1);
    match c2 {
        'X' => {
            if s > 1 {
                s - 1
            } else {
                3
            }
        }
        'Y' => s,
        'Z' => {
            if s < 3 {
                s + 1
            } else {
                1
            }
        }
        _ => unreachable!(),
    }
}

pub fn part2() -> usize {
    let mut res = 0;
    for line in read_lines("./data/day02.txt") {
        let c1 = line.as_bytes()[0] as char;
        let c2 = line.as_bytes()[2] as char;
        let round_score = match c2 {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => unreachable!(),
        };
        res += round_score + calc_shape_score_2(c1, c2);
    }
    res
}

#[test]
fn test_02() {
    assert_eq!(15691, part1());
    assert_eq!(12989, part2());
}
