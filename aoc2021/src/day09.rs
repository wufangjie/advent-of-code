use crate::read_lines;
use utils::Heap;

pub fn part1() -> usize {
    let map: Vec<Vec<u8>> = read_lines("./data/day9.txt")
        .into_iter()
        .map(|row| row.bytes().collect::<Vec<u8>>())
        .collect();
    let nrow = map.len();
    let ncol = map[0].len();

    let mut risk_levels = 0;
    for i in 0..nrow {
        for j in 0..ncol {
            risk_levels += get_risk_level(&map, i, j);
        }
    }
    risk_levels
}

fn get_risk_level(map: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    if i > 0 && map[i - 1][j] <= map[i][j] {
        return 0;
    }
    if i < map.len() - 1 && map[i + 1][j] <= map[i][j] {
        return 0;
    }
    if j > 0 && map[i][j - 1] <= map[i][j] {
        return 0;
    }
    if j < map[0].len() - 1 && map[i][j + 1] <= map[i][j] {
        return 0;
    }
    map[i][j] as usize - 47 // - b'0' + 1
}

//const N9: u8 = b'9';

pub fn part2() -> usize {
    let map: Vec<Vec<u8>> = read_lines("./data/day9.txt")
        .into_iter()
        .map(|row| row.bytes().collect::<Vec<u8>>())
        .collect();
    let nrow = map.len();
    let ncol = map[0].len();

    let mut visited = vec![vec![false; ncol]; nrow];
    let mut heap = Heap::new();
    for i in 0..nrow {
        for j in 0..ncol {
            if map[i][j] != b'9' && !visited[i][j] {
                let area = dfs(&map, &mut visited, i, j);
                if heap.len() < 3 {
                    heap.push(area);
                } else {
                    heap.pushpop(area);
                }
            }
        }
    }

    let mut ret = 1;
    while let Some(area) = heap.pop() {
        ret *= area;
    }
    ret
}

fn dfs(map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    visited[i][j] = true;
    let mut ret = 1;
    if i > 0 && map[i - 1][j] != b'9' && !visited[i - 1][j] {
        ret += dfs(map, visited, i - 1, j);
    }
    if i < map.len() - 1 && map[i + 1][j] != b'9' && !visited[i + 1][j] {
        ret += dfs(map, visited, i + 1, j);
    }
    if j > 0 && map[i][j - 1] != b'9' && !visited[i][j - 1] {
        ret += dfs(map, visited, i, j - 1);
    }
    if j < map[0].len() - 1 && map[i][j + 1] != b'9' && !visited[i][j + 1] {
        ret += dfs(map, visited, i, j + 1);
    }
    ret
}
