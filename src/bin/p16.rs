extern crate failure;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use failure::{Error, format_err};
use regex::Regex;

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

fn main() -> Result<(), Error> {
    let  file = BufReader::new(File::open("data/p16.txt")?);

    let mut lines = vec![];

    for line in file.lines() {
        lines.push(line?);
    }

    let re = Regex::new(r"(\d+),? (\d+),? (\d+),? (\d+)")?;

    let mut in_op = false;

    let mut before: Vec<u64> = vec![];
    let mut op: Vec<u64> = vec![];

    let mut total = 0;

    let ops: Vec<fn(u64, u64, u64, &Vec<u64>) -> Vec<u64>> = vec![
        ops_bani,
        ops_gtri,
        ops_seti,
        ops_eqir,
        ops_eqrr,
        ops_borr,
        ops_bori,
        ops_banr,
        ops_muli,
        ops_eqri,
        ops_mulr,
        ops_gtrr,
        ops_setr,
        ops_addr,
        ops_gtir,
        ops_addi,
    ];

    for line in &lines {
        if line.starts_with("Before:") {
            // read start
            in_op = true;
            let c = re.captures(&line).ok_or(format_err!("error parsing: {}", line))?;
            before = (1..5).filter_map(|x| c.get(x)).filter_map(|x| x.as_str().parse().ok()).collect();
            continue;
        } else if in_op {
            // read op
            let c = re.captures(&line).ok_or(format_err!("error parsing: {}", line))?;
            op = (1..5).filter_map(|x| c.get(x)).filter_map(|x| x.as_str().parse().ok()).collect();
            in_op = false;
            continue;
        } else if line.starts_with("After:") {
            // read end
            let c = re.captures(&line).ok_or(format_err!("error parsing: {}", line))?;
            let after: Vec<_> = (1..5).map(|x| c.get(x).unwrap().as_str().parse().unwrap()).collect();
            let mut matching_ops = 0;
            for operation in &ops {
                if operation(op[1], op[2], op[3], &before) == after {
                    matching_ops += 1;
                }
            }
            if matching_ops >= 3 {
                total += 1;
            }
        }
    }

    println!("Part 1: {}", total);

    const START_LINE: usize = 3246;

    let mut registers = vec![0, 0, 0, 0];

    for line in &lines[START_LINE..] {
            let c = re.captures(&line).ok_or(format_err!("error parsing: {}", line))?;
            let op: Vec<u64> = (1..5).filter_map(|x| c.get(x)).filter_map(|x| x.as_str().parse().ok()).collect();

            registers = ops[op[0] as usize](op[1], op[2], op[3], &registers);
    }

    println!("Part 2: {}", registers[0]);

    Ok(())
}
