// Rust implementation of https://www.reddit.com/r/adventofcode/comments/a6wpup/2018_day_17_solutions/ebyq6mj/

extern crate failure;
extern crate regex;

use failure::{err_msg, format_err, Error};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Pt = (i64, i64);

fn in_range(x: i64, a0: i64, a1: i64) -> bool {
    x >= a0 && x <= a1
}

struct State {
    clay: HashSet<Pt>,
    settled: HashSet<Pt>,
    flowing: HashSet<Pt>,
    max_y: i64,
    min_y: i64,
}

impl State {
    fn new(clay: HashSet<Pt>) -> Result<Self, Error> {
        let max_y = clay
            .iter()
            .map(|(_, y)| *y)
            .max()
            .ok_or(err_msg("empty set"))?;
        let min_y = clay
            .iter()
            .map(|(_, y)| *y)
            .min()
            .ok_or(err_msg("empty set"))?;
        Ok(State {
            clay,
            settled: HashSet::new(),
            flowing: HashSet::new(),
            max_y,
            min_y,
        })
    }
    fn fill(&mut self, pt: Pt, dir: Pt) -> bool {
        macro_rules! clay {
            ($x:expr) => {
                self.clay.contains($x)
            };
        }
        macro_rules! settled {
            ($x:expr) => {
                self.settled.contains($x)
            };
        }
        macro_rules! flowing {
            ($x:expr) => {
                self.flowing.contains($x)
            };
        }

        self.flowing.insert(pt);

        let below = (pt.0, pt.1 + 1);

        if !clay!(&below) && !flowing!(&below) && in_range(below.1, 1, self.max_y) {
            self.fill(below, (0, 1));
        }

        if !clay!(&below) && !settled!(&below) {
            return false;
        }

        let mut left = (pt.0 - 1, pt.1);
        let mut right = (pt.0 + 1, pt.1);

        let left_filled = clay!(&left) || !flowing!(&left) && self.fill(left, (-1, 0));
        let right_filled = clay!(&right) || !flowing!(&right) && self.fill(right, (1, 0));

        if dir == (0, 1) && left_filled && right_filled {
            self.settled.insert(pt);

            while flowing!(&left) {
                self.settled.insert(left);
                left.0 = left.0 - 1;
            }

            while flowing!(&right) {
                self.settled.insert(right);
                right.0 = right.0 + 1;
            }
        }

        dir == (-1, 0) && (left_filled || clay!(&left))
            || dir == (1, 0) && (right_filled || clay!(&right))
    }

    fn report(&self) {
        let part1 = self .flowing
            .union(&self.settled)
            .filter(|x| in_range(x.1, self.min_y, self.max_y))
            .count();
        println!("Part 1: {}", part1);

        let part2 = self .settled
            .iter()
            .filter(|x| in_range(x.1, self.min_y, self.max_y))
            .count();
        println!("Part 2: {}", part2);
    }
}

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("data/p17.txt")?);

    let re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)(..)?(\d+)?")?;

    let mut clay: HashSet<Pt> = HashSet::new();

    for line in file.lines() {
        let line = line?;
        let c = re
            .captures(&line)
            .ok_or(format_err!("error parsing {}", line))?;
        if c.len() == 5 {
            // single
            let pt = (c[2].parse()?, c[4].parse()?);
            if c[1] == *"x" {
                clay.insert(pt);
            } else if c[1] == *"y" {
                clay.insert((pt.1, pt.0));
            }
        } else if c.len() == 7 {
            // range
            let a: i64 = c[2].parse()?;
            let b0: i64 = c[4].parse()?;
            let b1: i64 = c[6].parse()?;
            if c[1] == *"x" {
                for y in b0..=b1 {
                    clay.insert((a, y));
                }
            } else if c[1] == *"y" {
                for x in b0..=b1 {
                    clay.insert((x, a));
                }
            }
        }
    }

    let mut state = State::new(clay)?;

    state.fill((500, 0), (0, 1));
    state.report();

    Ok(())
}
