//! Advent of Code 2020 Day 2
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

fn p1(input: &str) -> io::Result<u32> {
    let mut valid = 0;
    for l in input.lines() {
        let mut chunks = l.split(&['-', ' ', ':'][..]);
        let lower: usize = chunks.next().expect("failed to get lower").parse().unwrap();
        let upper: usize = chunks.next().expect("failed to get upper").parse().unwrap();
        let c = chunks.next().expect("failed to get letter");
        chunks.next().expect("failed to get colon");
        let s = chunks.next().expect("failed to get rest");
        
        // println!("{}, {}, {}, {}", lower, upper, c, s);
       
        let count = s.matches(c).count();
        if count <= upper && count >= lower {
            valid += 1;
        }
    }
    Ok(valid)
}

fn p2(input: &str) -> io::Result<u32> {
    let mut valid = 0;
    for l in input.lines() {
        let mut chunks = l.split(&['-', ' ', ':'][..]);
        let lower: usize = chunks.next().expect("failed to get lower").parse().unwrap();
        let upper: usize = chunks.next().expect("failed to get upper").parse().unwrap();
        let c = chunks.next().expect("failed to get letter").as_bytes()[0];
        chunks.next().expect("failed to get colon");
        let s = chunks.next().expect("failed to get rest").as_bytes();
        
        if (s[lower - 1] == c && s[upper - 1] != c) || (s[lower - 1] != c && s[upper - 1] == c) {
            valid += 1;
        } 
    }
    Ok(valid)
}
