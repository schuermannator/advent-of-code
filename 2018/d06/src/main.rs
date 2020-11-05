use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    p1(&buffer)?;
    Ok(())
}

fn p1(input: &str) -> io::Result<()> {
    let mut xmax = 0;
    let mut ymax = 0;
    for l in input.lines() {
        let (x, y) = parse_line(l);
        if x > xmax {
            xmax = x;
        }
        if y > ymax {
            ymax = y;
        }
    }

    let grid = vec![vec![0; xmax as usize]; ymax as usize];

    Ok(())
}

fn parse_line(line: &str) -> (u32, u32) {
    let mut i = line.split(',');
    let x: u32 = i.next().unwrap().parse().expect("failed to parse");
    let y: u32 = i.next().unwrap().parse().expect("failed to parse");
    (x, y)
}
