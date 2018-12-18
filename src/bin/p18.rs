extern crate failure;

use failure::{Error};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const SIZE: usize = 50;

fn count(x: i64, y: i64, grid: &Vec<Vec<char>>) -> (u64, u64) {
    let neighbours: Vec<(i64, i64)> = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1)
    ]
    .iter()
    .map(|(a, b)| (x + a, y + b))
    .filter(|(a, _)| a >= &0 && a < &(SIZE as i64))
    .filter(|(_, a)| a >= &0 && a < &(SIZE as i64))
    .collect();

    let mut trees = 0;
    let mut lumber = 0;

    for n in neighbours {
        match grid[n.1 as usize][n.0 as usize] {
            '|' => trees += 1,
            '#' => lumber += 1,
            _ => { },
        }
    }

    (trees, lumber)
}

fn generate(grid: &mut Vec<Vec<char>>) -> u64 {
    let mut new_grid = vec![vec!['.'; SIZE]; SIZE];

    let mut trees = 0;
    let mut lumber = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let counts = count(x as i64, y as i64, &grid);

            new_grid[y][x] = grid[y][x];

            if grid[y][x] == '.' && counts.0 >= 3 {
                new_grid[y][x] = '|';
            }

            if grid[y][x] == '|' && counts.1 >= 3 {
                new_grid[y][x] = '#';
            }

            if grid[y][x] == '#' && counts.1 >= 1 && counts.0 >= 1 {
                new_grid[y][x] = '#';
            } else if grid[y][x] == '#' {
                new_grid[y][x] = '.';
            }

            match new_grid[y][x] {
                '|' => trees += 1,
                '#' => lumber += 1,
                _ => {},
            }
        }
    }

    *grid = new_grid;

    trees * lumber
}

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("data/p18.txt")?);

    let mut grid: Vec<Vec<char>> = vec![];

    for line in file.lines() {
        grid.push(line?.chars().collect());
    }

    // MUL * PERIOD needs to be past where cycle starts
    // PERIOD found by inspection
    const MUL: usize = 15;
    const PERIOD: usize = 28;

    for i in 1.. {
        let x = generate(&mut grid);
        if i == 10 {
            println!("Part 1: {}", x);
        }

        if i == PERIOD * MUL + (1_000_000_000 % PERIOD) {
            println!("Part 2: {}", x);
            break;
        }
    }

    Ok(())
}
