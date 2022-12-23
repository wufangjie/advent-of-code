use crate::read_lines;

const N: usize = 9;

pub fn part1() -> usize {
    count_fish(80)
}

pub fn part2() -> usize {
    count_fish(256)
}

fn count_fish(days: usize) -> usize {
    let lines = read_lines("./data/day06.txt");

    let mut timer = [0; N];
    for s in lines[0].split(',') {
        timer[s.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..days {
        let to_create = timer[0];
        for j in 0..N - 1 {
            timer[j] = timer[j + 1];
        }
        timer[N - 1] = to_create;
        timer[6] += to_create;
    }
    timer.into_iter().sum()
}

#[test]
fn test_06() {
    // dbg!(part1());
    // dbg!(part2());

    assert_eq!(349549, part1());
    assert_eq!(1589590444365, part2());
}
