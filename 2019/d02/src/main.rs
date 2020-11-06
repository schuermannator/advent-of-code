//! Advent of Code 2019 Day 2
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    // clean input (strip ending whitespace)
    buf.retain(|c| !c.is_whitespace());

    println!(
        "Part 1: {}",
        p1(&buf)?
            .get(0)
            .expect("unable to get position zero from program")
    );
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

fn p1(input: &str) -> io::Result<Vec<i32>> {
    let mut program: Vec<i32> = input
        .split(',')
        .map(|x| {
            x.parse::<i32>()
                .expect(format!("failed to parse element {} in program to i32", x).as_ref())
        })
        .collect();
    let mut pc = 0;
    program[1] = 12;
    program[2] = 2;
    while step_program(&mut pc, &mut program) {}
    Ok(program)
}

fn p2(input: &str) -> io::Result<i32> {
    let program: Vec<i32> = input
        .split(',')
        .map(|x| {
            x.parse::<i32>()
                .expect(format!("failed to parse element {} in program to i32", x).as_ref())
        })
        .collect();

    for n in 0..=99 {
        for v in 0..=99 {
            if run_program(&program, n, v) == 19690720 {
                return Ok(100 * n + v);
            }
        }
    }
    Ok(-1)
}

fn run_program(program: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut p = program.clone();
    let mut pc = 0;
    p[1] = noun;
    p[2] = verb;
    while step_program(&mut pc, &mut p) {}
    *p.get(0).expect("unable to get pos zero from program")
}

fn step_program(pc: &mut usize, program: &mut Vec<i32>) -> bool {
    let op = program.get(*pc).expect("failed to get opcode from PC");
    match op {
        // add
        1 => {
            let i = program[*pc + 1];
            let j = program[*pc + 2];
            let res = program[i as usize] + program[j as usize];
            let pos = program[*pc + 3];
            program[pos as usize] = res;
            *pc += 4;
            true
        }
        // mult
        2 => {
            let i = program[*pc + 1];
            let j = program[*pc + 2];
            let res = program[i as usize] * program[j as usize];
            let pos = program[*pc + 3];
            program[pos as usize] = res;
            *pc += 4;
            true
        }
        // halt
        99 => false,

        // error parsing
        _ => {
            println!("failed to parse opcode {}", op);
            false
        }
    }
}
