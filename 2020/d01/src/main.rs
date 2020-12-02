//! Advent of Code 2020 Day 1
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

// brute force
fn p1(input: &str) -> io::Result<i32> {
    let nums: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().expect("failed on parse i32"))
        .collect();
    for i in 0..nums.len() {
        let n = nums[i]; 
        for j in 0..nums.len() {
            let n2 = nums[j];
            if i == j {
                continue;
            }
            if n + n2 == 2020 {
                return Ok(n * n2);
            }
        }
    }
    Ok(-1)
}

#[allow(dead_code)]
fn p2_bf(input: &str) -> io::Result<i32> {
    let nums: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().expect("failed on parse i32"))
        .collect();
    for i in 0..nums.len() {
        let n = nums[i]; 
        for j in 0..nums.len() {
            let n2 = nums[j];
            if i == j {
                continue;
            }
            for k in 0..nums.len() {
                let n3 = nums[k];
                if i == k || j == k {
                    continue;
                }
                if n + n2 + n3 == 2020 {
                    return Ok(n * n2 * n3);
                }
            }
        }
    }
    Ok(-1)
}

fn p2(input: &str) -> io::Result<i32> {
    let nums: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().expect("failed on parse i32"))
        .collect();
    for n1 in &nums {
        let needles_sum = 2020 - n1;
        for n2 in &nums {
            let needle = needles_sum - n2;
            if n1 == n2 {
                continue;
            }
            if let Ok(_) = nums.binary_search(&needle) {
                return Ok(n1 * n2 * needle);
            }
        }
    }
    Ok(-1)
}

