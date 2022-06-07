extern crate failure;
extern crate time;

use std::path::Path;
use time::{Instant, Duration};
use failure::Error;

fn main() -> Result<(), Error> {
    let mut times = vec![];

    let programs: Vec<_> = (1..=25).map(|n| format!("target/release/p{}", n)).collect();

    for p in programs {
        if !Path::new(&p).exists() {
            continue;
        }
        println!("{}", p);
        let start = Instant::now();
        std::process::Command::new(&p).status()?;
        times.push((p, start.elapsed()));
        println!();
    }

    println!("Times");
    println!("----------------------------------------");
    for (p, t) in &times {
        println!("{}: {}ms", p, t.whole_milliseconds());
    }

    let total: Duration = times.iter().map(|x| x.1).sum();
    println!("\nTotal time: {}ms", total.whole_milliseconds());

    Ok(())
}
