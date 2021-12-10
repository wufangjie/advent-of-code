use crate::read_lines;

pub fn part1() -> usize {
    let lines = read_lines("./data/day10.txt");

    let mut acc = 0;
    for line in lines {
        let mut stack = vec![];
        for c in line.bytes() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' | b']' | b'}' | b'>' => {
                    let last = stack.pop();
                    if last.is_none() || !is_match(last.unwrap(), c) {
                        acc += get_point(c);
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    acc
}

#[inline]
fn is_match(c1: u8, c2: u8) -> bool {
    match c1 {
        b'(' => c2 == b')',
        b'[' => c2 == b']',
        b'{' => c2 == b'}',
        b'<' => c2 == b'>',
        _ => unreachable!(),
    }
}

#[inline]
fn get_point(c: u8) -> usize {
    match c {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!(),
    }
}

#[inline]
fn get_point_2(c: u8) -> usize {
    match c {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => unreachable!(),
    }
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day10.txt");

    let mut scores = vec![];
    'main: for line in lines {
        let mut stack = vec![];
        for c in line.bytes() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' | b']' | b'}' | b'>' => {
                    let last = stack.pop();
                    if last.is_none() || !is_match(last.unwrap(), c) {
                        continue 'main;
                    }
                }
                _ => unreachable!(),
            }
        }
        let mut score = 0;
        while let Some(c) = stack.pop() {
            score *= 5;
            score += get_point_2(c);
        }
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() >> 1]
}
