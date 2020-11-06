//! Advent of Code 2019 Day 3
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("test1: {}", p1("R8,U5,L5,D3\nU7,R6,D4,L4")?);
    println!(
        "test2: {}",
        p1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")?
    );
    println!(
        "test3: {}",
        p1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?
    );
    println!("Part 1: {}", p1(&buf)?);
    // 30
    println!("test1 (p2): {}", p2("R8,U5,L5,D3\nU7,R6,D4,L4")?);
    // 610
    println!(
        "test2 (p2): {}",
        p2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")?
    );
    // 410
    println!(
        "test3 (p2): {}",
        p2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?
    );
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

enum Action {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl FromStr for Action {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, num) = s.split_at(1);
        let num = num.parse::<u32>()?;
        match c {
            "U" => Ok(Action::Up(num)),
            "D" => Ok(Action::Down(num)),
            "L" => Ok(Action::Left(num)),
            "R" => Ok(Action::Right(num)),
            _ => {
                // TODO this is bad lol
                println!("PARSE ERROR");
                Ok(Action::Up(0))
            }
        }
    }
}

fn p1(input: &str) -> io::Result<u32> {
    let mut it = input.lines();
    let a: Vec<Action> = it
        .next()
        .expect("missing line 1")
        .split(',')
        .filter(|x| x.chars().all(|x| !x.is_whitespace()))
        .map(|i| Action::from_str(i).expect("failed to parse action"))
        .collect();
    let b: Vec<Action> = it
        .next()
        .expect("missing line 2")
        .split(',')
        .filter(|x| x.chars().all(|x| !x.is_whitespace()))
        .map(|i| Action::from_str(i).expect("failed to parse action"))
        .collect();

    let trail_a = trail(a);
    let trail_b = trail(b);
    let intersections = intersect(trail_a, trail_b);
    Ok(intersections
        .iter()
        .map(|p| dist(p))
        .min()
        .expect("no intersections"))
}

fn p2(input: &str) -> io::Result<u32> {
    let mut it = input.lines();
    let a: Vec<Action> = it
        .next()
        .expect("missing line 1")
        .split(',')
        .filter(|x| x.chars().all(|x| !x.is_whitespace()))
        .map(|i| Action::from_str(i).expect("failed to parse action"))
        .collect();
    let b: Vec<Action> = it
        .next()
        .expect("missing line 2")
        .split(',')
        .filter(|x| x.chars().all(|x| !x.is_whitespace()))
        .map(|i| Action::from_str(i).expect("failed to parse action"))
        .collect();

    let trail_a = dist_trail(a);
    let trail_b = dist_trail(b);
    let intersections = intersect_dist_trails(trail_a, trail_b);
    Ok(intersections
        .iter()
        .map(|(d1, d2)| *d1 + *d2)
        .min()
        .expect("no intersections"))
}

fn intersect_dist_trails(a: Vec<(Point, u32)>, b: Vec<(Point, u32)>) -> Vec<(u32, u32)> {
    let a: HashMap<Point, u32> = a.into_iter().collect();
    let b: HashMap<Point, u32> = b.into_iter().collect();

    let a_keys: HashSet<Point> = a.keys().cloned().collect();
    let b_keys: HashSet<Point> = b.keys().cloned().collect();
    a_keys
        .intersection(&b_keys)
        .map(|point| {
            (
                *a.get(point).expect(format!("failed to get {:?} from hashmap", point).as_ref()),
                *b.get(point).expect(format!("failed to get {:?} from hashmap", point).as_ref()),
            )
        })
        .collect()
}

/// TODO this isn't very DRY lol
fn dist_trail(actions: Vec<Action>) -> Vec<(Point, u32)> {
    let mut dist = 0;
    let mut pos = Point::default();
    let mut trail = vec![];
    for action in actions {
        match action {
            Action::Up(n) => {
                for y in 1..=n {
                    dist += 1;
                    trail.push((
                        Point {
                            x: pos.x,
                            y: pos.y + y as i32,
                        },
                        dist,
                    ));
                }
                pos.y += n as i32;
            }
            Action::Down(n) => {
                for y in 1..=n {
                    dist += 1;
                    trail.push((
                        Point {
                            x: pos.x,
                            y: pos.y - y as i32,
                        },
                        dist,
                    ));
                }
                pos.y -= n as i32;
            }
            Action::Left(n) => {
                for x in 1..=n {
                    dist += 1;
                    trail.push((
                        Point {
                            x: pos.x - x as i32,
                            y: pos.y,
                        },
                        dist,
                    ));
                }
                pos.x -= n as i32;
            }
            Action::Right(n) => {
                for x in 1..=n {
                    dist += 1;
                    trail.push((
                        Point {
                            x: pos.x + x as i32,
                            y: pos.y,
                        },
                        dist,
                    ));
                }
                pos.x += n as i32;
            }
        }
    }
    trail
}

fn trail(actions: Vec<Action>) -> Vec<Point> {
    let mut pos = Point::default();
    let mut trail = vec![];
    for action in actions {
        match action {
            Action::Up(n) => {
                for y in 1..=n {
                    trail.push(Point {
                        x: pos.x,
                        y: pos.y + y as i32,
                    });
                }
                pos.y += n as i32;
            }
            Action::Down(n) => {
                for y in 1..=n {
                    trail.push(Point {
                        x: pos.x,
                        y: pos.y - y as i32,
                    });
                }
                pos.y -= n as i32;
            }
            Action::Left(n) => {
                for x in 1..=n {
                    trail.push(Point {
                        x: pos.x - x as i32,
                        y: pos.y,
                    });
                }
                pos.x -= n as i32;
            }
            Action::Right(n) => {
                for x in 1..=n {
                    trail.push(Point {
                        x: pos.x + x as i32,
                        y: pos.y,
                    });
                }
                pos.x += n as i32;
            }
        }
    }
    trail
}

/// find intersections of two trails
/// TODO could have this step take a trail and a set of actions and simply return
/// the intersections as it's traversing
fn intersect(a: Vec<Point>, b: Vec<Point>) -> Vec<Point> {
    let a: HashSet<Point> = a.into_iter().collect();
    let b: HashSet<Point> = b.into_iter().collect();
    // TODO remove cloned - I don't like this (and remove Clone derive from Point)
    a.intersection(&b).cloned().collect()
}

fn dist(p: &Point) -> u32 {
    (p.x.abs() + p.y.abs()) as u32
}
