use clap::{App, Arg};

use std::collections::BinaryHeap;

type P = (i32, i32);
type Grid = BinaryHeap<(i32, P)>; // power-level, point

pub fn get_input(app_name: &str) -> i32 {
    App::new(app_name)
        .arg(
            Arg::with_name("INPUT")
                .help("The grid serial number; must be a positive integer.")
                .required(true)
                .index(1),
        )
        .get_matches()
        .value_of("INPUT")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_hundos(num: i32) -> i32 {
    ((num % 1000 - num % 100) / 100).abs()
}

fn cell_power(serial: i32, cell: P) -> i32 {
    let rack_id = cell.0 + 10;
    let mut pl = rack_id * cell.1;
    pl += serial;
    pl *= rack_id;
    get_hundos(pl) - 5
}

fn square_power(id: P, serial: i32) -> i32 {
    let mut ret: i32 = 0;
    for y in id.1..=(id.1 + 2) {
        for x in id.0..=(id.0 + 2) {
            ret += cell_power(serial, (x, y));
        }
    }

    ret
}

fn main() {
    let serial = get_input("day11");

    let mut grid = Grid::new();

    for y in 1..=298 {
        for x in 1..=298 {
            let xy = (x, y);
            let pl = square_power(xy, serial);
            grid.push((pl, xy));
        }
    }

    let (pl, id) = grid.pop().unwrap();

    println!("Grid {:?} has {} power", id, pl);
}
