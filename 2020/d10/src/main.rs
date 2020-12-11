//! Advent of Code 2020 Day 10
use std::{collections::HashSet, io::{self, Read}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("test: {}", p2(r#"28
10
15
5
1
11
7
19
6
12
4"#)?);
    println!("test2: {}", p2(r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#)?);
    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

fn p1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut jolts: Vec<_> = input.lines().map(|l| l.parse::<i32>().expect("couldnt parse i32")).collect();
    jolts.push(0);
    jolts.sort();
    jolts.push(jolts.get(jolts.len() - 1).expect("failed to get last element") + 3);
    let (mut ones, mut threes) = (0, 0);
    for i in 0..jolts.len() - 1 {
        let diff = jolts[i+1] - jolts[i];
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            2 => continue,
            _ => panic!("unexpected diff"),
        }
    }
    Ok(ones * threes)
}

/// kinda worked through this by reverse-engineering the prime factors of the second example
/// (2, 2, 2, 7, 7, 7, 7) -> I originally did 2^skippable, but this doesn't work b/c it over-counts
/// some options which aren't possible, ex: 1, 2, 3, 4, 5 => not just 2^5 possibilities since all
/// technically 'skippable' when considered individually
fn p2(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut jolts: Vec<_> = input.lines().map(|l| l.parse::<i32>().expect("couldnt parse i32")).collect();
    jolts.push(0);
    jolts.sort();
    let mut skippable = HashSet::new();
    for i in 0..jolts.len() {
        let three_away = jolts[i] + 3;
        let mut j = i + 2;
        while jolts.get(j).map_or(false, |jolt| jolt <= &three_away) {
            skippable.insert(jolts[j-1]);
            j += 1;
        }
    }
    let mut v = skippable.into_iter().collect::<Vec<_>>();
    v.sort();
    println!("{:?}", v);

    let (mut twos, mut sevens) = (0, 0);
    let mut i = 0;
    while i < v.len() {
        let j = i + 2;
        if j < v.len() && v[j] - v[i] == 2 {
            sevens += 1;
            i += 3;
        } else {
            twos += 1;
            i += 1;
        }
    }

    Ok(2u64.pow(twos) * 7u64.pow(sevens))
}

