use clap::clap_app;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let input_file: String = clap_app!(day1 =>
                                (@arg INPUT: +required "What file has the input.")
    )
    .get_matches()
    .value_of("INPUT")
    .unwrap()
    .into();

    let mut set = HashSet::new();
    let mut sum = 0;

    let file = File::open(input_file)?;
    for line in BufReader::new(file).lines() {
        let x: i32 = line.unwrap().parse().unwrap();
        sum += x;
        println!("current sum: {}", sum);
        if !set.insert(sum) {
            println!("Failed to insert {}", sum);
        }
    }
    println!("{}", sum);
    Ok(())
}
