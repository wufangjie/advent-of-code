pub fn part1() -> usize {
    let earliest: usize = 1014511;
    let ids: Vec<usize> = "17,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,643,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,433,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19".split(',').filter(|&x| x != "x").map(|x| x.parse().unwrap()).collect();

    let mut wait = usize::MAX;
    let mut id = 0;
    for i in ids {
        let w = (i - earliest % i) % i;
        if w < wait {
            wait = w;
            id = i;
        }
    }
    id * wait
}

pub fn part2() -> usize {
    let mut limits: Vec<(usize, usize)> = vec![];
    for (i, id) in "17,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,643,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,433,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19".split(',').enumerate() {
	match id {
	    "x" => (),
	    x => limits.push((x.parse().unwrap(), i)),
	}
    }

    // prime: a * x + b
    let mut a = 1;
    let mut b = 0;
    for (x, c) in limits {
        for i in 0..x {
            if (a * i + b + c) % x == 0 {
                b += a * i;
                a *= x;
                break;
            }
        }
    }
    b
}
