use crate::read_lines;
use regex::Regex;
use std::collections::HashSet;

pub fn part1() -> usize {
    let lines = read_lines("./data/day22.txt");
    let re = Regex::new(
        r"^(on|off) x=([-0-9]+)\.\.([-0-9]+),y=([-0-9]+)\.\.([-0-9]+),z=([-0-9]+)\.\.([-0-9]+)",
    )
    .unwrap();
    let n = lines.len();
    let mut state_lst = Vec::with_capacity(n);
    let mut cube_lst: Vec<Vec<i32>> = Vec::with_capacity(n);
    for line in lines {
        let caps = re.captures(&line).unwrap();
        state_lst.push(if caps.get(1).unwrap().as_str() == "on" {
            true
        } else {
            false
        });
        cube_lst.push(
            (2..8)
                .into_iter()
                .map(|i| caps.get(i).unwrap().as_str().parse().unwrap())
                .collect(),
        );
    }

    let mut xset = HashSet::new();
    let mut yset = HashSet::new();
    let mut zset = HashSet::new();

    xset.insert(-50);
    xset.insert(51);
    yset.insert(-50);
    yset.insert(51);
    zset.insert(-50);
    zset.insert(51);

    for cube in &cube_lst {
        for x in [cube[0], cube[1] + 1] {
            if x > -50 && x < 51 {
                xset.insert(x);
            }
        }
        for x in [cube[2], cube[3] + 1] {
            if x > -50 && x < 51 {
                yset.insert(x);
            }
        }
        for x in [cube[4], cube[5] + 1] {
            if x > -50 && x < 51 {
                zset.insert(x);
            }
        }
    }

    let mut xlst: Vec<i32> = xset.into_iter().collect();
    let mut ylst: Vec<i32> = yset.into_iter().collect();
    let mut zlst: Vec<i32> = zset.into_iter().collect();
    xlst.sort_unstable();
    ylst.sort_unstable();
    zlst.sort_unstable();
    let nx = xlst.len();
    let ny = ylst.len();
    let nz = zlst.len();

    let mut small_cubes = Vec::with_capacity((nx - 1) * (ny - 1) * (nz - 1));
    for i in 1..nx {
        for j in 1..ny {
            for k in 1..nz {
                small_cubes.push((
                    xlst[i - 1],
                    xlst[i],
                    ylst[j - 1],
                    ylst[j],
                    zlst[k - 1],
                    zlst[k],
                ));
            }
        }
    }

    let m = small_cubes.len();
    let mut state_small = vec![false; m];
    for i in 0..n {
        for j in 0..m {
            if state_lst[i] != state_small[j] {
                if small_cubes[j].0 >= cube_lst[i][0]
                    && small_cubes[j].1 - 1 <= cube_lst[i][1]
                    && small_cubes[j].2 >= cube_lst[i][2]
                    && small_cubes[j].3 - 1 <= cube_lst[i][3]
                    && small_cubes[j].4 >= cube_lst[i][4]
                    && small_cubes[j].5 - 1 <= cube_lst[i][5]
                {
                    state_small[j] = state_lst[i];
                }
            }
        }
    }

    let mut count = 0;
    for j in 0..m {
        if state_small[j] {
            count += (small_cubes[j].1 - small_cubes[j].0) as usize
                * (small_cubes[j].3 - small_cubes[j].2) as usize
                * (small_cubes[j].5 - small_cubes[j].4) as usize;
        }
    }
    count
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day22.txt");
    let re = Regex::new(
        r"^(on|off) x=([-0-9]+)\.\.([-0-9]+),y=([-0-9]+)\.\.([-0-9]+),z=([-0-9]+)\.\.([-0-9]+)",
    )
    .unwrap();
    let n = lines.len();
    let mut state_lst = Vec::with_capacity(n);
    let mut cube_lst: Vec<Vec<i32>> = Vec::with_capacity(n);
    for line in lines {
        let caps = re.captures(&line).unwrap();
        state_lst.push(if caps.get(1).unwrap().as_str() == "on" {
            true
        } else {
            false
        });
        cube_lst.push(
            (2..8)
                .into_iter()
                .map(|i| caps.get(i).unwrap().as_str().parse().unwrap())
                .collect(),
        );
    }

    let mut xset = HashSet::new();
    let mut yset = HashSet::new();
    let mut zset = HashSet::new();

    for cube in &cube_lst {
        xset.insert(cube[0]);
        xset.insert(cube[1] + 1);
        yset.insert(cube[2]);
        yset.insert(cube[3] + 1);
        zset.insert(cube[4]);
        zset.insert(cube[5] + 1);
    }

    let mut xlst: Vec<i32> = xset.into_iter().collect();
    let mut ylst: Vec<i32> = yset.into_iter().collect();
    let mut zlst: Vec<i32> = zset.into_iter().collect();
    xlst.sort_unstable();
    ylst.sort_unstable();
    zlst.sort_unstable();
    let nx = xlst.len();
    let ny = ylst.len();
    let nz = zlst.len();

    let mut state_small = vec![vec![vec![false; nz - 1]; ny - 1]; nx - 1];
    for m in 0..n {
        for i in 0..nx - 1 {
            if xlst[i] < cube_lst[m][0] || xlst[i + 1] - 1 > cube_lst[m][1] {
                continue;
            }
            for j in 0..ny - 1 {
                if ylst[j] < cube_lst[m][2] || ylst[j + 1] - 1 > cube_lst[m][3] {
                    continue;
                }
                for k in 0..nz - 1 {
                    if zlst[k] < cube_lst[m][4] || zlst[k + 1] - 1 > cube_lst[m][5] {
                        continue;
                    }
                    state_small[i][j][k] = state_lst[m];
                }
            }
        }
    }

    let mut count = 0;
    for i in 0..nx - 1 {
        for j in 0..ny - 1 {
            for k in 0..nz - 1 {
                if state_small[i][j][k] {
                    count += (xlst[i + 1] - xlst[i]) as usize
                        * (ylst[j + 1] - ylst[j]) as usize
                        * (zlst[k + 1] - zlst[k]) as usize;
                }
            }
        }
    }
    count
}
