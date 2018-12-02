extern crate failure;
extern crate time;

use std::path::Path;
use failure::Error;

fn main() -> Result<(), Error> {
    let mut timestamps = vec![];

    let programs: Vec<_> = (1..21).map(|n| format!("target/release/p{}", n)).collect();

    timestamps.push(time::precise_time_ns());

    for p in &programs {
        if !Path::new(p).exists() {
            break;
        }
        println!("{}", p);
        std::process::Command::new(p).status()?;
        timestamps.push(time::precise_time_ns());
        println!();
    }

    println!("Times");
    println!("----------------------------------------");
    for (win, p) in timestamps.windows(2).zip(programs) {
        let d = time::Duration::nanoseconds((win[1] - win[0]) as i64);

        println!("{}: {}ms", p, d.num_milliseconds());
    }

    if let Some(last) = timestamps.last() {
        let total = time::Duration::nanoseconds((last - timestamps[0]) as i64);
        println!("\nTotal time: {}ms", total.num_milliseconds());
    }

    Ok(())
}
