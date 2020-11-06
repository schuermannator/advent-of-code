//! Advent of Code 2019 Day 1
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("Part 1 sum: {}", p1(&buf)?);
    println!("Part 2 sum: {}", p2(&buf)?);
    Ok(())
}

fn p1(input: &str) -> io::Result<u32> {
    // imperative
    let mut sum = 0;
    for l in input.lines() {
        let mass: u32 = l.parse().expect("failed to parse mass to u32");
        sum += (mass / 3) - 2;
    }
    // functional
    let sum2 = input
        .lines()
        .map(|l| l.parse::<u32>().expect("failed to parse mass to u32") / 3 - 2)
        .sum();
    assert_eq!(sum, sum2);
    Ok(sum)
}

fn p2(input: &str) -> io::Result<u32> {
    let sum = input
        .lines()
        .map(|l| fuel_total(l.parse::<u32>().expect("failed to parse mass to u32")))
        .sum();
    Ok(sum)
}

fn fuel_total(mass: u32) -> u32 {
    let mut total = 0;
    let mut mass = mass;
    loop {
        let new = fuel(mass);
        if new == 0 {
            break;
        }
        total += new;
        mass = new;
    }
    total
}

fn fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2) 
}
