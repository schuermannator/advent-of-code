//! Advent of Code 2020 Day 1
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("Part 1: {}", p1(&buf)?);
    Ok(())
}

fn p1(input: &str) -> io::Result<u32> {
    println!("here");
    Ok(sum)
}

