use crate::read_lines;

pub fn part1() -> usize {
    let lines: Vec<Vec<u8>> = read_lines("./data/day15.txt")
        .into_iter()
        .map(|line| line.bytes().map(|x| x - b'0').collect())
        .collect();
    let nrow = lines.len();
    let ncol = lines[0].len();
    let mut dp = Vec::with_capacity(ncol);
    dp.push(0);
    for j in 1..ncol {
        dp.push(dp[j - 1] + lines[0][j] as usize);
    }
    for i in 1..nrow {
        dp[0] += lines[i][0] as usize;
        for j in 1..ncol {
            dp[j] = dp[j - 1].min(dp[j]) + lines[i][j] as usize;
        }
    }
    dp[ncol - 1]
}

const N: usize = 5;

pub fn part2() -> usize {
    let lines: Vec<Vec<usize>> = read_lines("./data/day15.txt")
        .into_iter()
        .map(|line| line.bytes().map(|x| (x - b'0') as usize).collect())
        .collect();
    let nrow = lines.len();
    let ncol = lines[0].len();

    let find_real_risk = |i: usize, j: usize| -> usize {
        let ret = (lines[i % nrow][j % ncol] + i / nrow + j / ncol);
        ret % 10 + ret / 10
    };

    let mut dp = vec![usize::MAX; ncol * N];
    dp[0] = 0;
    for i in 0..nrow * N {
        dp[0] += find_real_risk(i, 0);
        for j in 1..ncol * N {
            dp[j] = dp[j - 1].min(dp[j]) + find_real_risk(i, j);
        }
    }
    dp[ncol * N - 1] - lines[0][0]
}

// 2948
