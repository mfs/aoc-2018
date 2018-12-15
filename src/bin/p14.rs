extern crate failure;

use failure::Error;

const PART_1: u64 = 1;
const PART_2: u64 = 2;

const INPUT: usize = 919901;
const INPUT_SLICE: [usize; 6] = [9, 1, 9, 9, 0, 1];

fn check_end(scoreboard: &Vec<usize>) -> bool {
    let input_len = INPUT_SLICE.len();
    let l = scoreboard.len();
    if l >= input_len  {
        if scoreboard[l - input_len..] == INPUT_SLICE {
            println!("Part 2: {}", l - input_len);
            return true;
        }
    }

    false
}

fn score(part: u64) {
    let mut scoreboard = vec![];
    scoreboard.push(3);
    scoreboard.push(7);

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        // create new recipes
        let x = scoreboard[elf1] + scoreboard[elf2];
        if x < 10 {
            scoreboard.push(x);
        } else {
            scoreboard.push(x / 10);
            if part == PART_2 && check_end(&scoreboard) {
                break;
            }
            scoreboard.push(x % 10);
        }
        if part == PART_2 && check_end(&scoreboard) {
            break;
        }
        // inc elves
        let l = scoreboard.len();
        let elf1_delta = 1 + scoreboard[elf1];
        let elf2_delta = 1 + scoreboard[elf2];

        elf1 = (elf1 + elf1_delta) % l;
        elf2 = (elf2 + elf2_delta) % l;

        if part == PART_1 && scoreboard.len() == INPUT + 10 {
            break;
        }
    }

    if part == PART_1 {
        print!("Part 1: ");
        for x in scoreboard[INPUT..].iter() {
            print!("{}", x);
        }

        println!();
    }
}

fn main() -> Result<(), Error> {

    score(PART_1);
    score(PART_2);

    Ok(())
}
