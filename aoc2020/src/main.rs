use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day17;
// mod day9;
// mod day16;
mod day18;

fn main() {
    dbg!(day18::part2());
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
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}
