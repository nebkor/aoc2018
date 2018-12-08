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
    let mut nodes = HashMap::new();

    let mut stack: Vec<usize> = Vec::with_capacity(input.len());

    let mut reading_md = false;
    let mut id: usize = 0;

    for (k, v) in input.iter().enumerate() {
        if stack.is_empty() {
            stack.push(k);
        }

        if let Some(cid) = stack.pop() {
            id = cid;
            nodes.entry(id).or_insert(Node::new());
            let num_children = input[id];
            let num_meta = input[id + 1];

            if k >= id + 2 && k < id + 2 + num_meta as usize && num_children == 0 {
                reading_md = true;
            } else {
                reading_md = false;
            }

            if reading_md {
                nodes.entry(id).and_modify(|n| n.md.push(*v));
            }

            if !(reading_md || k == id + 1) {
                stack.push(k)
            }
        }
    }

    nodes
}

fn main() {
    let input: Vec<u32> = include_str!("../input")
        .split_whitespace()
        .flat_map(|n| n.parse::<u32>())
        .collect();

    let nodes = parse(&input);

    let mut tot = 0;
    for node in nodes.values() {
        tot += node;
    }

    println!("checksum: {}", tot);
}
