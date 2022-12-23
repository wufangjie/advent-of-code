use crate::read_lines;
use utils::Heap;

pub fn part1() -> usize {
    calc_lowest(1)
}

pub fn part2() -> usize {
    calc_lowest(5)
}

// NOTE: the DP answer is wrong: left, up is possible
// pub fn part2() -> usize {
//     let lines: Vec<Vec<usize>> = read_lines("./data/15.txt")
//         .into_iter()
//         .map(|line| line.bytes().map(|x| (x - b'0') as usize).collect())
//         .collect();
//     let nrow = lines.len();
//     let ncol = lines[0].len();

//     let find_real_risk = |i: usize, j: usize| -> usize {
//         let ret = (lines[i % nrow][j % ncol] + i / nrow + j / ncol);
//         ret % 10 + ret / 10
//     };

//     let mut dp = vec![vec![usize::MAX; ncol * N]; nrow];
//     dp[0] = 0;
//     for i in 0..nrow * N {
//         dp[0] += find_real_risk(i, 0);
//         for j in 1..ncol * N {
//             dp[j] = dp[j - 1].min(dp[j]) + find_real_risk(i, j);
//         }
//     }
//     dp[ncol * N - 1] - lines[0][0]
// }

fn calc_lowest(n: usize) -> usize {
    let lines: Vec<Vec<usize>> = read_lines("./data/day15.txt")
        .into_iter()
        .map(|line| line.bytes().map(|x| (x - b'0') as usize).collect())
        .collect();
    let nrow = lines.len();
    let ncol = lines[0].len();

    let find_real_risk = |i: usize, j: usize| -> usize {
        let ret = lines[i % nrow][j % ncol] + i / nrow + j / ncol;
        ret % 10 + ret / 10
    };

    let nrow_n = nrow * n;
    let ncol_n = ncol * n;

    let mut lowest = vec![vec![0; ncol_n]; nrow_n];
    lowest[0][0] = 1;
    let mut heap = Heap::new();
    heap.push((0, 0, 0));
    while let Some((d, i, j)) = heap.pop() {
        if i == nrow_n - 1 && j == ncol_n - 1 {
            return d;
        }

        for ii in make_iter(i, nrow_n - 1) {
            if lowest[ii][j] == 0 {
                let d2 = d + find_real_risk(ii, j);
                lowest[ii][j] = d2;
                heap.push((d2, ii, j));
            }
        }
        for jj in make_iter(j, ncol_n - 1) {
            if lowest[i][jj] == 0 {
                let d2 = d + find_real_risk(i, jj);
                lowest[i][jj] = d2;
                heap.push((d2, i, jj));
            }
        }
    }
    unreachable!();
}

#[inline]
fn make_iter(i: usize, hi: usize) -> impl Iterator<Item = usize> {
    if i == 0 {
        0..=1
    } else if i < hi {
        i - 1..=i + 1
    } else {
        i - 1..=i
    }
}

#[test]
fn test_15() {
    assert_eq!(707, part1());
    assert_eq!(2942, part2());
}
