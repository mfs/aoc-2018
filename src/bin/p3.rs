extern crate failure;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use failure::{Error, format_err};

struct Square {
    id: i64,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

impl Square {
    fn parse(s: &str) -> Result<Square, Error> {
        let el: Vec<_> = s.split(|x: char| !x.is_numeric())
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();

        if el.len() != 5 {
            return Err(format_err!("error parsing: {}", s));
        }

        Ok(Square {
            id: el[0],
            x: el[1],
            y: el[2],
            w: el[3],
            h: el[4],
        })
    }
}

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("data/p3.txt")?);

    let mut squares = vec![];

    for line in file.lines() {
        let sq = Square::parse(&line?)?;
        squares.push(sq);
    }

    let mut fabric = HashMap::new();

    for sq in &squares {
        for x in 0..sq.w {
            for y in 0..sq.h {
                let square = fabric.entry((x + sq.x, y + sq.y)).or_insert(sq.id);
                if *square != sq.id {
                    *square = -1;
                }
            }
        }
    }

    let count = fabric.values().filter(|&x| *x == -1).count();

    println!("Part 1: {}", count);

    let mut counts = HashMap::new();

    for x in fabric.values() {
        *counts.entry(x).or_insert(0) += 1;
    }

    for sq in &squares {
        if *counts.get(&sq.id).unwrap_or(&0) == (sq.w * sq.h) {
            println!("Part 2: {}", sq.id);
            break;
        }
    }

    Ok(())
}
