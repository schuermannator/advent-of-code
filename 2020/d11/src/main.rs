//! Advent of Code 2020 Day 11
use std::io::{self, Read};
use d11::{p1, p2};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let test = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
    println!("test: {}", p2(test)?);
    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

