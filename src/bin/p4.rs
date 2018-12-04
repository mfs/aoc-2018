extern crate failure;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use failure::{Error, format_err, err_msg};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    year: u64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    action: String,
}

struct Graph {
    guard: u64,
    minutes: HashSet<u8>,
}

fn parse() -> Result<Vec<Graph>, Error> {
    let file = BufReader::new(File::open("data/p4.txt")?);

    let re  = regex::Regex::new(r"\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] (.+)")?;

    let mut entries = vec![];

    for line in file.lines() {
        let line = line?;
        let c = re.captures(&line).ok_or(format_err!("error parsing: {}", line))?;

        if c.len() != 7 {
            return Err(format_err!("error parsing: {}", line));
        }

        let e = Entry {
            year: c[1].parse()?,
            month: c[2].parse()?,
            day: c[3].parse()?,
            hour: c[4].parse()?,
            minute: c[5].parse()?,
            action: c[6].to_string(),
        };

        entries.push(e);
    }

    entries.sort();

    let re = regex::Regex::new(r"#(\d+) ")?;

    let mut graphs: Vec<Graph> = vec![];

    let mut sleep_start = 0;

    for x in &entries {
        if x.action == "falls asleep" {
            sleep_start = x.minute;
        } else if x.action == "wakes up" {
            for x in sleep_start..x.minute {
                let g = graphs.last_mut().ok_or(err_msg("error: missing graph"))?;
                g.minutes.insert(x);
            }
        } else {
            let c = re.captures(&x.action).ok_or(format_err!("error parsing: {}", &x.action))?;
            let current_guard = c[1].parse()?;
            graphs.push(
                Graph {
                    guard: current_guard,
                    minutes: HashSet::new(),
                }
            );
        }
    }

    Ok(graphs)
}

fn part1(graphs: &Vec<Graph>) {

    // make map guard => minutes
    let mut sleep_times = HashMap::new();
    for g in graphs {
        *sleep_times.entry(g.guard).or_insert(0) += g.minutes.iter().count();
    }

    // convert to tuple (guard, minutes), reverse sort, guard is first
    let mut guard_times: Vec<_> = sleep_times.iter().collect();

    guard_times.sort_unstable_by(|(_,b), (_,a)| a.cmp(b));

    let guard = *guard_times[0].0;

    // make map minutes => count
    let mut minutes = HashMap::new();

    for x in graphs.iter().filter(|x| x.guard == guard) {
        for m in 0..60 {
            if x.minutes.contains(&m) {
                *minutes.entry(m).or_insert(0) += 1;
            }
        }
    }

    // convert to tuple (minute, count), reverse sort, minute is first
    let mut minutes_count: Vec<_> = minutes.iter().collect();

    minutes_count.sort_unstable_by(|(_,b), (_,a)| a.cmp(b));

    let minute = *minutes_count[0].0;

    println!("Part 1: {}", guard * minute as u64);
}


fn part2(graphs: &Vec<Graph>) {

    // make set of all guards
    let mut guards = HashSet::new();
    for g in graphs {
        guards.insert(g.guard);
    }

    // generate vec of tupples (guard, minute, count)
    let mut times = vec![];

    for m in 00..60 {
        for guard in &guards {
            let mut count = 0;
            for g in graphs.iter().filter(|x| x.guard == *guard) {
                if g.minutes.contains(&m) {
                    count += 1;
                }
            }
            times.push((guard, m, count));
        }
    }

    // reverse sort, first is highest
    times.sort_unstable_by(|(_, _, b), (_, _, a)| a.cmp(b));

    println!("Part 2: {}", *times[0].0 * times[0].1 as u64);
}

fn main() -> Result<(), Error> {

    let graphs = parse()?;

    part1(&graphs);
    part2(&graphs);

    Ok(())
}
