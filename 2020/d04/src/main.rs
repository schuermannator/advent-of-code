//! Advent of Code 2020 Day 4
use std::{str::FromStr, io::{self, Read}};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

fn p1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut valid = 0;
    for passport in input.split("\n\n") {
        let keys: Vec<&str> = passport
            .split_ascii_whitespace()
            .map(|field| field.split(':').next().expect("failed get field key"))
            .collect();
        let mut do_valid = 1; // FIXME
        for required in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
            if !keys.contains(&required) {
                do_valid = 0;
            }
        }
        valid += do_valid;
    }
    Ok(valid)
}

enum Units {
    Cm,
    In,
}

struct Height {
    n: u32,
    units: Units,
}

// FIXME
#[derive(Debug)]
struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl std::error::Error for MyError {}

impl FromStr for Height {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // WRONG
        // match s.trim_end_matches(|p| p == 'i' || p == 'n').parse::<u32>() {
        //     Ok(inches) => Ok(Height { n: inches, units: Units::In }),
        //     Err(_) => {
        //         let cm = s.trim_end_matches(|p| p == 'c' || p == 'm').parse::<u32>()?;
        //         Ok(Height { n: cm, units: Units::Cm })
        //     }
        // }
        let re = Regex::new(r"(\d{2,3})(in|cm)").unwrap();
        let caps = re.captures(s).ok_or("no caps")?;
        let n = caps.get(1).unwrap().as_str().parse::<u32>()?;
        match caps.get(2).unwrap().as_str() {
            "in" => Ok(Height { n, units: Units::In }),
            "cm" => Ok(Height { n, units: Units::Cm }),
            _ => Err(Box::new(MyError(String::from("oops"))))
        }
    }
}

impl Height {
    fn is_valid(&self) -> bool {
        match self.units {
            Units::Cm => self.n >= 150 && self.n <= 193,
            Units::In => self.n >= 59 && self.n <= 76,
        }
    }
}

fn p2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut valid = 0;
    for passport in input.split("\n\n") {
        let fields: Vec<(&str, &str)> = passport
            .split_ascii_whitespace()
            .map(|field| {
                let mut i = field.split(':');
                (i.next().expect("failed get field key"), i.next().expect("failed to get field val"))
            })
            .collect();
        let mut ok = 1;
        let keys = fields.iter().map(|f| f.0).collect::<Vec<_>>();

        for f in fields {
            match f.0 {
                "byr" => {
                    if let Ok(byr) = f.1.parse::<u32>() {
                        if !(byr >= 1920 && byr <= 2002) {
                            ok = 0;
                            break;
                        }
                    } else { // FIXME
                        ok = 0;
                        break;
                    }
                },
                "iyr" => {
                    if let Ok(iyr) = f.1.parse::<u32>() {
                        if !(iyr >= 2010 && iyr <= 2020) {
                            ok = 0;
                            break;
                        }
                    } else {
                        ok = 0;
                        break;
                    }
                },
                "eyr" => {
                    if let Ok(eyr) = f.1.parse::<u32>() {
                        if !(eyr >= 2020 && eyr <= 2030) {
                            ok = 0;
                            break;
                        }
                    } else {
                        ok = 0;
                        break;
                    }
                },
                "hgt" => {
                    if let Ok(h) = f.1.parse::<Height>() {
                        if !h.is_valid() {
                            ok = 0;
                            break;
                        }
                    } else {
                        ok = 0;
                        break;
                    }
                },
                "hcl" => {
                    let re = Regex::new(r"^#[0-9,a-f]{6}$").unwrap();
                    if re.find(f.1).is_none() {
                        ok = 0;
                        break;
                    }
                },
                "ecl" => {
                    let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
                    if re.find(f.1).is_none() {
                        ok = 0;
                        break;
                    }
                },
                "pid" => {
                    let re = Regex::new(r"^\d{9}$").unwrap();
                    if re.find(f.1).is_none() {
                        ok = 0;
                        break;
                    }
                },
                _ => continue
            }
        }

        for required in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
            if !keys.contains(&required) {
                ok = 0;
                break;
            }
        }
        
        valid += ok;
    }
    Ok(valid)
}
