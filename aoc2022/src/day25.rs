use crate::read_lines;

const N: usize = 22;
const FILENAME: &str = "./data/day25.txt";

fn parse(s: &str, count: &[i64; N]) -> i64 {
    let n = s.len();
    let mut res = 0;
    match &s[..1] {
        "1" => res += count[n - 1],
        "2" => res += count[n - 1] + 5i64.pow(n as u32 - 1),
        _ => (),
    }

    let mut unit = 1;
    for c in s.bytes().skip(1).rev() {
        match c {
            b'=' => (),
            b'-' => res += unit,
            b'0' => res += unit * 2,
            b'1' => res += unit * 3,
            b'2' => res += unit * 4,
            _ => unreachable!(),
        }
        unit *= 5;
    }
    res + 1
}

fn unparse(mut num: i64, count: &[i64; N]) -> String {
    let mut i = 0;
    while count[i] < num {
        i += 1;
    }

    num -= count[i - 1] + 1;
    let mut chars = [""; N];
    chars[N - i] = "1";
    let mut j = N - 1;
    while num > 0 {
        if j == N - i {
            chars[j] = "2";
        } else {
            chars[j] = ["=", "-", "0", "1", "2"][num as usize % 5];
        }
        j -= 1;
        num /= 5;
    }
    for k in N - i + 1..=j {
        chars[k] = "=";
    }
    chars.join("")
}

pub fn part1() -> String {
    let mut count = [0; N]; // 位数不超过下标数的个数
    for i in 1..N {
        count[i] = count[i - 1] + 2 * 5i64.pow(i as u32 - 1);
    }

    let mut res = 0;
    for line in read_lines(FILENAME) {
        res += parse(&line, &count);

        // let temp = parse(&line, &count);
        // let rev = unparse(temp, &count);
        // println!("{}: {}, {}", line, parse(&line, &count), rev);
    }

    unparse(res, &count)
}

#[test]
fn test_25() {
    assert_eq!("2=0=02-0----2-=02-10".to_string(), part1());
}
