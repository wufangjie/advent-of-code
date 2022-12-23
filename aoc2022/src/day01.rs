use crate::read_lines;

pub fn part1() -> i64 {
    let mut acc = 0;
    let mut max = 0;
    for line in read_lines("./data/day01.txt") {
        if line.is_empty() {
            if acc > max {
                max = acc;
            }
            acc = 0;
        } else {
            acc += line.parse::<i64>().unwrap();
        }
    }
    max.max(acc)
}

const TOP_N: usize = 3;

pub fn part2() -> i64 {
    let mut h = utils::Heap::with_capacity(TOP_N);
    let mut acc = 0;
    for line in read_lines("./data/day01.txt") {
        if line == "" {
            if h.is_empty() {
                h.push(acc);
            } else if h.len() < TOP_N {
                h.push(acc);
            } else if h.peek().unwrap() < &acc {
                h.pushpop(acc);
            }
            acc = 0;
        } else {
            acc += line.parse::<i64>().unwrap();
        }
    }
    h.into_inner().into_iter().sum::<i64>()
}

#[test]
fn test_01() {
    assert_eq!(69289, part1());
    assert_eq!(205615, part2());
}
