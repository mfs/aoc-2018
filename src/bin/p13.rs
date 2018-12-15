extern crate failure;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use failure::Error;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}


#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Cart {
    y: usize,
    x: usize,
    direction: Direction,
    next_turn: Turn,
    is_alive: bool,
}

fn main() -> Result<(), Error> {
    let  file = BufReader::new(File::open("data/p13.txt")?);

    let mut grid: Vec<Vec<char>> = vec![];

    for line in file.lines() {
        let line = line?;

        grid.push(line.chars().collect());
    }

    let mut carts = vec![];

    for y in 0..150 {
        for x in 0..150 {
            let c = grid[y][x];
            let square = match c {
                '^' => Some(Cart {x: x, y: y, direction: Direction::Up, next_turn: Turn::Left, is_alive: true }),
                'v' => Some(Cart {x: x, y: y, direction: Direction::Down, next_turn: Turn::Left, is_alive: true }),
                '<' => Some(Cart {x: x, y: y, direction: Direction::Left, next_turn: Turn::Left, is_alive: true }),
                '>' => Some(Cart {x: x, y: y, direction: Direction::Right, next_turn: Turn::Left, is_alive: true }),
                _   => None,
            };

            if let Some(cart) = square {
                carts.push(cart);
            }

        }
    }

    let num_carts = carts.len();
    let mut collisions = vec![];

    'outer: loop {
        carts.sort();
        for i in 0..carts.len() {
            // move cart
            match carts[i].direction {
                Direction::Up => { carts[i].y -= 1 },
                Direction::Down => { carts[i].y += 1 },
                Direction::Left => { carts[i].x -= 1 },
                Direction::Right => { carts[i].x += 1 },
            }

            // check collisions
            for j in 0..carts.len() {
                if carts[i].x == carts[j].x && carts[i].y == carts[j].y && i != j && carts[j].is_alive && carts[i].is_alive {
                    if collisions.is_empty() {
                        println!("Part 1: {} {}", carts[i].x, carts[j].y);
                    }
                    collisions.push((carts[i].x, carts[i].y));
                    carts[i].is_alive = false;
                    carts[j].is_alive = false;
                }
            }

            // adjust direction corners, intersections
            let c = grid[carts[i].y][carts[i].x];

            match c {
                '+' => {
                    match carts[i].next_turn {
                        Turn::Left => { carts[i].direction = carts[i].direction.left() },
                        Turn::Right => { carts[i].direction = carts[i].direction.right() },
                        Turn::Straight => {},
                    }
                    carts[i].next_turn = carts[i].next_turn.next();
                },
                '/' => {
                    carts[i].direction = match carts[i].direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                },
                '\\' => {
                    carts[i].direction = match carts[i].direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                },
                _ => {},
            }
        }

        // last survivor
        if collisions.len() * 2 == num_carts - 1 {
            for j in 0..carts.len() {
                if carts[j].is_alive {
                    println!("Part 2: {} {}", carts[j].x, carts[j].y);
                }
            }
            break 'outer;
        }
    }

    Ok(())
}
