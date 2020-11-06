//! AoC 2018: Day 7
//! This is like topological sorting

use std::io::{self, Read};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

/// adjacency list for graph
#[derive(Default)]
struct Graph {
    nodes: Vec<char>,
    edges: HashMap<char, Vec<char>>
}

impl Graph {
    fn add_edge(&mut self, from: char, to: char) {
        if !self.nodes.contains(&from) {
            self.nodes.push(from);    
        }
        if !self.nodes.contains(&to) {
            self.nodes.push(to);    
        }
        self.edges.entry(from).or_default().push(to);
    }

    fn find_roots(&self) -> Vec<char> {
        let mut candidates = self.nodes.clone();
        for (_node, edges) in &self.edges {
            for e in edges {
                let i = candidates.iter().position(|x| x == e);
                if let Some(i) = i {
                    candidates.remove(i);
                }
            }
        }
        candidates
    }

    // fn dfs(&self, n: char, &mut visited: Vec<char>) {
    //     if visited.contains(&n) {
    //         return; 
    //     }
    //     visited.push(n);
    //     for child in self.edges.get(&n) {
    //         self.dfs(n, visited);
    //     }
    //     
    // }
}

///   -->A--->B--
///  /    \      \
/// C      -->D----->E
///  \           /
///   ---->F-----
/// 
/// CABDFE

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let input = buf;
    let mut g = Graph::default();
    for l in input.lines() {
        let (from, to) = parse_line(l);
        g.add_edge(from, to);
    }

    let roots = g.find_roots();
    println!("found roots: {:?}", roots);

    // let mut available = BinaryHeap::from(roots);
    let mut available = BinaryHeap::new();
    for r in roots {
        available.push(Reverse(r));
    }
    let mut out = String::new();
    loop {
        // pick lowest available
        println!("AVAILABLE {:?}", available);
        let next_move = available.pop();
        println!("POP {:?}\n", next_move);
        match next_move {
            Some(Reverse(next_move)) => {
                out.push(next_move);
                // add children to available
                let children = g.edges.get_mut(&next_move);
                if let Some(children) = children {
                    for child in children {
                        if !out.contains(*child) {
                            available.push(Reverse(*child));
                        }
                    }
                }
            },
            None => break,
        };
    }
    println!("{}", out);
    Ok(())
}

fn parse_line(line: &str) -> (char, char) {
    let mut it = line.split_whitespace();
    it.next(); // skip first word
    let from = it.next().unwrap();
    for _ in 0..5 { // skip 5 words
        it.next();
    }
    let to = it.next().unwrap();
    (from.chars().next().unwrap(), to.chars().next().unwrap())
}
