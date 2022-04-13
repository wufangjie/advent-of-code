use crate::read_lines;

pub fn part1() -> i32 {
    let lines = read_lines("./data/day1.txt");
    let mut lines: Vec<i32> = lines.into_iter().map(|x| x.parse().unwrap()).collect();
    lines.sort_unstable();

    let mut lo = 0;
    let mut hi = lines.len() - 1;
    while hi > lo {
        match lines[hi] + lines[lo] {
            2020 => return lines[hi] * lines[lo],
            x if x > 2020 => hi -= 1,
            _ => lo += 1,
        }
    }
    -1
}

pub fn part2() -> i32 {
    let lines = read_lines("./data/day1.txt");
    let mut lines: Vec<i32> = lines.into_iter().map(|x| x.parse().unwrap()).collect();
    lines.sort_unstable();

    let mut lo2 = 0;
    let mut hi2 = lines.len() - 1;
    while lo2 < hi2 {
        let target = 2020 - lines[lo2];
        let mut lo = lo2 + 1;
        let target_hi = target - lines[lo];
        while lines[hi2] > target_hi {
            hi2 -= 1;
        }
        let mut hi = hi2;
        while hi > lo {
            match lines[hi] + lines[lo] {
                x if x == target => return lines[lo2] * lines[hi] * lines[lo],
                x if x > target => hi -= 1,
                _ => lo += 1,
            }
        }
        lo2 += 1;
    }
    -1
}

#[test]
fn test_day01() {
    assert_eq!(326211, part1());
    assert_eq!(131347190, part2());
}
