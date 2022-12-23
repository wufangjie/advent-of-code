use crate::read_lines;
use regex::Regex;

const N: usize = 1000;

pub fn part1() -> usize {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let lines = read_lines("./data/day05.txt");

    let mut count = [[0u16; N]; N];
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let xyxy: Vec<i16> = (1..5)
            .into_iter()
            .map(|i| caps.get(i).unwrap().as_str().parse().unwrap())
            .collect();
        if xyxy[0] == xyxy[2] || xyxy[1] == xyxy[3] {
            counting(&mut count, xyxy);
        }
    }
    count_by(&count, |x| x > 1)
}

fn counting(count: &mut [[u16; N]; N], xyxy: Vec<i16>) {
    let mut x0 = xyxy[0];
    let mut y0 = xyxy[1];
    let dx = match xyxy[2] - x0 {
        0 => 0,
        x => x / x.abs(),
    };
    let dy = match xyxy[3] - y0 {
        0 => 0,
        y => y / y.abs(),
    };
    for _ in 0..=(xyxy[3] - y0).abs().max((xyxy[2] - x0).abs()) {
        count[y0 as usize][x0 as usize] += 1;
        x0 += dx;
        y0 += dy;
    }
}

fn count_by(count: &[[u16; N]; N], filter: impl Fn(u16) -> bool) -> usize {
    count
        .iter()
        .map(|row| row.iter().filter(|&&item| filter(item)).count())
        .sum()
}

pub fn part2() -> usize {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let lines = read_lines("./data/day05.txt");

    let mut count = [[0u16; N]; N];
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let xyxy: Vec<i16> = (1..5)
            .into_iter()
            .map(|i| caps.get(i).unwrap().as_str().parse().unwrap())
            .collect();
        counting(&mut count, xyxy);
    }
    count_by(&count, |x| x > 1)
}

#[test]
fn test_05() {
    assert_eq!(6687, part1());
    assert_eq!(19851, part2());
}
