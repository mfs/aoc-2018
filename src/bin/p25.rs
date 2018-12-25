extern crate failure;

use failure::Error;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn dis(a: (i64, i64, i64, i64, i64), b: (i64, i64, i64, i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("data/p25.txt")?);

    let mut points = vec![];

    for (i, line) in file.lines().enumerate() {

        let el: Vec<i64> = line?.split(',').filter_map(|x| x.parse().ok()).collect();

        // (a, b, c, d, constellation_id)
        points.push((el[0], el[1], el[2], el[3], i as i64));
    }

    for i in 0..points.len() {
        for j in i+1..points.len() {

            let p0 = points[i];
            let p1 = points[j];

            if dis(p0, p1) <= 3 && p0.4 != p1.4 {
                // merge constellations
                for k in 0..points.len() {
                    if points[k].4 == p1.4 {
                        points[k].4 = p0.4;
                    }
                }
            }
        }
    }

    let part1: HashSet<_> = points.iter().map(|x| x.4).collect();

    println!("Part 1: {}", part1.len());

    Ok(())
}
