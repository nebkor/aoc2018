use clap::{App, Arg};

use std::collections::BinaryHeap;

type P = (i32, i32);
type Grid = BinaryHeap<(i32, P, i32)>; // power-level, point

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

fn square_power(id: P, size: i32, serial: i32) -> i32 {
    let mut ret: i32 = 0;
    for y in id.1..(id.1 + size) {
        for x in id.0..(id.0 + size) {
            ret += cell_power(serial, (x, y));
        }
    }

    ret
}

fn main() {
    let serial = get_input("day11");

    let mut grid = Grid::with_capacity(2);

    for s in 1..=300 {
        let mut brk = true;
        for y in 1..=(300 - s) {
            for x in 1..=(300 - s) {
                let xy = (x, y);
                let pl = square_power(xy, s, serial);
                brk = brk && pl < 0;
                if let Some((po, _, _)) = grid.peek() {
                    if *po < pl {
                        grid.push((pl, xy, s));
                    }
                } else {
                    grid.push((pl, xy, s));
                }
            }
        }
        if brk {
            break;
        }
    }

    let (pl, id, s) = grid.pop().unwrap();

    println!("Grid {},{},{} has {} power", id.0, id.1, s, pl);
}
