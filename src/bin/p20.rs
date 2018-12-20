extern crate failure;

use failure::{err_msg, Error};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone)]
struct Room {
    x: i64,
    y: i64,
    distance: u64,
}

impl Room {
    fn new() -> Self {
        Room {
            x: 0,
            y: 0,
            distance: 0,
        }
    }
}

fn main() -> Result<(), Error> {
    let mut file = File::open("data/p20.txt")?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    // stack of rooms where we branch
    let mut stack = vec![];
    // current room
    let mut room = Room::new();
    // rooms x, y -> distances
    let mut grid: HashMap<(i64, i64), u64> = HashMap::new();

    for c in buffer {
        let c = c as char;

        match c {
            '(' => {
                // save branch
                stack.push(room);
            }
            ')' => {
                // restore branch
                room = stack.pop().ok_or(err_msg("mismatched )"))?;
            }
            '|' => {
                // reset branch
                room = *stack.last().ok_or(err_msg("missing ("))?;
            }
            'N' => room.y -= 1,
            'S' => room.y += 1,
            'E' => room.x += 1,
            'W' => room.x -= 1,
            _ => continue, // skip ^ and $
        }

        if ['N', 'S', 'E', 'W'].contains(&c) {
            room.distance += 1;
            grid.entry((room.x, room.y)).or_insert(room.distance);
        }
    }

    let part1 = grid.values().max().ok_or(err_msg("empty grid"))?;

    println!("Part 1: {}", part1);

    let part2 = grid.values().filter(|x| *x >= &1000).count();

    println!("Part 2: {}", part2);

    Ok(())
}
