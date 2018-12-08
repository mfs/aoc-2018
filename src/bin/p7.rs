extern crate failure;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use failure::{Error, err_msg};

fn part1(available: &mut Vec<char>, rules: Vec<(char, char)>) {

    let mut out = String::new();
    loop {
        if available.is_empty() {
            break;
        }

        available.sort();
        available.dedup();

        let first = available[0];
        out.push(first);
        available.remove(0);

        let new: Vec<_> = rules.iter().filter(|(a, _)| *a == first).map(|(_,x)| x).collect();

        'outer: for c in new {
            let pre_req: Vec<_> = rules.iter().filter(|(_, a)| a == c).map(|(a, _)| a).collect();

            for d in pre_req {
                if !out.contains(*d) {
                    continue 'outer;
                }
            }
            available.push(*c);
        }
    }

    println!("Part 1: {}", out);
}

struct Worker {
    time: i64,
    task: Option<char>,
}

impl Worker {
    fn new() -> Self {
        Self {
            time: 0,
            task: None,
        }
    }
}

fn part2(available: &mut Vec<char>, rules: Vec<(char, char)>) {
    let mut tick = 0;
    let mut out = String::new();

    let mut workers: Vec<_> = (0..5).map(|_| Worker::new()).collect();

    loop {

        available.sort();
        available.dedup();

        for i in 0..5 {
            if workers[i].time == 0 {
                if let Some(c) = workers[i].task {
                    out.push(c);

                    let new: Vec<_> = rules.iter().filter(|(a, _)| *a == c).map(|(_,x)| x).collect();

                    'outer: for c in new {
                        let pre_req: Vec<_> = rules.iter().filter(|(_, a)| a == c).map(|(a, _)| a).collect();

                        for d in pre_req {
                            if !out.contains(*d) {
                                continue 'outer;
                            }
                        }
                        available.push(*c);
                    }
                }
                workers[i].task = None;

                if !available.is_empty() {
                    let first = available[0];
                    available.remove(0);
                    workers[i].task = Some(first);
                    workers[i].time = 60 + first as i64 - 64;
                }
            }
            workers[i].time = std::cmp::max(0, workers[i].time - 1);
        }

        if out.len() == 26 {
            break;
        }

        tick += 1;
    }

    println!("Part 2: {}", tick);
}

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("data/p7.txt")?);

    let mut rules = vec![];
    let mut steps = HashSet::new();

    for line in file.lines() {
        let line = line?;
        let step0 = line.chars().nth(5).ok_or(err_msg("line too short"))?;
        let step1 = line.chars().nth(36).ok_or(err_msg("line too short"))?;

        rules.push((step0, step1));
        steps.insert(step0);
        steps.insert(step1);
    }

    let right: HashSet<char> = rules.iter().map(|(_, x)| *x).collect();

    let mut available: Vec<char> = steps.difference(&right).map(|x| *x).collect();

    part1(&mut available.clone(), rules.clone());

    part2(&mut available, rules);

    Ok(())
}
