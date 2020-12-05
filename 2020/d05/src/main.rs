//! Advent of Code 2020 Day 4
use std::{str::FromStr, io::{self, Read}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    //println!("test 1: {:?}", parse_seat("FBFBBFFRLR"));
    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

struct Seat(u32, u32);

impl FromStr for Seat {
    type Err = (); // TODO add errors propogated from get_row/get_col
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seat(get_row(s), get_col(s)))
    }
}

fn p1(input: &str) -> Result<u32, &str> {
    let seats = input.lines().map(|l| l.parse::<Seat>().expect("failed to parse Seat"));
    let seat_ids = seats.map(|Seat(r, c)| (r * 8 + c));
    seat_ids.max().ok_or("couldn't find max")
}

fn p2(input: &str) -> Result<u32, &str> {
    let seats = input.lines().map(|l| l.parse::<Seat>().expect("failed to parse Seat"));
    let mut spots = [false; 1024];
    let seat_ids = seats.map(|Seat(r, c)| (r * 8 + c));
    for id in seat_ids {
        spots[id as usize] = true;
    }
    for (id, filled) in spots.iter().enumerate() {
        if !filled && spots[id.saturating_sub(1)] && spots[std::cmp::min(1023, id + 1)] {
            return Ok(id as u32); // TODO safer casting?
        }
    }
    Err("Couldn't find seat")
}

fn get_row(s: &str) -> u32 {
    let mut lower = 0;
    let mut upper = 127;
    for (i, c) in s.chars().enumerate() {
        if i == 6 {
            return match c {
                'F' => lower,
                'B' => upper,
                _ => panic!("idk"), // FIXME
            };
        }
        match c {
            'F' => upper = (upper + lower) / 2,
            'B' => lower = (upper + lower) / 2 + 1,
            _ => println!("ERROR: unexpected char in seat string"),
        }
    }
    println!("ERROR"); // TODO
    0
}

fn get_col(s: &str) -> u32 {
    let mut lower = 0;
    let mut upper = 7;
    for (i, c) in s.chars().enumerate() {
        if i < 7 {
            continue;
        }
        if i == 9 {
            return match c {
                'L' => lower,
                'R' => upper,
                _ => panic!("idk"), // FIXME
            };
        }
        match c {
            'L' => upper = (upper + lower) / 2,
            'R' => lower = (upper + lower) / 2 + 1,
            _ => println!("ERROR: unexpected char in seat string"),
        }
    }
    println!("ERROR"); // TODO
    0
}
