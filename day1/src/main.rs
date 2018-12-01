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
    let mut did_sum = false;
    let mut found = false;

    while !found {
        let file = File::open(&input_file)?;
        for line in BufReader::new(file).lines() {
            let x: i32 = line.unwrap().parse().unwrap();
            sum += x;
            if !set.insert(sum) {
                println!("Failed to insert {}", sum);
                found = true;
                break;
            }
        }
        if !did_sum {
            println!("Total: {}", sum);
            did_sum = true;
        }
    }

    Ok(())
}
