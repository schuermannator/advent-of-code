use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    p1(&buf)?;
    p2(&buf)?;

    Ok(())
}

fn p1(buf: &str) -> Result<()> {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 10000]; 10000];
    for line in buf.lines() {
        let (x, y, w, h) = parse_line(line)?;
        for i in x..x + w {
            for j in y..y + h {
                grid[i][j] += 1;
            }
        }
    }

    let mut shared_spaces = 0;
    for row in grid {
        for x in row {
            if x > 1 {
                shared_spaces += 1;
            }
        }
    }
    println!("{}", shared_spaces);
    Ok(())
}

fn p2(buf: &str) -> Result<()> {
    let mut grid: Vec<Vec<Option<u32>>> = vec![vec![None; 10000]; 10000];
    let mut full_claims = HashSet::new();
    for line in buf.lines() {
        let (x, y, w, h) = parse_line(line)?;
        let claim = parse_claim(line)?;
        full_claims.insert(claim);
        for i in x..x + w {
            for j in y..y + h {
                match grid[i][j] {
                    Some(c) => {
                        full_claims.remove(&c);
                        full_claims.remove(&claim);
                    }
                    None => grid[i][j] = Some(claim),
                };
            }
        }
    }

    println!("{}", full_claims.drain().next().unwrap());
    Ok(())
}

// input:
// #12 @ 901,802: 12x24
// parse:
// (901, 802, 12, 24)
fn parse_line(l: &str) -> Result<(usize, usize, usize, usize)> {
    let mut i = l.split_whitespace();
    let mut wh = i.next_back().unwrap().split('x');
    let mut xy = i.next_back().unwrap().split(|c| c == ',' || c == ':');

    let x = xy.next().unwrap().parse()?;
    let y = xy.next().unwrap().parse()?;
    let w = wh.next().unwrap().parse()?;
    let h = wh.next().unwrap().parse()?;

    Ok((x, y, w, h))
}

// input:
// #12 @ 901,802: 12x24
// parse:
// 12
fn parse_claim(l: &str) -> Result<(u32)> {
    let mut i = l.split_whitespace();
    let claim = i.next().unwrap().trim_matches('#');
    Ok(claim.parse()?)
}
