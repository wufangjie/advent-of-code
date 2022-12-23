use crate::read_string;

pub fn part1() -> usize {
    //let position: Vec<usize> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let position: Vec<usize> = read_string("./data/day07.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let calc_fuel = |p| {
        position
            .iter()
            .map(|&x| if x > p { x - p } else { p - x })
            .sum()
    };

    calc_convex_min(&position, calc_fuel)
}

pub fn part2() -> usize {
    // min \Sigma n(n + 1) / 2, n == x - p0, \Sigma x == C
    // (x - p0) * (x - p0 + 1) = x^2 + (-2p0 + 1)x - p0
    // which is a convex function

    let position: Vec<usize> = read_string("./data/day07.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let calc_fuel = |p| {
        position
            .iter()
            .map(|&x| {
                if x > p {
                    (x - p) * (x - p + 1) / 2
                } else {
                    (p - x) * (p - x + 1) / 2
                }
            })
            .sum()
    };

    calc_convex_min(&position, calc_fuel)
}

fn calc_convex_min(position: &[usize], calc_fuel: impl Fn(usize) -> usize) -> usize {
    let avg = position.iter().sum::<usize>() / position.len();
    let mut min_cost: usize = calc_fuel(avg);

    let mut i = avg + 1;
    loop {
        let cost = calc_fuel(i);
        if cost >= min_cost {
            if i == avg + 1 {
                break;
            } else {
                return min_cost;
            }
        } else {
            min_cost = cost;
        }
        i += 1;
    }

    let mut i = avg - 1;
    loop {
        let cost = calc_fuel(i);
        if cost > min_cost {
            return min_cost;
        } else {
            min_cost = cost;
        }
        i -= 1;
    }
}

#[test]
fn test_07() {
    // dbg!(part1());
    // dbg!(part2());
    assert_eq!(359648, part1());
    assert_eq!(100727924, part2());
}
