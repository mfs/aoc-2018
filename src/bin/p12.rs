extern crate failure;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::VecDeque;
use failure::{Error, err_msg};


fn generate(state: &mut String, rules: &HashMap<String, char>, generations: usize) -> i64 {
    let mut total = 0;

    let mut history = VecDeque::new();
    for i in 0..4 {
        history.push_back(i);
    }

    for g in 0..generations {
        let mut next = "...".to_string();
        for i in 2..state.len() - 2 {
            let slice = &state[i - 2..=i + 2];
            let c = match rules.get(slice) {
                Some('#') => '#',
                _ => '.',
            };
            next.push(c);
        }
        next.push_str("...");

        *state = next;

        total = 0;

        for (i, c) in state[g + 1..].chars().enumerate() {
            if c == '#' {
                total += -3 + i as i64;
            }
        }

        history.pop_front();
        history.push_back(total);

        let d1 = history[3] - history[2];
        let d2 = history[2] - history[1];
        let d3 = history[1] - history[0];

        if d1 == d2 && d2 == d3 {
            return total + (generations as i64 - g as i64 - 1) * d1;
        }
    }

    total
}

fn main() -> Result<(), Error> {
    let  file = BufReader::new(File::open("data/p12.txt")?);

    let mut rules = HashMap::new();

    let mut state = String::new();

    for line in file.lines() {
        let line = line?;

        if line.starts_with("initial state:") {
            let el = line.split(": ").nth(1).ok_or(err_msg("error parsing state"))?;
            state = "...".to_string();
            state.push_str(el);
            state.push_str("...");
        } else if line.contains("=>") {
            let el: Vec<_> = line.split(" => ").collect();
            rules.insert(el[0].to_string(), el[1].chars().nth(0).ok_or(err_msg("error empty"))?);
        }
    }

    let part1 = generate(&mut state.clone(), &rules, 20);

    println!("Part 1: {}", part1);

    let part2 = generate(&mut state, &rules, 50_000_000_000);

    println!("Part 2: {}", part2);

    Ok(())
}
