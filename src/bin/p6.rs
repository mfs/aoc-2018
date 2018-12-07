extern crate failure;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use failure::{Error};

fn dis(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    (x0 - x1).abs() + (y0 - y1).abs()
}

fn around(min_x: i64, min_y: i64, max_x: i64, max_y: i64, points: &Vec<(i64, i64)>) -> HashMap<i64, i64> {

    let mut totals: HashMap<i64, i64> = HashMap::new();

    for x in min_x..max_x {
        for y in min_y..max_y {

            let distances: Vec<_> = points.iter().map(|(x0, y0)| dis(*x0, *y0, x, y)).collect();

            let mut distances: Vec<_> = distances.iter().enumerate().collect();

            distances.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

            if distances[0].1 < distances[1].1 {
                *totals.entry(distances[0].0 as i64).or_insert(0) += 1;
            }
        }
    }

    totals
}

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("data/p6.txt")?);

    let mut points = vec![];

    let mut min_x: i64 = 1000;
    let mut min_y: i64 = 1000;
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;

    for line in file.lines() {
        let line = line?;

        let pos: Vec<i64> = line.split(", ").filter_map(|x| x.parse::<i64>().ok()).collect();

        max_x = std::cmp::max(max_x, pos[0]);
        max_y = std::cmp::max(max_y, pos[1]);

        min_x = std::cmp::min(min_x, pos[0]);
        min_y = std::cmp::min(min_y, pos[1]);

        points.push((pos[0], pos[1]));
    }

    // part 1
    let mut s: Vec<_> = around(min_x, min_y, max_x, max_y, &points)
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();

    let mut t: Vec<_> = around(min_x - 10, min_y - 10, max_x + 10, max_y + 10, &points)
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();

    s.sort_unstable_by(|(_, b), (_, a)| a.cmp(b));
    t.sort_unstable_by(|(_, b), (_, a)| a.cmp(b));

    for i in 0..s.len() {
        if s[i].1 == t[i].1 {
            println!("Part 1: {}", s[i].1);
            break;
        }
    }

    // part 2
    let mut total = 0;

    for x in min_x..max_x {
        for y in min_y..max_y {

            let sum: i64 = points.iter().map(|(x0, y0)| dis(*x0, *y0, x, y)).sum();

            if sum < 10000 {
                total += 1;
            }
        }
    }

    println!("Part 2: {}", total);

    Ok(())
}
