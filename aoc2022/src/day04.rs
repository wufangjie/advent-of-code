use crate::read_lines;

pub fn part1() -> usize {
    let mut count = 0;
    for line in read_lines("./data/day04.txt") {
        let quad: Vec<i32> = line
            .split(&[',', '-'][..])
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if quad[0] == quad[2] || quad[1] == quad[3] {
            count += 1;
        } else if quad[0] < quad[2] {
            if quad[1] > quad[3] {
                count += 1;
            }
        } else if quad[3] > quad[1] {
            count += 1;
        }
    }
    count
}

pub fn part2() -> usize {
    let mut count = 0;
    for line in read_lines("./data/day04.txt") {
        let quad: Vec<i32> = line
            .split(&[',', '-'][..])
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if quad[0] <= quad[3] && quad[2] <= quad[1] {
            count += 1;
        }
    }
    count
}

#[test]
fn test_04() {
    assert_eq!(534, part1());
    assert_eq!(841, part2());
}
