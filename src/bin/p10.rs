extern crate failure;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use failure::{Error, format_err};

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Point {
    fn new(x: i64, y: i64, vx: i64, vy: i64) -> Point {
        Point {x, y, vx, vy}
    }

    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

fn main() -> Result<(), Error> {
    let  file = BufReader::new(File::open("data/p10.txt")?);

    let re = regex::Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>")?;

    let mut points = vec![];

    for line in file.lines() {
        let line = line?;

        let c = re.captures(&line).ok_or(format_err!("error parsing {}", line))?;

        let p = Point::new(c[1].parse()?, c[2].parse()?, c[3].parse()?, c[4].parse()?);

        points.push(p);
    }

    let mut seconds = 0;

    const MIN_HEIGHT: i64 = 9; // found by experimentation

    loop {
        let mut max_x = -100000;
        let mut max_y = -100000;
        let mut min_x =  100000;
        let mut min_y =  100000;

        for p in points.iter_mut() {
            p.update();
            max_x = std::cmp::max(max_x, p.x);
            max_y = std::cmp::max(max_y, p.y);
            min_x = std::cmp::min(min_x, p.x);
            min_y = std::cmp::min(min_y, p.y);
        }

        seconds += 1;

        if max_y - min_y == MIN_HEIGHT {
            println!("Part 1:");
            let mut pixels = vec![ vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
            for p in points {
                pixels[(p.y - min_y) as usize][(p.x - min_x) as usize] = 1;
            }
            for l in pixels {
                for r in l {
                    match r {
                        0 => print!(" "),
                        _ => print!("#"),
                    }
                }
                println!();
            }
            break;
        }

    }

    println!("Part 2: {}", seconds);

    Ok(())
}
