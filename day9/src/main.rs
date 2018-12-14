use clap::{App, Arg};

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

fn get_index(index: isize, size: usize) -> usize {
    if size == 0 {
        return 0;
    }
    match index.signum() {
        0 => 0,
        1 => index as usize % size,
        _ => size - (index.abs() as usize % size),
    }
}

fn main() {
    let (num_players, num_marbles) = get_input();

    let marbs = 0..num_marbles;
    let elves = 0..num_players;

    let mut ring: Vec<usize> = Vec::with_capacity(num_marbles);
    let mut scores: Vec<usize> = vec![0; num_players];

    let mut cur: isize = 0;
    for (marble, elf) in marbs.zip(elves.cycle()) {
        match ring.len() {
            0 => {
                ring.push(marble);
                cur = 0;
            }
            1 => {
                ring.push(marble);
                cur = 1;
            }
            2 => {
                ring.insert(1, marble);
                cur = 1;
            }
            _ => {
                if marble % 23 == 0 {
                    scores[elf] += marble;
                    let remove = get_index(cur - 7, ring.len());
                    scores[elf] += ring.remove(remove);
                    cur = remove as isize;
                } else {
                    let insert = get_index(cur + 2, ring.len());
                    ring.insert(insert, marble);
                    cur = insert as isize;
                }
            }
        }
    }

    let high_score = scores.iter().max();

    println!("Max score: {}", high_score.unwrap_or(&0));
}
