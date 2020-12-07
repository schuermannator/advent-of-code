//! Advent of Code 2020 Day 7
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    // println!("test: {}", p2("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.")?);
    // expect p1 = 4
    // expect p2 = 32
    println!("Part 1: {}", p1(&buf)?);
    println!("Part 2: {}", p2(&buf)?);
    Ok(())
}

// A
// | \
// B  C
// nodes: A, B, C
// f_edges: [[1, 2]]
// b_edges: [[], [0], [0]]
#[derive(Debug)]
struct Graph {
    nodes: Vec<String>,
    edges: Vec<Vec<usize>>, // TODO remove (only need back edges)
    b_edges: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct WGraph {
    nodes: Vec<String>,
    edges: Vec<Vec<(usize, u32)>>,
}

impl WGraph {
    // idempotent
    fn add_node(&mut self, n: String) -> usize {
        match self.nodes.iter().enumerate().find(|(_, node)| node == &&n) {
            None => {
                self.edges.push(Vec::new());
                self.nodes.push(n);
                self.nodes.len() - 1
            }
            Some((i, _)) => i,
        }
    }

    // adds the "to" node if needed
    fn add_child(&mut self, from: usize, to: String, num: u32) {
        let to = self.add_node(to);
        self.edges[from].push((to, num));
    }

    fn calc_weight(&self, node: usize) -> u32 {
        self.edges[node].iter().map(|(n, w)| self.calc_weight(*n) * w).sum::<u32>() + 1
    }
}

impl Graph {
    // idempotent
    fn add_node(&mut self, n: String) -> usize {
        match self.nodes.iter().enumerate().find(|(_, node)| node == &&n) {
            None => {
                self.edges.push(Vec::new());
                self.b_edges.push(Vec::new());
                self.nodes.push(n);
                self.nodes.len() - 1
            }
            Some((i, _)) => i,
        }
    }

    // adds the "to" node if needed
    fn add_child(&mut self, from: usize, to: String) {
        let to = self.add_node(to);
        self.edges[from].push(to);
        self.b_edges[to].push(from);
    }

    fn rev_dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![];
        self.rec_dfs(start, &mut visited);
        visited
    }

    fn rec_dfs(&self, start: usize, visited: &mut Vec<usize>) {
        for child in self.b_edges[start].iter() {
            if !visited.contains(child) {
                visited.push(*child);
                self.rec_dfs(*child, visited);
            }
        }
    }
}

fn p1(input: &str) -> Result<usize, &str> {
    let mut g = Graph {
        nodes: vec![],
        edges: vec![],
        b_edges: vec![],
    };
    for rule in input.lines() {
        let mut it = rule.split(" bags contain ");
        let this_bag = String::from(it.next().ok_or("couln't parse root bag")?);
        //dbg!(this_bag.clone());
        let root = g.add_node(this_bag);
        let contents = it
            .next()
            .ok_or("couldnt parse root bag")?
            .split(',')
            .map(|s| s.trim().trim_end_matches('.'));
        for bag in contents {
            let mut it = bag.split_whitespace();
            let n = it
                .next()
                .ok_or("failed to split whitespace in contents bag")?;
            let bag = bag.trim_start_matches(|c: char| c.is_ascii_digit() || c.is_whitespace());
            let bag = bag.trim_end_matches(" bag").trim_end_matches(" bags");
            g.add_child(root, String::from(bag));
            //dbg!(n, bag);
        }
        //dbg!(&g);
        //dbg!();
    }

    let (start_index, _) = g
        .nodes
        .iter()
        .enumerate()
        .find(|(_, n)| *n == "shiny gold")
        .expect("couldnt find shiny gold");
    //dbg!(start_index);
    let visited = g.rev_dfs(start_index);
    //dbg!(g);
    //dbg!(visited);
    Ok(visited.len())
}

fn p2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut g = WGraph {
        nodes: vec![],
        edges: vec![],
    };
    for rule in input.lines() {
        let mut it = rule.split(" bags contain ");
        let this_bag = String::from(it.next().ok_or("couln't parse root bag")?);
        //dbg!(this_bag.clone());
        let root = g.add_node(this_bag);
        let contents = it
            .next()
            .ok_or("couldnt parse root bag")?
            .split(',')
            .map(|s| s.trim().trim_end_matches('.'));
        for bag in contents {
            if bag.contains("no other bag") {
                continue;
            }
            let mut it = bag.split_whitespace();
            let n = it
                .next()
                .ok_or("failed to split whitespace in contents bag")?
                .parse::<u32>()?;
            let bag = bag.trim_start_matches(|c: char| c.is_ascii_digit() || c.is_whitespace());
            let bag = bag.trim_end_matches(" bag").trim_end_matches(" bags");
            g.add_child(root, String::from(bag), n);
            //dbg!(n, bag);
        }
        //dbg!(&g);
        //dbg!();
    }

    let (start_index, _) = g
        .nodes
        .iter()
        .enumerate()
        .find(|(_, n)| *n == "shiny gold")
        .expect("couldnt find shiny gold");
    //dbg!(start_index);
    let w = g.calc_weight(start_index) - 1; // remove our bag's weight
    //dbg!(g);
    //dbg!(visited);
    //Ok(visited.len())
    Ok(w)
}
