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
        1 => (index % size as isize) as usize,
        _ => ((size as isize + index) % size as isize) as usize,
    }
}

fn next_player(cur: usize, tot: usize) -> usize {
    (cur + 1) % tot
}

fn main() {
    let (num_players, num_marbles) = get_input();

    let mut marbs = 0..num_marbles;
    let mut ring: Vec<usize> = Vec::new();
    let mut score: Vec<usize> = vec![0; num_players];

    let mut elf: usize = 0;
    let mut cm: isize = 0;
    while let Some(marble) = marbs.next() {
        if marble % 23 == 0 {
            score[elf] += marble;
            let remove_at = get_index(cm - 7, ring.len());
            score[elf] += ring.remove(remove_at);
            cm = (remove_at + 1) as isize;
            elf = next_player(elf, num_players);
            continue;
        }

        if ring.len() < 2 {
            ring.push(marble);
            cm = (ring.len() - 1) as isize;
            elf = next_player(elf, num_players);
            continue;
        }

        let insert_at = get_index(cm + 1, ring.len());
        ring.insert(insert_at, marble);
        cm = insert_at as isize;
        elf = next_player(elf, num_players);
    }

    let high_score = score.iter().max();

    println!("part 1: {}", high_score.unwrap_or(&0));
}
