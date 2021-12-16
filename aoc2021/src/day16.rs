use crate::read_string;

pub fn part1() -> usize {
    let s = read_string("./data/day16.txt").unwrap();
    let mut ver_sum = 0;
    parse_bits(get_bits(s.trim()).into_iter().by_ref(), &mut ver_sum);
    ver_sum
}

#[test]
fn test_16_1() {
    let f = |s: &str| {
        let mut ver_sum = 0;
        parse_bits(get_bits(s.trim()).into_iter().by_ref(), &mut ver_sum);
        ver_sum
    };
    assert_eq!(16, f("8A004A801A8002F478"));
    assert_eq!(12, f("620080001611562C8802118E34"));
    assert_eq!(23, f("C0015000016115A2E0802F182340"));
    assert_eq!(31, f("A0016C880162017C3686B18A3D4780"));
}

fn get_bits(s: &str) -> Vec<u8> {
    let mut bits = vec![];
    for c in s.bytes() {
        match c {
            b'0'..=b'9' => {
                let cc = c - b'0';
                for i in [3, 2, 1, 0] {
                    bits.push((cc & (1 << i)) >> i);
                }
            }
            b'A'..=b'F' => {
                let cc = c - b'A' + 10;
                for i in [3, 2, 1, 0] {
                    bits.push((cc & (1 << i)) >> i);
                }
            }
            _ => unreachable!(),
        }
    }
    bits
}

fn calc_chunks(iter: &mut impl Iterator<Item = u8>, n: usize) -> usize {
    let mut acc = 0;
    for _ in 0..n {
        acc <<= 1;
        acc |= iter.next().unwrap() as usize;
    }
    acc
}

fn parse_bits(iter: &mut impl Iterator<Item = u8>, ver_sum: &mut usize) -> usize {
    let ver = calc_chunks(iter.by_ref(), 3);
    let typ = calc_chunks(iter.by_ref(), 3);
    let mut cur = 6;
    *ver_sum += ver;

    if typ == 4 {
        cur += parse_type_4(iter.by_ref()).len() / 4 * 5;
    } else {
        let len_id = iter.next().unwrap();
        cur += 1;
        match len_id {
            0 => {
                let mut lens = calc_chunks(iter.by_ref(), 15);
                cur += lens + 15;
                while lens > 0 {
                    let l = parse_bits(iter.by_ref(), ver_sum);
                    lens -= l;
                }
                assert_eq!(lens, 0);
            }
            1 => {
                let mut nums = calc_chunks(iter.by_ref(), 11);
                cur += 11;
                while nums > 0 {
                    let l = parse_bits(iter.by_ref(), ver_sum);
                    nums -= 1;
                    cur += l;
                }
            }
            _ => unreachable!(),
        }
    }
    cur
}

fn parse_type_4(iter: &mut impl Iterator<Item = u8>) -> Vec<u8> {
    let mut ret = vec![];
    loop {
        let prefix = iter.next().unwrap();
        for _ in 0..4 {
            ret.push(iter.next().unwrap());
        }
        if prefix == 0 {
            return ret;
        }
    }
}

pub fn part2() -> usize {
    let s = read_string("./data/day16.txt").unwrap();
    parse_bits_with_op(get_bits(s.trim()).into_iter().by_ref()).1
}

fn parse_bits_with_op(iter: &mut impl Iterator<Item = u8>) -> (usize, usize) {
    let ver = calc_chunks(iter.by_ref(), 3);
    let typ = calc_chunks(iter.by_ref(), 3);
    let mut cur = 6;
    let mut num = 0;

    if typ == 4 {
        let bins = parse_type_4(iter.by_ref());
        cur += bins.len() / 4 * 5;
        for i in bins {
            num <<= 1;
            num |= i as usize;
        }
    } else {
        let len_id = iter.next().unwrap();
        cur += 1;
        let mut operands = vec![];
        match len_id {
            0 => {
                let mut lens = calc_chunks(iter.by_ref(), 15);
                cur += lens + 15;
                while lens > 0 {
                    let (l, num) = parse_bits_with_op(iter.by_ref());
                    lens -= l;
                    operands.push(num);
                }
                assert_eq!(lens, 0);
            }
            1 => {
                let mut nums = calc_chunks(iter.by_ref(), 11);
                cur += 11;
                while nums > 0 {
                    let (l, num) = parse_bits_with_op(iter.by_ref());
                    nums -= 1;
                    cur += l;
                    operands.push(num);
                }
            }
            _ => unreachable!(),
        }

        num = match typ {
            0 => operands.into_iter().sum(),
            1 => operands.into_iter().product(),
            2 => operands.into_iter().min().unwrap(),
            3 => operands.into_iter().max().unwrap(),
            5 => (operands[0] > operands[1]) as usize,
            6 => (operands[0] < operands[1]) as usize,
            7 => (operands[0] == operands[1]) as usize,
            _ => unreachable!(),
        };
    }
    (cur, num)
}

#[test]
fn test_16_2() {
    let f = |s: &str| parse_bits_with_op(get_bits(s).into_iter().by_ref()).1;
    assert_eq!(3, f("C200B40A82"));
    assert_eq!(54, f("04005AC33890"));
    assert_eq!(7, f("880086C3E88112"));
    assert_eq!(9, f("CE00C43D881120"));
    assert_eq!(1, f("D8005AC2A8F0"));
    assert_eq!(0, f("F600BC2D8F"));
    assert_eq!(0, f("9C005AC2F8F0"));
    assert_eq!(1, f("9C0141080250320F1802104A08"));
}
