use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    p1(&buffer)?;
    p2(&buffer)?;
    Ok(())
}

fn p1(b: &str) -> io::Result<()> {
    let mut s = 0;
    for line in b.lines() {
        s += line.parse::<i32>().expect("unable to parse");
    }

    println!("{}", s);
    Ok(())
}

fn p2(b: &str) -> io::Result<()> {
    let mut d = HashSet::new();
    let mut s = 0;
    d.insert(s);
    // could just loop { also
    for line in b.lines().into_iter().cycle() {
        let v = line.parse::<i32>().expect("unable to parse");
        s += v;
        if !d.insert(s) {
            println!("{}", s);
            return Ok(());
        }
    }
    println!("no repeated frequencies");
    Ok(())
}
