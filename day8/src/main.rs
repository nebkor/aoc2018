use std::collections::HashMap;
use std::ops::{Add, AddAssign};

#[derive(Clone)]
struct Node {
    pub children: Vec<usize>,
    pub md: Vec<u32>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: Vec::new(),
            md: Vec::new(),
        }
    }
}

impl Add for Node {
    type Output = u32;
    fn add(self, rhs: Self) -> u32 {
        self.md.iter().sum::<u32>() + rhs.md.iter().sum::<u32>()
    }
}

impl AddAssign<&Node> for u32 {
    fn add_assign(&mut self, rhs: &Node) {
        *self = *self + rhs.md.iter().sum::<u32>();
    }
}

fn parse(input: &Vec<u32>) -> HashMap<usize, Node> {
    unimplemented!()
}

fn main() {
    let input: Vec<u32> = include_str!("../input")
        .split_whitespace()
        .flat_map(|n| n.parse::<u32>())
        .collect();

    let nodes: HashMap<usize, Node> = parse(&input);

    let mut tot = 0;
    for node in nodes.values() {
        tot += node;
    }

    println!("checksum: {}", tot);
}
