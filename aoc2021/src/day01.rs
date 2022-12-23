use crate::read_lines;

pub fn part1() -> usize {
    let lst: Vec<i32> = read_lines("./data/day01.txt")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();
    (1..lst.len())
        .into_iter()
        .filter(|&i| lst[i] > lst[i - 1])
        .count()
}

pub fn part2() -> usize {
    let lst: Vec<i32> = read_lines("./data/day01.txt")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();
    (3..lst.len())
        .into_iter()
        .filter(|&i| lst[i] > lst[i - 3])
        .count()
}

#[test]
fn test_01() {
    assert_eq!(1288, part1());
    assert_eq!(1311, part2());
}
