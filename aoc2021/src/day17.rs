use std::collections::HashSet;

pub fn part1() -> i32 {
    // let (xlo, xhi) = (34, 67);
    let (ylo, yhi): (i32, i32) = (-215, -186);

    // ymax = (n - 1) / 2 + (ylo, yhi) / n, just max(n)
    let mut n = ylo.abs() << 1; // need ylo < yhi < 0
    loop {
        let to_add = n * (n - 1) / 2;
        let ymax = (yhi + to_add) / n;
        if ymax * n >= ylo + to_add {
            return ymax * (ymax + 1) / 2;
        }
        n -= 1;
    }
}

pub fn part2() -> usize {
    // let (xlo, xhi) = (20, 30);
    // let (ylo, yhi) = (-10, -5);

    let (xlo, xhi) = (34, 67);
    let (ylo, yhi): (i32, i32) = (-215, -186);

    let mut nmin = 0;
    while nmin * (nmin + 1) < xlo << 1 {
        nmin += 1
    }
    let mut nmax = nmin;
    while nmax * (nmax + 1) <= xhi << 1 {
        nmax += 1
    }

    let mut res = HashSet::new();

    for n in 1..nmin {
        let to_add = n * (n - 1) / 2;
        let to_sub = (n + 1) / 2;
        for x in xlo / n - to_sub..=xhi / n {
            if n * x + to_add < xlo || n * x + to_add > xhi {
                continue;
            }
            for y in ylo / n - to_sub..=yhi / n {
                if n * y + to_add >= ylo && n * y + to_add <= yhi {
                    res.insert((x + n - 1, y + n - 1));
                }
            }
        }
    }

    let mut n = ylo.abs() << 1; // need ylo < yhi < 0
    while n >= nmin {
        let to_add = n * (n - 1) / 2;
        let mut ymax = (yhi + to_add) / n;
        while ymax * n >= ylo + to_add {
            for x in nmin..nmax {
                // x > nmin, always have > xlo when n >= nmin
                res.insert((x, ymax));
            }
            ymax -= 1;
        }
        n -= 1;
    }

    let mut res: Vec<(i32, i32)> = res.into_iter().collect();
    res.sort();
    // for row in &res {
    //     println!("{}, {}", row.0, row.1);
    // }
    res.len()
}

#[test]
fn test_17() {
    assert_eq!(23005, part1());
    assert_eq!(2040, part2());
}
