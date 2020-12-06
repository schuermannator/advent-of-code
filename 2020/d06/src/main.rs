//! Advent of Code 2020 Day 6
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

fn p1(input: &str) -> Result<u32, &str> {
    let bitvecs = input.split("\n\n").map(|group| {
        let mut questions: u32 = 0;
        assert!(group.is_ascii());
        for c in group.bytes() {
            if c == 10 || c == 32 {
                continue; // skip ascii space/newline
            }
            let i = c - b'a';
            questions |= 1 << i;
        }
        questions
    });

    let counts = bitvecs.map(|bv| bv.count_ones());
    Ok(counts.sum())
}

fn p2(input: &str) -> Result<u32, &str> {
    let bitvecs = input.split("\n\n").map(|group| {
        assert!(group.is_ascii());
        let mut bvs = group.lines().map(|p|  {
            let mut qs: u32 = 0;
            for c in p.chars() {
                if c.is_whitespace() {
                    continue;
                }
                let i =  c as u8 - b'a'; // TODO safer casting
                qs |= 1 << i;
            }
            qs
        });
        let first = bvs.next().expect("no bitvecs in group");
        bvs.fold(first, |acc, x| acc & x) // should be fold_first
    });

    let counts = bitvecs.map(|bv| bv.count_ones());
    Ok(counts.sum())
}
