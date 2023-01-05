use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

//mod day01;
//mod day02;
//mod day03;
//mod day04;
//mod day05;
//mod day06;
//mod day07;
//mod day08;
mod day09;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    io::BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|x| x.unwrap())
}
