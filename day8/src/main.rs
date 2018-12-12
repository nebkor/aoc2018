use std::ops::{Add, AddAssign};

#[derive(Clone, Default)]
struct Node {
    pub children: Vec<Node>,
    pub md: Vec<u32>,
}

impl Node {
    fn mdsum(&self) -> u32 {
        self.md.iter().sum()
    }
}

impl Add<&Node> for u32 {
    type Output = u32;
    fn add(self, rhs: &Node) -> u32 {
        self + rhs.mdsum()
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

    let mut node = Node::default();
    for _ in 0..nc {
        node.children.push(parse(input));
    }

    node.md = input.take(nmd as usize).collect();

    node
}

fn sum_md(node: &Node) -> u32 {
    node.children.iter().map(sum_md).sum::<u32>() + node
}

fn sum2(node: &Node) -> u32 {
    match node.children.len() {
        0 => node.mdsum(),
        _ => {
            let mut tot = 0;
            // use metadata entries as indices into children vec, see if they're leaf nodes.
            for i in node.md.iter().filter(|&x| *x > 0) {
                if let Some(c) = node.children.get((*i - 1) as usize) {
                    tot += sum2(c);
                }
            }
            tot
        }
    }
}

fn main() {
    let input: Vec<u32> = include_str!("../input")
        .split_whitespace()
        .flat_map(|n| n.parse::<u32>())
        .collect();

    let nodes = parse(&mut input.into_iter());

    let tot: u32 = sum_md(&nodes);

    println!("checksum: {}", tot);

    println!("checksum2: {}", sum2(&nodes));
}
