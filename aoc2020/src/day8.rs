use crate::read_lines;

const NOP: u8 = 0;
const JMP: u8 = 1;
const ACC: u8 = 2;

fn get_lines() -> Vec<(u8, i32)> {
    let conv = |line: String| {
        let kv: Vec<&str> = line.split(' ').collect();
        (
            match kv[0] {
                "jmp" => JMP,
                "acc" => ACC,
                _ => NOP,
            },
            kv[1].parse().unwrap(),
        )
    };

    read_lines("./data/day8.txt")
        .into_iter()
        .map(conv)
        .collect()
}

pub fn part1() -> i32 {
    let mut lines = get_lines();
    let nrow = lines.len();
    let mut flag = vec![false; nrow];
    let mut acc = 0;
    let mut i = 0;

    loop {
        if flag[i] {
            break;
        }
        flag[i] = true;
        match lines[i].0 {
            JMP => i = (i as i32 + lines[i].1) as usize,
            ACC => {
                acc += lines[i].1;
                i += 1;
            }
            _ => i += 1,
        }
    }
    acc
}

pub fn part2() -> i32 {
    let mut lines = get_lines();

    let nrow = lines.len();
    let mut rec_acc = vec![i32::MIN; nrow];
    let mut acc = 0;
    let mut i = 0;

    loop {
        if rec_acc[i] != i32::MIN {
            break;
        }
        rec_acc[i] = acc;
        match lines[i].0 {
            JMP => i = (i as i32 + lines[i].1) as usize,
            ACC => {
                acc += lines[i].1;
                i += 1;
            }
            _ => i += 1,
        }
    }

    acc = rec_acc[i];
    loop {
        match lines[i].0 {
            JMP => {
                lines[i].0 ^= 1;
                let (succeed, res) = can_succeed(&lines, i, acc);
                if succeed {
                    return res;
                }

                lines[i].0 ^= 1;
                i = (i as i32 + lines[i].1) as usize;
            }
            ACC => {
                acc += lines[i].1;
                i += 1;
            }
            _ => {
                lines[i].0 ^= 1;
                let (succeed, res) = can_succeed(&lines, i, acc);
                if succeed {
                    return res;
                }

                lines[i].0 ^= 1;
                i += 1;
            }
        }
    }
}

fn can_succeed(lines: &Vec<(u8, i32)>, mut i: usize, mut acc: i32) -> (bool, i32) {
    let nrow = lines.len();
    for _ in 0..nrow {
        if i == nrow {
            return (true, acc);
        } else if i < 0 || i > nrow {
            return (false, acc);
        }
        match lines[i].0 {
            JMP => i = (i as i32 + lines[i].1) as usize,
            ACC => {
                acc += lines[i].1;
                i += 1;
            }
            _ => i += 1,
        }
    }
    (false, acc)
}
