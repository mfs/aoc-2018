extern crate failure;

use failure::Error;

const GRID_SERIAL: i64 = 2866;

fn power(x: i64, y: i64) -> i64 {
    let rack_id = x + 11;
    let mut power: i64 = rack_id * (y + 1);
    power += GRID_SERIAL;
    power *= rack_id;
    power = (power / 100) % 10;
    power -= 5;

    power
}

fn square(x: usize, y: usize, size: usize, grid: &Vec<Vec<i64>>) -> i64 {
    let mut total = 0;
    for i in 0..size {
        for j in 0..size {
            total += grid[x+i][y+j];
        }
    }

    total
}


fn main() -> Result<(), Error> {

    let mut grid = vec![ vec![0i64; 300]; 300];

    for x in 0..300 {
        for y in 0..300 {
            let power = power(x, y);
            grid[x as usize][y as usize] = power;
        }
    }

    let mut max_sum = 0;
    let mut co_ords = (0, 0);

    for x in 0..298 {
        for y in 0..298 {
            let sum = square(x, y, 3, &grid);

            if sum > max_sum {
                max_sum = sum;
                co_ords = (x, y);
            }
        }
    }

    println!("Part 1: {},{}", co_ords.0 + 1, co_ords.1 + 1);

    let mut max_sum = 0;
    let mut co_ords = (0, 0, 0);

    for x in 0..300 {
        for y in 0..300 {
            let max_size = std::cmp::min(300 - x , 300 - y);
            let mut sum = 0;

            for s in 0..max_size {
                for d in 0..s {
                    sum += grid[x + d][y + s];
                    sum += grid[x + s][y + d];
                }
                sum += grid[x+s][y+s];
                if sum > max_sum {
                    max_sum = sum;
                    co_ords = (x, y, s);
                }
            }
        }
    }

    println!("Part 2: {},{},{}", co_ords.0 + 1, co_ords.1 + 1, co_ords.2 + 1);

    Ok(())
}
