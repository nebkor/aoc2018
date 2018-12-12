use std::ops::{Add, AddAssign};

#[derive(Clone, Default)]
struct Node {
    pub children: Vec<Node>,
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

impl Add<Node> for u32 {
    type Output = u32;
    fn add(self, rhs: Node) -> u32 {
        self + rhs.md.iter().sum::<u32>()
    }
}

impl AddAssign<&Node> for u32 {
    fn add_assign(&mut self, rhs: &Node) {
        *self = *self + rhs.md.iter().sum::<u32>();
    }
}

fn parse(input: &mut impl Iterator<Item = u32>) -> Node {
    let nc: u32 = input.next().unwrap();
    let nmd: u32 = input.next().unwrap();

    let mut node = Node::new();
    for _ in 0..nc {
        node.children.push(parse(input));
    }

    node.md = input.take(nmd as usize).collect();

    node
}

fn sum_md(node: &Node) -> u32 {
    let mut tot = 0;
    tot += node;
    for c in node.children.iter() {
        tot += sum_md(c);
    }

    tot
}

fn main() {
    let input: Vec<u32> = include_str!("../input")
        .split_whitespace()
        .flat_map(|n| n.parse::<u32>())
        .collect();

    let nodes = parse(&mut input.into_iter());

    let tot: u32 = sum_md(&nodes);

    println!("checksum: {}", tot);
}
