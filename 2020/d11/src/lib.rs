use std::{fmt::Display, str::FromStr};

/// note: from AoC reddit (u/sporksmith and u/LinAGKar)
///
/// Mine is ~33ms in a release build for part 2. The only thing I see that
/// might explain the difference is your potentially hitting the allocator (which can end up being
/// a syscall) in your loop's Vec and HashSet operations. Creating all of those with_capacity of
/// rows * cols might help a bit, though I would be surprised if it made that big of a difference
/// TBH.
/// 
/// with_capacity didn't make a significant difference. I did gain a little by precalculating
/// adjacent seats (especially in part 2), and using a HashMap of bools rather than a HashSet.
/// However, something that gave it a major performance boost was to get rid of the hashing from
/// the main loop altogether, and instead storing the occupied state in a Vec<Vec<bool>>, so
/// getting/setting the occupied state requires just a simple offset lookup rather than hashing.
/// This speed it up by an order of magnitude, and both parts now run in 10-20 ms.

#[derive(Debug, Clone, Eq, PartialEq)]
enum Space {
    Floor,
    Occupied,
    Empty,
}

struct Grid {
    spaces: Vec<Space>,
    w: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> &Space {
        &self.spaces[y * self.w + x]
    }

    /// return vec of up to eight spots on cardinals + diagonals
    fn get_adjacent(&self, i: usize) -> Vec<&Space> {
        let x = i % self.w;
        let y = i / self.w;
        let mut adj = vec![];

        // TODO
        // // cardinals
        // for yi in &[1, -1] {
        //     // x, y+yi
        //     self.get(x, y+yi);
        // }
        // for xi in &[1, -1] {
        //     // x+xi, y
        // }
        // // diagonals
        // for xi in &[1, -1] {
        //     for yi in &[1, -1] {
        //         // x+xi, y+yi
        //
        //     }
        // }

        let h = self.spaces.len() / self.w;

        if x > 0 {
            adj.push(self.get(x - 1, y));
            if y > 0 {
                adj.push(self.get(x - 1, y - 1));
            }
        }

        if y > 0 {
            adj.push(self.get(x, y - 1));
            if x < self.w - 1 {
                adj.push(self.get(x + 1, y - 1));
            }
        }

        if x < self.w - 1 {
            adj.push(self.get(x + 1, y));
            if y < h - 1 {
                adj.push(self.get(x + 1, y + 1));
            }
        }

        if y < h - 1 {
            adj.push(self.get(x, y + 1));
            if x > 0 {
                adj.push(self.get(x - 1, y + 1));
            }
        }

        adj
    }

    fn get_adjacent2(&self, i: usize) -> Vec<&Space> {
        let mut adj = vec![];

        // TODO better way to do this?
        let down_edge_ok = |j| j < self.spaces.len();
        let left_edge_ok = |j| j >= i - (i % self.w);
        let right_edge_ok = |j| j < i + (self.w - i % self.w);

        // up - after edge hits zero we break (edge is i >= 0)
        let mut j = i;
        loop { 
            if let Some(jj) = j.checked_sub(self.w) {
                j = jj;
                let s = &self.spaces[j];
                if s != &Space::Floor {
                    adj.push(s);
                    break;
                }
            } else {
                break;
            }
        }
        // down - edge is i < self.spaces.len()
        let mut j = i;
        loop { 
            j += self.w;
            if !down_edge_ok(j) {
                break;
            }
            let s = &self.spaces[j];
            if s != &Space::Floor {
                adj.push(s);
                break;
            }
        }
        // left - edge is i > i - i % self.w
        let mut j = i;
        loop { 
            if let Some(jj) = j.checked_sub(1) {
                j = jj;
                if !left_edge_ok(j) {
                    break;
                }
                let s = &self.spaces[j];
                if s != &Space::Floor {
                    adj.push(s);
                    break;
                }
            } else {
                break;
            }
        }
        // right - edge is j < i + (self.w - i % self.w)
        let mut j = i;
        loop { 
            j += 1;
            if !right_edge_ok(j) {
                break;
            }
            let s = &self.spaces[j];
            if s != &Space::Floor {
                adj.push(s);
                break;
            }
        }
        // LU diag - edge is left && up
        let mut j = i;
        for _ in 0..(i % self.w) { 
            if let Some(jj) = j.checked_sub(self.w + 1) {
                j = jj;
                let s = &self.spaces[j];
                if s != &Space::Floor {
                    adj.push(s);
                    break;
                }
            } else {
                break;
            }
        }
        // RU diag - edge is right && up
        let mut j = i;
        for _ in 0..self.w - (i % self.w)  - 1 {
            if let Some(jj) = j.checked_sub(self.w - 1) {
                j = jj;
                let s = &self.spaces[j];
                if s != &Space::Floor {
                    adj.push(s);
                    break;
                }
            } else {
                break;
            }
        }
        // LD diag - edge is left && down
        let mut j = i;
        for _ in 0..(i % self.w) {
            j += self.w - 1;
            if !down_edge_ok(j) {
                break;
            }
            let s = &self.spaces[j];
            if s != &Space::Floor {
                adj.push(s);
                break;
            }
        }
        // RD diag - edge is right && down
        let mut j = i;
        for _ in 0..self.w - (i % self.w) - 1 {
            j += self.w + 1;
            if !down_edge_ok(j) {
                break;
            }
            let s = &self.spaces[j];
            if s != &Space::Floor {
                adj.push(s);
                break;
            }
        }

        adj
    }


    fn run_sim(&mut self) -> bool {
        let mut new = self.spaces.clone();
        for (i, s) in self.spaces.iter().enumerate() {
            match s {
                Space::Empty => {
                    if self.no_occupied_adjacent(i) {
                        new[i] = Space::Occupied;
                    }
                }
                Space::Occupied => {
                    if self.four_plus_occupied(i) {
                        new[i] = Space::Empty;
                    }
                }
                _ => continue,
            }
        }
   
        std::mem::replace(&mut self.spaces, new) != self.spaces
    }

    fn run_sim2(&mut self) -> bool {
        let mut new = self.spaces.clone();
        for (i, s) in self.spaces.iter().enumerate() {
            match s {
                Space::Empty => {
                    if self.no_occupied_adjacent2(i) {
                        new[i] = Space::Occupied;
                    }
                }
                Space::Occupied => {
                    if self.n_plus_occupied(i, 5) {
                        new[i] = Space::Empty;
                    }
                }
                _ => continue,
            }
        }
   
        std::mem::replace(&mut self.spaces, new) != self.spaces
    }


    fn no_occupied_adjacent(&self, i: usize) -> bool {
        let adjacent = self.get_adjacent(i);
        adjacent.iter().all(|&s| s != &Space::Occupied)
    }

    fn no_occupied_adjacent2(&self, i: usize) -> bool {
        let adjacent = self.get_adjacent2(i);
        adjacent.iter().all(|&s| s != &Space::Occupied)
    }

    fn four_plus_occupied(&self, i: usize) -> bool {
        let adjacent = self.get_adjacent(i);
        adjacent.iter().filter(|&&s| s == &Space::Occupied).count() >= 4
    }

    fn n_plus_occupied(&self, i: usize, n: usize) -> bool {
        let adjacent = self.get_adjacent2(i);
        adjacent.iter().filter(|&&s| s == &Space::Occupied).count() >= n
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let height = self.spaces.len() / self.w;
        for y in 0..height {
            for x in 0..self.w {
                match self.get(x, y) {
                    Space::Empty => write!(f, "L")?,
                    Space::Occupied => write!(f, "#")?,
                    Space::Floor => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// TODO can we do a FromChar or something? so we avoid the char.to_string().parse() below
impl FromStr for Space {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Space::Floor),
            "#" => Ok(Space::Occupied),
            "L" => Ok(Space::Empty),
            _ => Err("unknown char encountered".to_string()),
        }
    }
}

pub fn p1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let width = input
        .lines()
        .next()
        .expect("failed to get first line")
        .trim()
        .len();
    // represent grid as 1D vector
    let spaces: Vec<Space> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            c.to_string()
                .parse::<Space>()
                .expect("failed to parse Space")
        })
        .collect();

    let mut grid = Grid { spaces, w: width };
    while grid.run_sim() {}

    //println!("{}", grid);

    Ok(grid
        .spaces
        .iter()
        .filter(|&s| s == &Space::Occupied)
        .count())
}

pub fn p2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let width = input
        .lines()
        .next()
        .expect("failed to get first line")
        .trim()
        .len();
    // represent grid as 1D vector
    let spaces: Vec<Space> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            c.to_string()
                .parse::<Space>()
                .expect("failed to parse Space")
        })
        .collect();

    let mut grid = Grid { spaces, w: width };
    while grid.run_sim2() {}
    //println!("{}", grid);

    Ok(grid
        .spaces
        .iter()
        .filter(|&s| s == &Space::Occupied)
        .count())
}
