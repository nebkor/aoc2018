use lazy_static::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use utils::*;

struct Claim {
    pub id: u32,
    xrange: (u32, u32),
    yrange: (u32, u32),
}

impl Claim {
    fn new(ent: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+,\d+): (\d+x\d+)").unwrap();
        }

        let caps = RE.captures(ent).unwrap();
        let id: u32 = caps[1].parse().unwrap();
        let startx: u32 = caps[2].split(',').collect::<Vec<_>>()[0].parse().unwrap();
        let starty: u32 = caps[2].split(',').collect::<Vec<_>>()[1].parse().unwrap();
        let spanx: u32 = caps[3].split('x').collect::<Vec<_>>()[0].parse().unwrap();
        let spany: u32 = caps[3].split('x').collect::<Vec<_>>()[1].parse().unwrap();

        Claim {
            id,
            xrange: (startx, startx + spanx),
            yrange: (starty, starty + spany),
        }
    }

    fn startx(&self) -> u32 {
        self.xrange.0
    }
    fn endx(&self) -> u32 {
        self.xrange.1
    }

    fn starty(&self) -> u32 {
        self.yrange.0
    }
    fn endy(&self) -> u32 {
        self.yrange.1
    }
}

fn main() {
    let input = get_input("day3");

    let file = read_file(&input);

    let mut hmap: HashMap<(u32, u32), HashSet<u32>> = HashMap::new(); // coordinates to IDs

    for entry in file.lines() {
        let claim = Claim::new(&entry.unwrap());

        for x in claim.startx()..claim.endx() {
            for y in claim.starty()..claim.endy() {
                hmap.entry((x, y))
                    .or_insert(HashSet::new())
                    .insert(claim.id);
            }
        }
    }

    let tot: u32 = hmap.values().map(|s| if s.len() > 1 { 1 } else { 0 }).sum();
    println!("Found {} square inches multiply-claimed.", tot)
}
