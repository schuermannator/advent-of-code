/// AoC2018 Day 5
use std::io::{self, Read};
use std::mem;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    p1(&buffer)?;
    // p2(&buffer)?;
    Ok(())
}

fn p1(input: &str) -> io::Result<()> {
    let mut polymer: Vec<u8> = input.as_bytes().into();
    let mut polymer_swap = vec![];
    loop {
        let mut reacted = false;
        let mut i = 0;
        while i < polymer.len() - 1 {
            if does_react(polymer[i], polymer[i + 1]) {
                reacted = true;
                i += 2;
                continue;
            }
            polymer_swap.push(polymer[i]);
            i += 1;
        }

        if i == polymer.len() - 1 {
            polymer_swap.push(polymer[i]);
        }

        mem::swap(&mut polymer, &mut polymer_swap);
        polymer_swap.clear(); 

        if !reacted {
            break; // cannot reduce further
        }
    }

    let polymer = String::from_utf8(polymer).expect("failed to convert to UTF8 String");
    println!("{}", polymer);
    println!("len: {}", polymer.len());
    Ok(())
}

fn does_react(c1: u8, c2: u8) -> bool {
    if c1.is_ascii_alphabetic() && c2.is_ascii_alphabetic() {
        if c1 > c2 {
            c1 - c2 == 32
        } else {
            c2 - c1 == 32
        }
    } else {
        false
    }
}
