extern crate failure;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use failure::Error;

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("data/p1.txt")?);

    let mut data = vec![];
    for line in file.lines() {
        data.push(line?.parse()?);
    }

    let total: i64 = data.iter().sum();
    println!("Part 1: {}", total);

    let mut total = 0;
    let mut freqs = HashSet::new();

    for n in data.iter().cycle() {
        total += n;
        if freqs.contains(&total) {
            break;
        }
        freqs.insert(total);
    }

    println!("Part 2: {}", total);

    Ok(())
}
