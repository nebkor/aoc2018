use lazy_static::lazy_static;
use regex::Regex;
pub use std::i32::{MAX, MIN};

pub type Vp = (i32, i32); // vector/point

pub struct Rect {
    pub xmin: i32,
    pub ymin: i32,
    pub xspan: i32,
    pub yspan: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Beacon {
    pub init_pos: Vp,
    pub vel: Vp,
}

impl Beacon {
    pub fn new(init_pos: Vp, vel: Vp) -> Self {
        Beacon { init_pos, vel }
    }

    pub fn t(&self, t: i32) -> Vp {
        (
            self.init_pos.0 + (t * self.vel.0),
            self.init_pos.1 + (t * self.vel.1),
        )
    }
}

pub struct Sky {
    pub beacons: Vec<Beacon>,
}

impl Sky {
    pub fn t(&self, t: i32) -> (Vec<Vp>, Rect) {
        let mut minx = MAX;
        let mut miny = MAX;
        let mut maxx = MIN;
        let mut maxy = MIN;

        let nps = self
            .beacons
            .iter()
            .map(|b| {
                let bt = b.t(t);
                minx = minx.min(bt.0);
                miny = miny.min(bt.1);

                maxx = maxx.max(bt.0);
                maxy = maxy.max(bt.1);

                bt
            })
            .collect();

        (
            nps,
            Rect {
                xmin: minx,
                ymin: miny,
                xspan: maxx - minx,
                yspan: maxy - miny,
            },
        )
    }

    pub fn print(&self, t: i32) -> String {
        let (points, rect) = self.t(t);

        let mut grid = vec![vec![' '; 1 + rect.xspan as usize]; 1 + rect.yspan as usize];

        for p in points.iter() {
            grid[((p.1 - rect.ymin) as usize)][((p.0 - rect.xmin) as usize)] = '#';
        }

        grid.into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn get_input() -> Vec<Beacon> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"position=<(?P<px>[ -]?\d+), (?P<py>[ -]?\d+)> velocity=<(?P<vx>[ -]?\d+), (?P<vy>[ -]?\d+)"
        )
        .unwrap();
    }

    include_str!("../input")
        .lines()
        .map(|l| {
            let caps = RE.captures(l).unwrap();

            Beacon::new(
                (
                    caps.name("px")
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<i32>()
                        .unwrap(),
                    caps.name("py")
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<i32>()
                        .unwrap(),
                ),
                (
                    caps.name("vx")
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<i32>()
                        .unwrap(),
                    caps.name("vy")
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<i32>()
                        .unwrap(),
                ),
            )
        })
        .collect()
}
