//! Advent of Code 2020 Day 12
use std::{
    io::{self, Read},
    num::ParseIntError,
    str::FromStr,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("test: {}", p2("F10\nN3\nF7\nR90\nF11")?);
    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

enum Move {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

enum Direction {
    N,
    S,
    E,
    W,
}

struct Boat {
    x: i32,
    y: i32,
    a: Direction,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Move {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, num) = s.split_at(1);
        match c {
            "N" => Ok(Move::N(num.parse::<i32>()?)),
            "S" => Ok(Move::S(num.parse::<i32>()?)),
            "E" => Ok(Move::E(num.parse::<i32>()?)),
            "W" => Ok(Move::W(num.parse::<i32>()?)),
            "L" => Ok(Move::L(num.parse::<i32>()?)),
            "R" => Ok(Move::R(num.parse::<i32>()?)),
            "F" => Ok(Move::F(num.parse::<i32>()?)),
            _ => panic!("unexpected move type"),
        }
    }
}

fn p1(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut boat = Boat {
        x: 0,
        y: 0,
        a: Direction::E,
    };
    let moves: Vec<Move> = input
        .lines()
        .map(|l| l.parse::<Move>().expect("failed to parse move"))
        .collect();

    for m in moves {
        match m {
            Move::N(n) => boat.y += n,
            Move::S(n) => boat.y -= n,
            Move::E(n) => boat.x += n,
            Move::W(n) => boat.y -= n,
            Move::L(n) => boat.a = calc_direction(boat.a, Move::L(n)),
            Move::R(n) => boat.a = calc_direction(boat.a, Move::R(n)),
            Move::F(n) => match boat.a {
                Direction::N => boat.y += n,
                Direction::S => boat.y -= n,
                Direction::E => boat.x += n,
                Direction::W => boat.x -= n,
            },
        }
    }
    Ok(boat.x.abs() + boat.y.abs())
}

fn p2(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut boat = Point { x: 0, y: 0 };
    let mut waypoint = Point { x: 10, y: 1 };
    let moves: Vec<Move> = input
        .lines()
        .map(|l| l.parse::<Move>().expect("failed to parse move"))
        .collect();

    for m in moves {
        match m {
            Move::N(n) => waypoint.y += n,
            Move::S(n) => waypoint.y -= n,
            Move::E(n) => waypoint.x += n,
            Move::W(n) => waypoint.x -= n,
            Move::L(n) => waypoint = calc_waypoint(waypoint, Move::L(n)),
            Move::R(n) => waypoint = calc_waypoint(waypoint, Move::R(n)),
            Move::F(n) => {
                for _ in 0..n {
                    boat.x += waypoint.x;
                    boat.y += waypoint.y;
                }
            }
        }
        println!("{:?}", boat);
        println!("{:?}\n", waypoint);
    }
    Ok(boat.x.abs() + boat.y.abs())
}

// E, L 90 => N
fn calc_direction(start: Direction, mov: Move) -> Direction {
    let angle = match start {
        Direction::N => 90,
        Direction::S => 270,
        Direction::E => 0,
        Direction::W => 180,
    };
    let change = match mov {
        Move::L(n) => n,
        Move::R(n) => -1 * n,
        _ => panic!("unsupported move in calc_direction"),
    };
    let mut new = angle + change;
    if new < 0 {
        new += 360;
    } else if new >= 360 {
        new -= 360;
    }
    match new {
        0 => Direction::E,
        90 => Direction::N,
        180 => Direction::W,
        270 => Direction::S,
        _ => panic!("calculated unsupported angle {}", new),
    }
}

fn calc_waypoint(waypoint: Point, mov: Move) -> Point {
    let theta = match mov {
        Move::L(n) => n,
        Move::R(n) => -1 * n,
        _ => panic!("unsupported move in calc_direction"),
    };

    // lookups for sine/cosine
    let sin = |t| {
        match t {
            90 => 1,
            180 => 0,
            270 => -1,
            -90 => -1,
            -180 => 0,
            -270 => 1,
            _ => panic!("lul"),
        }
    };

    let cos = |t| {
        match t {
            90 => 0,
            180 => -1,
            270 => 0,
            -90 => 0,
            -180 => -1,
            -270 => 0,
            _ => panic!("lul"),
        }
    };

    // 2D rotation matrix
    let x = waypoint.x * cos(theta) - waypoint.y * sin(theta);
    let y = waypoint.x * sin(theta) + waypoint.y * cos(theta);
    Point { x, y }
}
