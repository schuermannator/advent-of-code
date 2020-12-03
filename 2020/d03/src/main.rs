//! Advent of Code 2020 Day 3
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

fn p1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut trees = 0;
    let mut x = 0;
    let width: usize = input
        .lines()
        .next()
        .ok_or("failed to get first line")?
        .chars()
        .count();
    for l in input.lines() {
        let space = l.as_bytes()[x];
        if space == '#' as u8 {
            trees += 1;
        }
        x = (x + 3) % width;
    }
    Ok(trees)
}

fn p2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(traverse(1, 1, input)?
        * traverse(3, 1, input)?
        * traverse(5, 1, input)?
        * traverse(7, 1, input)?
        * traverse(1, 2, input)?)
}

fn traverse(right: usize, down: usize, input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut trees = 0;
    let mut x = 0;
    let width: usize = input
        .lines()
        .next()
        .ok_or("failed to get first line")?
        .chars()
        .count();
    for (i, l) in input.lines().enumerate() {
        if i % down != 0 {
            continue;
        }
        let space = l.as_bytes()[x];
        if space == '#' as u8 {
            trees += 1;
        }
        x = (x + right) % width;
    }
    Ok(trees)
}
