use crate::read_lines;
use regex::Regex;
use std::collections::HashSet;

const FILENAME: &str = "./data/day15.txt";

pub fn part1() -> i64 {
    let y0 = 200_0000;
    let mut impossible = vec![];
    let mut except = HashSet::new();
    let re =
        Regex::new(r"x=(?P<x1>-?\d+), y=(?P<y1>-?\d+).*?x=(?P<x2>-?\d+), y=(?P<y2>-?\d+)").unwrap();
    for line in read_lines(FILENAME) {
        let caps = re.captures(&line).unwrap();

        let x1 = caps["x1"].parse::<i64>().unwrap();
        let y1 = caps["y1"].parse::<i64>().unwrap();
        let x2 = caps["x2"].parse::<i64>().unwrap();
        let y2 = caps["y2"].parse::<i64>().unwrap();
        if y2 == y0 {
            except.insert(x2);
        }

        let dx = (x1 - x2).abs() + (y1 - y2).abs() - (y1 - y0).abs();
        if dx >= 0 {
            impossible.push((x1 - dx, x1 + dx));
        }
    }
    impossible.sort_unstable();
    let mut count = 0;
    let mut pre = i64::MIN;
    for (x1, x2) in impossible {
        if x2 >= pre {
            count += x2 - x1.max(pre) + 1;
            pre = x2 + 1;
        }
    }
    count - except.len() as i64
}

/// if there is only one distress bacon, it must at the edge of one of those diamond
pub fn part2() -> i64 {
    let n: i64 = 400_0000; //20; //
    let limit = get_limit();
    for (x1, y1, thres) in &limit {
        let rr = thres + 1;
        for dx in 0..rr {
            let dy = rr - dx;
            for (x, y) in [
                (x1 - dx, y1 - dy),
                (x1 - dx, y1 + dy),
                (x1 + dx, y1 - dy),
                (x1 + dx, y1 + dy),
            ] {
                if (0..=n).contains(&x) && (0..=n).contains(&y) && can_place(x, y, &limit) {
                    return x * n + y;
                }
            }
        }
    }
    unreachable!()
}

fn get_limit() -> Vec<(i64, i64, i64)> {
    let re =
        Regex::new(r"x=(?P<x1>-?\d+), y=(?P<y1>-?\d+).*?x=(?P<x2>-?\d+), y=(?P<y2>-?\d+)").unwrap();
    let mut res = vec![];
    for line in read_lines(FILENAME) {
        let caps = re.captures(&line).unwrap();
        let x1 = caps["x1"].parse::<i64>().unwrap();
        let y1 = caps["y1"].parse::<i64>().unwrap();
        let x2 = caps["x2"].parse::<i64>().unwrap();
        let y2 = caps["y2"].parse::<i64>().unwrap();
        res.push((x1, y1, (x1 - x2).abs() + (y1 - y2).abs()))
    }
    res
}

fn can_place(x0: i64, y0: i64, limit: &[(i64, i64, i64)]) -> bool {
    for (x1, y1, thres) in limit {
        if (x0 - x1).abs() + (y0 - y1).abs() <= *thres {
            return false;
        }
    }
    true
}

#[test]
fn test_15() {
    assert_eq!(4951427, part1());
    assert_eq!(13029714573243, part2()); // NOTE: use release mode
}
