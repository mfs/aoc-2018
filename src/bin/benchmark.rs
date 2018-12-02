extern crate failure;
extern crate time;

use std::path::Path;
use time::Duration;
use failure::Error;

fn main() -> Result<(), Error> {
    let mut times = vec![];

    let programs: Vec<_> = (1..21).map(|n| format!("target/release/p{}", n)).collect();

    for p in &programs {
        if !Path::new(p).exists() {
            break;
        }
        println!("{}", p);
        let start = time::precise_time_ns();
        std::process::Command::new(p).status()?;
        let end = time::precise_time_ns();
        times.push(end - start);
        println!();
    }

    println!("Times");
    println!("----------------------------------------");
    for (t, p) in times.iter().zip(programs) {
        println!("{}: {}ms", p, Duration::nanoseconds(*t as i64).num_milliseconds());
    }

    let total = times.iter().sum::<u64>();
    println!("\nTotal time: {}ms", Duration::nanoseconds(total as i64).num_milliseconds());

    Ok(())
}
