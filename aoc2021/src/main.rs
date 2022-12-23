use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
//use std::env; // dbg!(env::current_dir());

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06; // check
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
// mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    // let lst = vec![Some(3), None, Some(42)];
    // use utils::dbgt;
    // dbgt!(&(&lst)[0]);
    // dbgt!(&(&lst)[0].unwrap());
    // dbgt!(&(&lst)[0].as_ref());
    // dbgt!(&(&lst)[0].as_ref().unwrap());

    //dbg!(day19::part2());

    // let mut cnt = 0;
    // let mut x = 0b1010101001;

    // println!("{:032b}", 0x55555555);
    // println!("{:032b}", 0x33333333);
    // println!("{:032b}", 0x0f0f0f0f);
    // println!("{:032b}", 0x01010101);

    // while x != 0 {
    //     // println!("{:b}", x);
    //     // println!("{:b}", x - 1);
    //     x &= x - 1;

    //     cnt += 1;
    // }
    // dbg!(cnt);
}

#[allow(dead_code)]
fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    io::BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_string(filename: impl AsRef<Path>) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}
