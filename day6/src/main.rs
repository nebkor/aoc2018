use std::collections::HashMap;
use std::i32::{MAX, MIN};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pin(i32, i32);

type Weight = (i32, Pin); // each coordinate has a distance to a pin

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

    let mut grid_weights: HashMap<Pin, Weight> = HashMap::new();

    let mut areas: HashMap<Pin, u32> = HashMap::new();

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

    for p in input.iter() {
        for x in grid.upper_left.0..grid.lower_right.0 {
            for y in grid.upper_left.1..grid.lower_right.1 {
                let dx = (x - p.0).abs();
                let dy = (y - p.1).abs();

                let weight = (dx + dy, *p);
                let w = grid_weights.entry(Pin(x, y)).or_insert(weight);
                if w.0 > weight.0 {
                    *w = weight;
                }
            }
        }
    }

    for weight in grid_weights.values() {
        let pin = weight.1;

        if grid.is_finite(&pin) {
            *areas.entry(pin).or_insert(0) += 1;
        }
    }

    let max = areas.values().max().unwrap_or(&666);

    println!("Max area is {}", max);
}
