use crate::read_lines;

pub fn part1() -> i32 {
    let rows = read_lines("./data/day2.txt");
    let mut x = 0;
    let mut y = 0;
    for row in rows {
        let ops: Vec<&str> = row.split(' ').collect();
        let n = ops[1].parse::<i32>().unwrap();
        match ops[0] {
            "forward" => x += n,
            "down" => y += n,
            "up" => y -= n,
            _ => unreachable!(),
        }
    }
    x * y
}

pub fn part2() -> i32 {
    let rows = read_lines("./data/day2.txt");
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for row in rows {
        let ops: Vec<&str> = row.split(' ').collect();
        let n = ops[1].parse::<i32>().unwrap();
        match ops[0] {
            "forward" => {
                x += n;
                y += aim * n;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => unreachable!(),
        }
    }
    x * y
}
