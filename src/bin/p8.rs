extern crate failure;

use std::fs::File;
use std::io::Read;
use failure::{Error, err_msg};

fn next(iter: &mut std::slice::Iter<u64>) -> Result<u64, Error> {
    Ok(*iter.next().ok_or(err_msg(""))?)
}

fn node(iter: &mut std::slice::Iter<u64>) -> Result<u64, Error> {

    let n_children = next(iter)?;
    let n_metadata = next(iter)?;

    let mut metadata = 0;

    for _ in 0..n_children {
        metadata += node(iter)?;
    }
    for _ in 0..n_metadata {
        metadata += next(iter)?;
    }

    Ok(metadata)
}

fn node2(iter: &mut std::slice::Iter<u64>) -> Result<u64, Error> {

    let n_children = next(iter)?;
    let n_metadata = next(iter)?;

    if n_children == 0 {
        let mut metadata = 0;

        for _ in 0..n_metadata {
            metadata += next(iter)?;
        }
        return Ok(metadata);
    }

    let mut children = vec![];

    for _ in 0..n_children {
        children.push(node2(iter)?);
    }

    let mut total = 0;
    for _ in 0..n_metadata {
        let n = (next(iter)? - 1) as usize;
        if n < children.len() {
            total += children[n];
        }
    }

    Ok(total)
}

fn main() -> Result<(), Error> {
    let mut file = File::open("data/p8.txt")?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let data: Vec<u64> = buffer.split(' ').filter_map(|x| x.parse().ok()).collect();

    let part1 = node(&mut data.iter())?;

    println!("Part 1: {}", part1);

    let part2 = node2(&mut data.iter())?;

    println!("Part 2: {}", part2);

    Ok(())
}
