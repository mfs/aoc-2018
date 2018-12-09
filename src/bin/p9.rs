extern crate failure;

use std::collections::VecDeque;
use failure::{Error, err_msg};

fn process(num_marbles: usize, num_players: usize) -> Result<usize, Error> {

    let mut current_player = 1;
    let mut players = vec![0; num_players];
    let mut circle = VecDeque::new();
    circle.push_back(0);

    for x in 1..num_marbles {
        if x % 23 == 0 {
            players[current_player] += x;
            for _ in 0..7 {
                let last = circle.pop_back().ok_or(err_msg("empty buffer"))?;
                circle.push_front(last);
            }

            players[current_player] += circle.pop_front().ok_or(err_msg("empty buffer"))?;
        } else {
            for _ in 0..2 {
                let first = circle.pop_front().ok_or(err_msg("empty buffer"))?;
                circle.push_back(first);
            }
            circle.push_front(x);
        }

        current_player = (current_player + 1) % num_players;
    }

   Ok(*players.iter().max().ok_or(err_msg("empty scores"))?)
}


fn main() -> Result<(), Error> {

    const NUM_MARBLES: usize = 72060;
    const NUM_PLAYERS: usize = 410;

    let part1 = process(NUM_MARBLES, NUM_PLAYERS)?;
    println!("Part 1: {}", part1);

    let part2 = process(NUM_MARBLES * 100, NUM_PLAYERS)?;
    println!("Part 2: {}", part2);

    Ok(())
}
