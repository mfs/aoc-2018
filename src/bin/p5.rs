extern crate failure;

use std::fs::File;
use std::io::Read;
use failure::{Error, err_msg};

fn process(buffer: &mut Vec<u8>) {
    loop {
        let mut buf = Vec::new();

        let mut i = 0;
        loop {

            if i == buffer.len() - 1 {
                buf.push(buffer[i]);
                break;
            } else if i > buffer.len() - 1 {
                break;
            }

            let a = buffer[i];
            let b = buffer[i+1];

            if (a & 0b11111) == (b & 0b11111) && a != b {
                i += 2; // skip
            } else {
                buf.push(a);
                i += 1;
            }
        }

        if buf.len() < buffer.len() {
            *buffer = buf;
        } else {
            break;
        }
    }
}


fn main() -> Result<(), Error> {
    let mut file = File::open("data/p5.txt")?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let buffer2 = buffer.clone();

    process(&mut buffer);

    println!("Part 1: {}", buffer.len());

    let mut scores = vec![];

    for c in 65..91 {
        let mut v: Vec<u8> = buffer2.iter().filter(|&x| *x != c && *x != c + 32).map(|&x| x).collect();
        process(&mut v);
        scores.push(v.len());
    }

    let shortest = scores.iter().min().ok_or(err_msg("empty vector"))?;
    println!("Part 2: {}", shortest);

    Ok(())
}
