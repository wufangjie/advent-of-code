use crate::read_lines;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

// NOTE: sub-game's same state will not end the `whole` game
// only ends a sub-game win for p1

#[derive(Debug)]
enum State {
    P1Win,
    P2Win,
}

pub fn part1() -> usize {
    let mut iter = read_lines("./data/day22.txt").into_iter();
    let mut p1 = Player::new(
        iter.by_ref()
            .skip(1)
            .take_while(|s| !s.is_empty())
            .map(|x| x.parse().unwrap())
            .collect(),
    );
    let mut p2 = Player::new(
        iter.by_ref()
            .skip(1)
            .take_while(|s| !s.is_empty())
            .map(|x| x.parse().unwrap())
            .collect(),
    );

    loop {
        let c1 = p1.draw();
        let c2 = p2.draw();

        if c1 < c2 {
            p2.win_two_card(c2, c1);
            if p1.len() == 0 {
                return p2.calc_score();
            }
        } else {
            p1.win_two_card(c1, c2);
            if p2.len() == 0 {
                return p1.calc_score();
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Player {
    deck: VecDeque<u8>,
}

impl Player {
    fn new(deck: VecDeque<u8>) -> Self {
        Self { deck }
    }

    fn len(&self) -> usize {
        self.deck.len()
    }

    fn draw(&mut self) -> u8 {
        self.deck.pop_front().unwrap()
    }

    fn win_two_card(&mut self, c1: u8, c2: u8) {
        self.deck.push_back(c1);
        self.deck.push_back(c2);
    }

    fn calc_score(&self) -> usize {
        let n = self.deck.len();
        self.deck
            .iter()
            .enumerate()
            .map(|(i, &d)| (n - i) * (d as usize))
            .sum()
    }

    fn copy_deck_next_n(&self, n: usize) -> Self {
        Self {
            deck: self.deck.iter().take(n).cloned().collect(),
        }
    }
}

pub fn part2() -> usize {
    let mut iter = read_lines("./data/day22.txt").into_iter();
    let mut p1 = Player::new(
        iter.by_ref()
            .skip(1)
            .take_while(|s| !s.is_empty())
            .map(|x| x.parse().unwrap())
            .collect(),
    );
    let mut p2 = Player::new(
        iter.by_ref()
            .skip(1)
            .take_while(|s| !s.is_empty())
            .map(|x| x.parse().unwrap())
            .collect(),
    );

    match dfs(&mut p1, &mut p2) {
        State::P1Win => p1.calc_score(),
        State::P2Win => p2.calc_score(),
    }
}

fn dfs(p1: &mut Player, p2: &mut Player) -> State {
    // sub-game will not share same state with their parents (lacking cards)

    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();

    s1.insert(p1.clone());
    s2.insert(p2.clone());

    loop {
        let c1 = p1.draw();
        let c2 = p2.draw();

        if p1.len() >= c1 as usize && p2.len() >= c2 as usize {
            // sub-game
            match dfs(
                &mut p1.copy_deck_next_n(c1 as usize),
                &mut p2.copy_deck_next_n(c2 as usize),
            ) {
                State::P1Win => p1.win_two_card(c1, c2),
                State::P2Win => p2.win_two_card(c2, c1),
            }
        } else if c1 < c2 {
            p2.win_two_card(c2, c1);
            if p1.len() == 0 {
                return State::P2Win;
            }
        } else {
            p1.win_two_card(c1, c2);
            if p2.len() == 0 {
                return State::P1Win;
            }
        }
        if s1.contains(p1) || s2.contains(p2) {
            return State::P1Win;
        }
        s1.insert(p1.clone());
        s2.insert(p2.clone());
    }
}

#[test]
fn test_22() {
    assert_eq!(32495, part1());
    assert_eq!(32665, part2());
}
