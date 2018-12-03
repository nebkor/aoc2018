use lazy_static::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
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

impl Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(id: {}, starting at {}, {})",
            self.id, self.xrange.0, self.yrange.0
        )
    }
}

fn main() {
    let input = get_input("day3");

    let file = read_file(&input);

    let mut cloth_map: HashMap<(u32, u32), HashSet<u32>> = HashMap::new(); // coordinates to IDs
    let mut singles: HashSet<u32> = HashSet::new();

    for entry in file.lines() {
        let claim = Claim::new(&entry.unwrap());
        let _ = singles.insert(claim.id);

        for x in claim.startx()..claim.endx() {
            for y in claim.starty()..claim.endy() {
                cloth_map
                    .entry((x, y))
                    .or_insert(HashSet::new())
                    .insert(claim.id);
            }
        }
    }

    let tot: u32 = cloth_map
        .values()
        .map(|s| match s.len() {
            x if x > 1 => {
                for id in s.iter() {
                    let _ = singles.remove(id);
                }
                1
            }
            _ => 0,
        })
        .sum();
    println!("Found {} square inches multiply-claimed.", tot);

    for id in singles.iter() {
        println!("Only {} made it through intact.", id);
    }
}
