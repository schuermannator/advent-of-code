//! Advent of Code 2020 Day 8
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let p1 = p1(&buf)?;
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2(&buf, p1 as u64)?);
    Ok(())
}

fn p1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut prev = VecDeque::new();
    // can I change this to like a .take(25) loop and then another loop?
    for (i, l) in input.lines().enumerate() {
        if i < 25 {
            prev.push_back(l.parse::<u32>()?);
        } else {
            let n = l.parse::<u32>()?;
            if !tester(&prev, n) {
                return Ok(n);
            }
            prev.push_back(n);
            prev.pop_front();
        }
    }
    Ok(0) // FIXME
}

fn tester(set: &VecDeque<u32>, n: u32) -> bool {
    for s in set {
        if n - s != n && set.contains(&(n - s)) {
            return true;
        }
    }
    return false;
}

fn p2(input: &str, sum: u64) -> Result<u64, &str> {
    let v: Vec<_> = input
        .lines()
        .map(|l| l.parse::<u64>().expect("failed to parse u64"))
        .collect();
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            let slice = &v[i..=j];
            if slice.iter().any(|n| n >= &sum) {
                continue;
            }
            if slice.iter().sum::<u64>() == sum {
                return Ok(slice.iter().min().expect("no min") + slice.iter().max().expect("no max"))
            }
        }
    }
    Err("couldnt find sum")
}
