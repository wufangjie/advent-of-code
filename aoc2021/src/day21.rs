use std::collections::HashMap;

pub fn part1() -> usize {
    // let mut p1 = Player::new(4, 2);
    // let mut p2 = Player::new(8, 5);
    let mut p1 = Player::new(8, 2);
    let mut p2 = Player::new(10, 5);

    loop {
        if p1.next() {
            return (p1.count + p2.count) * p2.score;
        } else {
            if p2.next() {
                return (p1.count + p2.count) * p1.score;
            }
        }
    }
}

#[derive(Debug)]
struct Player {
    space: usize, // 0~9
    dice: usize,  // 1~1000, the mid one
    score: usize,
    count: usize,
}

impl Player {
    fn new(space: usize, dice: usize) -> Self {
        Self {
            space: space - 1,
            dice,
            score: 0,
            count: 0,
        }
    }

    fn next(&mut self) -> bool {
        self.space = (self.space + self.dice * 3) % 10;
        self.score += self.space + 1;
        self.count += 3;
        if self.score >= 1000 {
            true
        } else {
            self.dice += 6;
            if self.dice > 100 {
                self.dice -= 100;
            }
            false
        }
    }
}

pub fn part2() {
    let mut game = Game::new();
    // dbg!(game.count(4, 0, 8, 0));
    dbg!(game.count(8, 0, 10, 0));
}

struct Game {
    cache: HashMap<(usize, usize, usize, usize), (usize, usize)>,
}

impl Game {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn count(&mut self, p1: usize, s1: usize, p2: usize, s2: usize) -> (usize, usize) {
        if s2 >= 21 {
            return (0, 1);
        }

        if let Some((c1, c2)) = self.cache.get(&(p1, s1, p2, s2)) {
            return (*c1, *c2);
        }
        // NOTE: NOTE: NOTE: following rows are wrong,
        // in this case p1 rolls, rather than p2
        // if let Some((c2, c1)) = self.cache.get(&(p2, s2, p1, s1)) {
        //     return (*c1, *c2);
        // }

        let mut c1s = 0;
        let mut c2s = 0;
        for (dice_sum, count) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let mut p12 = p1 + dice_sum;
            if p12 > 10 {
                p12 -= 10;
            }
            let (c2, c1) = self.count(p2, s2, p12, s1 + p12); // NOTE: the order
            c1s += c1 * count;
            c2s += c2 * count;
        }
        self.cache.insert((p1, s1, p2, s2), (c1s, c2s));
        (c1s, c2s)
    }
}
