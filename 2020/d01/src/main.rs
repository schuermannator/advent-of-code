//! Advent of Code 2020 Day 1
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

fn p1(input: &str) -> io::Result<i32> {
    let nums: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().expect("failed on parse i32"))
        .collect();
    // nums.sort_unstable();
    // for i in 0..nums.len() {
    //     let n = nums[i];
    //     for j in nums.len()
    // }
    // Ok(5)
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

fn p2(input: &str) -> io::Result<i32> {
    let nums: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().expect("failed on parse i32"))
        .collect();
    // nums.sort_unstable();
    // for i in 0..nums.len() {
    //     let n = nums[i];
    //     for j in nums.len()
    // }
    // Ok(5)
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

