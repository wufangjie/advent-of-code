const P: usize = 20201227; // is a prime
const CPK: usize = 14788856;
const DPK: usize = 19316454;

// const CPK: usize = 5764801;
// const DPK: usize = 17807724;

pub fn part1() {
    let cls = find_loop_size(CPK); // card loop size
    let dls = find_loop_size(DPK); // card loop size
    dbg!(cls, dls);

    dbg!(calc_encryption_key(CPK, dls));
    dbg!(calc_encryption_key(DPK, cls));
}

fn find_loop_size(public_key: usize) -> usize {
    let mut loop_size = 1;
    let mut x = 7;
    loop {
        x *= 7;
        x %= P;
        if x == public_key {
            return loop_size;
        }
        loop_size += 1;
    }
}

fn calc_encryption_key(public_key: usize, loop_size: usize) -> usize {
    // NOTE: different owner
    let mut encryption_key = public_key;
    for _ in 0..loop_size {
        encryption_key *= public_key;
        encryption_key %= P;
    }
    encryption_key
}

#[test]
fn test_25() {
    part1();
    //assert_eq!(545789, part1());
}
