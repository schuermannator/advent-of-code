//! Advent of Code 2020 Day 8
use std::{io::{self, Read}, str::FromStr, num::ParseIntError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    //println!("test: {}", p2("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6")?);
    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let inst = it.next().expect("no instruction read");
        let n = it.next().expect("no instruction arg found").parse::<i32>()?;
        Ok(match inst {
            "nop" => Instruction { kind: Inst::NOP, val: n },
            "acc" => Instruction { kind: Inst::ACC, val: n },
            "jmp" => Instruction { kind: Inst::JMP, val: n },
            _ => panic!("unexpected instruction"),
        })
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Inst {
    NOP,
    ACC,
    JMP,
}

#[derive(Clone)]
struct Instruction {
    kind: Inst,
    val: i32,
}

fn p1(input: &str) -> Result<i32, &str> {
    let program = input.lines().map(|l| l.parse::<Instruction>().expect("failed to parse Instruction")).collect::<Vec<_>>();
    let mut visited = vec![];
    let mut pc = 0;
    let mut acc = 0;
    while !visited.contains(&pc) {
        let inst = &program[pc];
        visited.push(pc);
        match inst.kind {
            Inst::NOP => pc += 1,
            Inst::ACC => {
                acc += inst.val;
                pc += 1;
            },
            Inst::JMP => pc += inst.val as usize, // TODO better cast/check
        }
    }
    Ok(acc)
}

/// lol brute force why not I love memory
fn p2(input: &str) -> Result<i32, &str> {
    let original = input.lines().map(|l| l.parse::<Instruction>().expect("failed to parse Instruction")).collect::<Vec<_>>();
    let mut programs = vec![];
    for (i, inst) in original.iter().enumerate() {
        if inst.kind == Inst::JMP {
            let mut mutated = original.clone();
            mutated[i].kind = Inst::NOP;
            programs.push(mutated);
        } else if inst.kind == Inst::NOP {
            let mut mutated = original.clone();
            mutated[i].kind = Inst::JMP;
            programs.push(mutated);
        }
    }
    for program in programs {
        if let Ok(acc) = test_program(program) {
            return Ok(acc);
        }
    }
    Err("no valid program found")
}

// TODO investigate 'static str in result (why static)
fn test_program(program: Vec<Instruction>) -> Result<i32, &'static str> {
    let mut visited = vec![];
    let mut pc = 0;
    let mut acc = 0;
    while !visited.contains(&pc) {
        if pc == program.len() {
            return Ok(acc);
        }
        let inst = &program[pc];
        visited.push(pc);
        match inst.kind {
            Inst::NOP => pc += 1,
            Inst::ACC => {
                acc += inst.val;
                pc += 1;
            },
            Inst::JMP => pc += inst.val as usize, // TODO better cast/check
        }
    }
    Err("looped")
}

