#[derive(Default)]
struct Node {
    pub children: Vec<Node>,
    pub md: Vec<u32>,
}

impl Node {
    fn mdsum(&self) -> u32 {
        self.md.iter().sum()
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

fn sum1(node: &Node) -> u32 {
    node.mdsum() + node.children.iter().map(sum1).sum::<u32>()
}

fn sum2(node: &Node) -> u32 {
    match node.children.len() {
        0 => node.mdsum(),
        _ => node
            .md
            .iter()
            .filter(|&x| *x > 0)
            .flat_map(|&i| node.children.get((i - 1) as usize))
            .map(sum2)
            .sum(),
    }
}

fn main() {
    let input: Vec<u32> = include_str!("../input")
        .split_whitespace()
        .flat_map(|n| n.parse::<u32>())
        .collect();

    let tree = parse(&mut input.into_iter());
    println!("checksum: {}", sum1(&tree));
    println!("checksum2: {}", sum2(&tree));
}
