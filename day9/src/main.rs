use clap::{App, Arg};
use std::collections::VecDeque;

trait Rotate {
    fn rotate(&mut self, rot: isize);
}

impl Rotate for VecDeque<usize> {
    fn rotate(&mut self, rot: isize) {
        if rot == 0 || self.len() == 0 {
            return;
        }
        if rot > 0 {
            for _ in 0..rot {
                let t = self.pop_front().expect("rotation error");
                self.push_back(t);
            }
        } else {
            for _ in 0..rot.abs() {
                let t = self.pop_back().expect("rotation error");
                self.push_front(t);
            }
        }
    }
}

pub fn get_input() -> (usize, usize) {
    let matches = App::new("day9")
        .arg(
            Arg::with_name("PLAYERS")
                .help("How many players are there?")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("POINTS")
                .help("How many marbles are there?")
                .required(true)
                .index(2),
        )
        .get_matches();

    (
        matches
            .value_of("PLAYERS")
            .unwrap()
            .parse()
            .expect("Couldn't parse PLAYERS as a number."),
        matches
            .value_of("POINTS")
            .unwrap()
            .parse()
            .expect("Couldn't parse POINTS as a number."),
    )
}

fn main() {
    let (num_players, num_marbles) = get_input();

    let marbs = 0..num_marbles + 1;
    let elves = 0..num_players;

    let mut ring: VecDeque<usize> = VecDeque::with_capacity(num_marbles);
    let mut scores: Vec<usize> = vec![0; num_players];

    for (marble, elf) in marbs.zip(elves.cycle()) {
        if marble % 23 == 0 && marble != 0 {
            scores[elf] += marble;
            ring.rotate(-7);
            scores[elf] += ring.pop_front().unwrap();
        } else {
            ring.rotate(2);
            ring.push_front(marble);
        }
    }

    let high_score = scores.iter().max();

    println!("Max score: {}", high_score.unwrap_or(&0));
}
