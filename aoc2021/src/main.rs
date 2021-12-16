use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
//use std::env; // dbg!(env::current_dir());

mod day16;

fn main() {
    dbg!(day16::part2());
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    io::BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect()
}

fn read_string<P>(filename: P) -> Result<String, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename)
}
