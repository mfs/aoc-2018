extern crate failure;
extern crate time;

use std::path::Path;
use time::Duration;
use failure::Error;

fn main() -> Result<(), Error> {
    let mut times = vec![];

    let programs: Vec<_> = (1..=25).map(|n| format!("target/release/p{}", n)).collect();

    for p in programs {
        if !Path::new(&p).exists() {
            continue;
        }
        println!("{}", p);
        let start = time::precise_time_ns();
        std::process::Command::new(&p).status()?;
        let end = time::precise_time_ns();
        times.push((p, end - start));
        println!();
    }

    println!("Times");
    println!("----------------------------------------");
    for (p, t) in &times {
        println!("{}: {}ms", p, Duration::nanoseconds(*t as i64).num_milliseconds());
    }

    let total = times.iter().map(|x| x.1).sum::<u64>();
    println!("\nTotal time: {}ms", Duration::nanoseconds(total as i64).num_milliseconds());

    Ok(())
}
