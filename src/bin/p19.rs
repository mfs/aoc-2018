extern crate failure;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use failure::{Error, format_err};

macro_rules! reg_op {
    ($id:ident, $op:tt) => (
        fn $id(a: u64, b: u64, c: u64, before: &Vec<u64>) -> Vec<u64> {
            let mut r = before.clone();
            r[c as usize] = r[a as usize] $op r[b as usize];
            r
        }
    )
}

reg_op!(ops_addr, +);
reg_op!(ops_mulr, *);
reg_op!(ops_banr, &);
reg_op!(ops_borr, |);

macro_rules! imm_op {
    ($id:ident, $op:tt) => (
        fn $id(a: u64, b: u64, c: u64, before: &Vec<u64>) -> Vec<u64> {
            let mut r = before.clone();
            r[c as usize] = r[a as usize] $op b;
            r
        }
    )
}

imm_op!(ops_addi, +);
imm_op!(ops_muli, *);
imm_op!(ops_bani, &);
imm_op!(ops_bori, |);

fn ops_setr(a: u64, _: u64, c: u64, before: &Vec<u64>) -> Vec<u64> {
    let mut r = before.clone();
    r[c as usize] = r[a as usize];
    r
}

fn ops_seti(a: u64, _: u64, c: u64, before: &Vec<u64>) -> Vec<u64> {
    let mut r = before.clone();
    r[c as usize] = a;
    r
}

macro_rules! cmp_ir_op {
    ($id:ident, $op:tt) => (
        fn $id(a: u64, b: u64, c: u64, before: &Vec<u64>) -> Vec<u64> {
            let mut r = before.clone();
            if a $op r[b as usize] {
                r[c as usize] = 1;
            } else {
                r[c as usize] = 0;
            }
            r
        }
    )
}

cmp_ir_op!(ops_gtir, >);
cmp_ir_op!(ops_eqir, ==);

macro_rules! cmp_ri_op {
    ($id:ident, $op:tt) => (
        fn $id(a: u64, b: u64, c: u64, before: &Vec<u64>) -> Vec<u64> {
            let mut r = before.clone();
            if r[a as usize] $op b {
                r[c as usize] = 1;
            } else {
                r[c as usize] = 0;
            }
            r
        }
    )
}

cmp_ri_op!(ops_gtri, >);
cmp_ri_op!(ops_eqri, ==);

macro_rules! cmp_rr_op {
    ($id:ident, $op:tt) => (
        fn $id(a: u64, b: u64, c: u64, before: &Vec<u64>) -> Vec<u64> {
            let mut r = before.clone();
            if r[a as usize] $op r[b as usize] {
                r[c as usize] = 1;
            } else {
                r[c as usize] = 0;
            }
            r
        }
    )
}

cmp_rr_op!(ops_gtrr, >);
cmp_rr_op!(ops_eqrr, ==);

#[derive(Copy, Clone)]
enum Op {
    Bani(u64, u64, u64),
    Gtri(u64, u64, u64),
    Seti(u64, u64, u64),
    Eqir(u64, u64, u64),
    Eqrr(u64, u64, u64),
    Borr(u64, u64, u64),
    Bori(u64, u64, u64),
    Banr(u64, u64, u64),
    Muli(u64, u64, u64),
    Eqri(u64, u64, u64),
    Mulr(u64, u64, u64),
    Gtrr(u64, u64, u64),
    Setr(u64, u64, u64),
    Addr(u64, u64, u64),
    Gtir(u64, u64, u64),
    Addi(u64, u64, u64),
}

fn main() -> Result<(), Error> {
    let  file = BufReader::new(File::open("data/p19.txt")?);

    let mut program = vec![];

    for line in file.lines() {
        let line = line?;
        if !line.starts_with("#") {
            let el: Vec<_> = line.split(" ").collect();

            let op = el[0];
            let a: u64 = el[1].parse()?;
            let b: u64 = el[2].parse()?;
            let c: u64 = el[3].parse()?;

            let x = match op {
                "bani" => Op::Bani(a, b, c),
                "gtri" => Op::Gtri(a, b, c),
                "seti" => Op::Seti(a, b, c),
                "eqir" => Op::Eqir(a, b, c),
                "eqrr" => Op::Eqrr(a, b, c),
                "borr" => Op::Borr(a, b, c),
                "bori" => Op::Bori(a, b, c),
                "banr" => Op::Banr(a, b, c),
                "muli" => Op::Muli(a, b, c),
                "eqri" => Op::Eqri(a, b, c),
                "mulr" => Op::Mulr(a, b, c),
                "gtrr" => Op::Gtrr(a, b, c),
                "setr" => Op::Setr(a, b, c),
                "addr" => Op::Addr(a, b, c),
                "gtir" => Op::Gtir(a, b, c),
                "addi" => Op::Addi(a, b, c),
                _ => panic!("AHHHHH")
            };

            program.push(x);
        }
    }

    const IP_REG: u64 = 5;

    let mut registers = vec![0u64, 0, 0, 0, 0, 0];

    let mut ip = 0u64;

    loop {
        let op = program[ip as usize];

        registers[IP_REG as usize] = ip;

        registers = match op {
            Op::Bani(a, b, c) => ops_bani(a, b, c, &registers),
            Op::Gtri(a, b, c) => ops_gtri(a, b, c, &registers),
            Op::Seti(a, b, c) => ops_seti(a, b, c, &registers),
            Op::Eqir(a, b, c) => ops_eqir(a, b, c, &registers),
            Op::Eqrr(a, b, c) => ops_eqrr(a, b, c, &registers),
            Op::Borr(a, b, c) => ops_borr(a, b, c, &registers),
            Op::Bori(a, b, c) => ops_bori(a, b, c, &registers),
            Op::Banr(a, b, c) => ops_banr(a, b, c, &registers),
            Op::Muli(a, b, c) => ops_muli(a, b, c, &registers),
            Op::Eqri(a, b, c) => ops_eqri(a, b, c, &registers),
            Op::Mulr(a, b, c) => ops_mulr(a, b, c, &registers),
            Op::Gtrr(a, b, c) => ops_gtrr(a, b, c, &registers),
            Op::Setr(a, b, c) => ops_setr(a, b, c, &registers),
            Op::Addr(a, b, c) => ops_addr(a, b, c, &registers),
            Op::Gtir(a, b, c) => ops_gtir(a, b, c, &registers),
            Op::Addi(a, b, c) => ops_addi(a, b, c, &registers),
        };

        ip = registers[IP_REG as usize];

        ip += 1;

        if ip > program.len() as u64 {
            println!("Part 1: {}", registers[0]);
            break;
        }
    }

    // found by experimenting
    const REGISTER: u64 = 10551267;

    // part2 is calculating sum of factors of REGISTER
    let part2 = REGISTER + (1..REGISTER/2).filter(|x| REGISTER % x == 0).sum::<u64>();
    println!("Part 2: {}", part2);

    Ok(())
}
