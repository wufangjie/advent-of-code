use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
//use std::env; // dbg!(env::current_dir());

mod day13;

fn main() {
    dbg!(day13::part2());
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
