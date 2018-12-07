use std::collections::HashMap;
use std::i32::{MAX, MIN};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pin(i32, i32);

type Weight = (u32, Pin); // each coordinate has a distance to a pin

struct Grid {
    pub upper_left: Pin,
    pub lower_right: Pin,
}

impl Grid {
    fn new(upper_left: Pin, lower_right: Pin) -> Self {
        Grid {
            upper_left,
            lower_right,
        }
    }

    fn is_finite(&self, p: &Pin) -> bool {
        self.upper_left.0 < p.0
            && self.lower_right.0 > p.0
            && self.upper_left.1 < p.1
            && self.lower_right.1 > p.1
    }
}

fn main() {
    let input: Vec<Pin> = include_str!("../input")
        .lines()
        .map(|line| {
            let l: Vec<_> = line.split(',').collect();
            Pin(l[0].trim().parse().unwrap(), l[1].trim().parse().unwrap())
        })
        .collect();

    let mut pins: HashMap<Pin, Weight> = HashMap::new();

    let mut lx = MAX;
    let mut ly = MAX;
    let mut rx = MIN;
    let mut ry = MIN;

    for p in input.iter() {
        lx = lx.min(p.0);
        ly = ly.min(p.1);
        rx = rx.max(p.0);
        ry = ry.max(p.1);
    }

    let grid = Grid::new(Pin(lx, ly), Pin(rx, ry));
}
