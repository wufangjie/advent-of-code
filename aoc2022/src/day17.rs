use crate::read_lines;
use std::collections::HashSet;

const FILENAME: &str = "./data/day17.txt";

#[derive(Clone, Copy)]
enum RockType {
    H, // horizontal
    P, // plus shaped
    L, // L shaped
    V, // vertical
    O, // o shaped
}

struct Rock {
    rock_type: RockType,
    x: i64, // left bottom x
    y: i64, // left bottom y
}

impl Rock {
    fn new(rock_type: RockType, y: i64) -> Self {
        Self { rock_type, x: 2, y }
    }

    fn get_right_x(&self) -> i64 {
        match self.rock_type {
            RockType::H => self.x + 3,
            RockType::P => self.x + 2,
            RockType::L => self.x + 2,
            RockType::V => self.x,
            RockType::O => self.x + 1,
        }
    }

    fn get_top_y(&self) -> i64 {
        match self.rock_type {
            RockType::H => self.y,
            RockType::P => self.y + 2,
            RockType::L => self.y + 2,
            RockType::V => self.y + 3,
            RockType::O => self.y + 1,
        }
    }

    fn get_xy(&self) -> Vec<(i64, i64)> {
        let (x, y) = (self.x, self.y);
        match self.rock_type {
            RockType::H => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockType::P => vec![
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 1),
            ],
            RockType::L => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            RockType::V => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            RockType::O => vec![(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)],
        }
    }

    fn get_left_edge(&self) -> Vec<(i64, i64)> {
        let (x, y) = (self.x - 1, self.y);
        match self.rock_type {
            RockType::H => vec![(x, y)],
            RockType::P => vec![(x, y + 1), (x + 1, y), (x + 1, y + 2)],
            RockType::L => vec![(x, y), (x + 2, y + 1), (x + 2, y + 2)],
            RockType::V => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            RockType::O => vec![(x, y), (x, y + 1)],
        }
    }

    fn get_right_edge(&self) -> Vec<(i64, i64)> {
        let (x, y) = (self.x + 1, self.y);
        match self.rock_type {
            RockType::H => vec![(x + 3, y)],
            RockType::P => vec![(x + 1, y), (x + 1, y + 2), (x + 2, y + 1)],
            RockType::L => vec![(x + 2, y), (x + 2, y + 1), (x + 2, y + 2)],
            RockType::V => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            RockType::O => vec![(x + 1, y), (x + 1, y + 1)],
        }
    }

    fn get_down_edge(&self) -> Vec<(i64, i64)> {
        let (x, y) = (self.x, self.y - 1);
        match self.rock_type {
            RockType::H => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockType::P => vec![(x, y + 1), (x + 1, y), (x + 2, y + 1)],
            RockType::L => vec![(x, y), (x + 1, y), (x + 2, y)],
            RockType::V => vec![(x, y)],
            RockType::O => vec![(x, y), (x + 1, y)],
        }
    }
}

struct Machine {
    top_y: i64,
    filled: HashSet<(i64, i64)>,
    gas_all: Vec<u8>,
    gas_cur: usize,
}

impl Machine {
    fn new() -> Self {
        let gas_all = read_lines(FILENAME).next().unwrap().as_bytes().to_vec();
        Self {
            top_y: 0,
            filled: HashSet::new(),
            gas_all,
            gas_cur: 0,
        }
    }

    fn run(&mut self, count: usize) -> i64 {
        let all_types = vec![
            RockType::H,
            RockType::P,
            RockType::L,
            RockType::V,
            RockType::O,
        ];
        for count_i in 0..count {
            let mut rock = Rock::new(all_types[count_i % 5], self.top_y + 4);

            self.move_h(&mut rock);
            // if self.gas_cur > 10091 - 4 {
            //     println!("{}, {}, {}", count_i, self.gas_cur, self.top_y);
            // }
            loop {
                if !self.move_v(&mut rock) {
                    break;
                }
                self.move_h(&mut rock);
            }
            // if self.gas_cur < 4 {
            //     println!("{}, {}, {}", count_i, self.gas_cur, self.top_y);
            // }
            self.top_y = self.top_y.max(rock.get_top_y());
        }
        self.top_y
    }

    fn get_next_gas_char(&mut self) -> u8 {
        if self.gas_cur < self.gas_all.len() {
            self.gas_cur += 1;
            self.gas_all[self.gas_cur - 1]
        } else {
            // dbg!("sdfdsfd");
            self.gas_cur = 1;
            self.gas_all[0]
        }
    }

    fn move_v(&mut self, rock: &mut Rock) -> bool {
        let mut no_block = true;
        if rock.y == 1 {
            no_block = false;
        } else {
            for p in rock.get_down_edge() {
                if self.filled.contains(&p) {
                    no_block = false;
                    break;
                }
            }
        }
        if no_block {
            rock.y -= 1;
            true
        } else {
            for (x, y) in rock.get_xy() {
                self.filled.insert((x, y));
            }
            false
        }
    }

    fn move_h(&mut self, rock: &mut Rock) {
        match self.get_next_gas_char() {
            b'<' => {
                if rock.x > 0 {
                    let mut no_block = true;
                    for p in rock.get_left_edge() {
                        if self.filled.contains(&p) {
                            no_block = false;
                            break;
                        }
                    }
                    if no_block {
                        rock.x -= 1
                    }
                }
            }
            b'>' => {
                if rock.get_right_x() < 6 {
                    let mut no_block = true;
                    for p in rock.get_right_edge() {
                        if self.filled.contains(&p) {
                            no_block = false;
                            break;
                        }
                    }
                    if no_block {
                        rock.x += 1
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

pub fn part1() -> i64 {
    Machine::new().run(2022)
}

pub fn part2() -> i64 {
    let mut m = Machine::new();
    // I really did not understand how the cycle is made, but it is work
    // I just print and find the rule:
    2582 * (1000000000000 / 1705 - 1) + m.run(1000000000000 % 1705 + 1705)
}

#[test]
fn test_17() {
    assert_eq!(3067, part1());
    assert_eq!(1514369501484, part2());
}
