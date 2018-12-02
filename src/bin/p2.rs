extern crate failure;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use failure::Error;

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("data/p2.txt")?);

    let mut twos = 0;
    let mut threes = 0;

    let mut box_ids = vec![];

    for line in file.lines() {
        box_ids.push(line?);
    }

    for id in &box_ids {

        let mut dict = HashMap::new();

        for ch in id.chars() {
            *dict.entry(ch).or_insert(0) += 1;
        }

        let mut counts = HashSet::new();
        for count in dict.values() {
            counts.insert(count);
        }

        if counts.contains(&2) {
            twos += 1;
        }
        if counts.contains(&3) {
            threes += 1;
        }
    }

    println!("Part 1: {}", twos * threes);

    'outer: for x in &box_ids {
        for y in &box_ids {
            let mut diff = 0;

            for (cx, cy) in x.chars().zip(y.chars()) {
                if cx != cy {
                    diff += 1;
                }
            }

            if diff == 1 {
                let answer:String = x.chars().zip(y.chars())
                    .filter(|&(a, b)| a == b)
                    .map(|(c,_)| c)
                    .collect();
                println!("Part 2: {}", answer);
                break 'outer;
            }
        }
    }

    Ok(())
}
