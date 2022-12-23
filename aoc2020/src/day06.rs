use crate::read_lines;
use std::collections::HashSet;

pub fn part1() -> usize {
    let lines = read_lines("./data/day6.txt");
    let mut questions = HashSet::new();
    let mut count = 0;
    for line in lines {
        if line.is_empty() {
            count += questions.len();
            questions.clear();
        } else {
            for c in line.chars() {
                questions.insert(c);
            }
        }
    }
    count + questions.len()
}

pub fn part2() -> usize {
    let lines = read_lines("./data/day6.txt");
    let mut questions = HashSet::new();
    let mut questions_inter = HashSet::new();
    let mut state = 0; // { 0: first line, 1: normal, 2: can skip }
    let mut count = 0;
    for line in lines {
        if line.is_empty() {
            count += questions.len();
            questions.clear();
            state = 0;
        } else {
            match state {
                0 => {
                    for c in line.chars() {
                        questions.insert(c);
                    }
                    state = 1;
                }
                1 => {
                    for c in line.chars() {
                        if questions.contains(&c) {
                            questions_inter.insert(c);
                        }
                    }
                    if questions_inter.is_empty() {
                        state = 2;
                    }
                    questions = questions_inter;
                    questions_inter = HashSet::new();
                }
                _ => (),
            }
        }
    }
    count + questions.len()
}

#[test]
fn test_06() {
    assert_eq!(7128, part1());
    assert_eq!(3640, part2());
}
