use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    p1(&buffer)?;
    p2(&buffer)?;
    Ok(())
}

fn p1(b: &str) -> Result<()> {
    let mut twos = 0;
    let mut threes = 0;
    for line in b.lines() {
        let mut chars = HashMap::new();
        for c in line.chars() {
            chars.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        if chars.values().find(|v| v == &&2).is_some() {
            twos += 1;
        }
        if chars.values().find(|v| v == &&3).is_some() {
            threes += 1;
        }
    }

    let checksum = twos * threes;
    println!("{}", checksum);
    Ok(())
}

fn p2(b: &str) -> Result<()> {
    let lines: Vec<_> = b.lines().collect();
    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            let sames = lines[i].chars().zip(lines[j].chars()).map(|(a, b)| a == b);
            let mut not_sames = sames.filter(|x| !*x);
            if not_sames.next().is_some() && not_sames.next().is_none() {
                println!(
                    "{}",
                    lines[i].chars()
                        .zip(lines[j].chars())
                        .filter(|(a, b)| a == b)
                        .map(|(a, _)| a)
                        .collect::<String>()
                );
                return Ok(());
            }
        }
    }

    Ok(())
}
