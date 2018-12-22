extern crate failure;

use std::collections::HashMap;
use std::collections::VecDeque;
use failure::Error;

// scan M * TARGET
const M: u64 = 4;

const DEPTH: u64 = 3066;
const TARGET: (u64, u64) = (13, 726);

struct Cave {
    regions: HashMap<(u64, u64), u64>,
}

impl Cave {
    fn new() -> Self {
        Cave {
            regions: HashMap::new(),
        }
    }

    fn init(&mut self) {
        for i in 0..TARGET.1 * M {
            for j in 0..TARGET.0 * M {
                let h = self.gi(j, i);
                self.regions.insert((j, i), h);
                let v = self.gi(i, j);
                self.regions.insert((i, j), v);
            }
        }
    }

    fn gi(&self, x: u64, y: u64) -> u64 {
        if x == 0 && y == 0 {
            return 0;
        } else if x == TARGET.0 && y == TARGET.1 {
            return 0;
        } else if y == 0 {
            return x * 16807;
        } else if x == 0 {
            return y * 48271;
        } else {
            return self.el(x - 1, y) * self.el(x, y - 1);
        }
    }

    fn el(&self, x: u64, y:u64) -> u64 {
        (self.regions[&(x, y)] + DEPTH) % 20183
    }

    fn region_type(&self, x: u64, y: u64) -> u64 {
        self.el(x, y) % 3
    }

    fn part1(&self) {
        let mut risk_level = 0;

        for y in 0..=TARGET.1 {
            for x in 0..=TARGET.0 {
                risk_level += self.region_type(x, y);
            }
        }

        println!("Part 1: {}", risk_level);
    }

    fn part2(&self) {
        let mut queue: VecDeque<(u64, u64, u64, u64)> = VecDeque::new();

        let mut dist: HashMap<(u64, u64, u64), u64> = HashMap::new();

        queue.push_back((0, 0, 0, 1));
        dist.insert((0, 0, 1), 0);

        let mut min_time = 0;

        loop {
            if queue.is_empty() {
                break;
            }

            let (d, x, y, e) = queue.pop_front().unwrap();

            if (x, y, e) == (TARGET.0, TARGET.1, 1) {
                min_time = d;
            }

            if x > 3 * TARGET.0 || y > 3 * TARGET.1 {
                continue;
            }

            if *dist.get(&(x, y, e)).unwrap() < d {
                continue;
            }

            for (nx, ny, ne, time) in self.neighbours(x as i64, y as i64, e) {
                if d + time < *dist.get(&(nx, ny, ne)).unwrap_or(&1000000000) {
                    dist.insert((nx, ny, ne), d + time);
                    queue.push_back((d + time, nx, ny, ne));
                }
            }
        }

        println!("Part 2: {}", min_time);
    }

    fn neighbours(&self, x: i64, y: i64, e: u64) -> Vec<(u64, u64, u64, u64)> {
        let mut neighbours = vec![];

        let deltas: Vec<_> = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        for delta in deltas.iter().filter(|(x, y)| *x >= 0 && *y >= 0) {
            let r = self.region_type(x as u64, y as u64);
            for ne in 0..3 {
                let mut time = 1;
                if ne != e {
                    time += 7;
                }
                if r != ne && r != e {
                    neighbours.push((delta.0 as u64, delta.1 as u64, ne, time));
                }
            }
        }
        neighbours
    }
}

fn main() -> Result<(), Error> {

    let mut cave = Cave::new();

    cave.init();

    cave.part1();
    cave.part2();

    Ok(())
}

