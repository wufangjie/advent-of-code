use crate::read_lines;

pub fn part1() -> i64 {
    let mut x = 1i64;
    let mut round = 0;

    let mut res = 0;
    for line in read_lines("./data/day10.txt") {
        if line == "noop" {
            round += 1;
            if (round - 20) % 40 == 0 {
                dbg!(round, x);
                res += round * x;
            }
        } else {
            round += 2;
            // NOTE: 用 +20 而不是 -20
            if (round + 20) % 40 <= 1 {
                res += if round % 2 == 1 { round - 1 } else { round } * x;
            }
            x += &line[5..].parse().unwrap();
        }
    }
    res
}

pub fn part2() {
    let mut x = 1i64;
    let mut pos = 0;

    for line in read_lines("./data/day10.txt") {
        if line == "noop" {
            pos += 1;
            if pos >= x && pos <= x + 2 {
                print!("#");
            } else {
                print!(".");
            }
            if pos >= 40 {
                pos -= 40;
                println!();
            }
        } else {
            for _ in 0..2 {
                pos += 1;
                if pos >= x && pos <= x + 2 {
                    print!("#");
                } else {
                    print!(".");
                }
                if pos >= 40 {
                    pos -= 40;
                    println!();
                }
            }
            x += &line[5..].parse().unwrap();
        }
    }
}

#[test]
fn test_10() {
    assert_eq!(16880, part1());
    dbg!(part2()); // RKAZAJBR
}
