use clap::clap_app;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn c2i(c: char) -> usize {
    let a: u32 = 'a' as u32;
    (c as u32 - a) as usize
}

fn has_n_chars(s: &str, n: u32) -> bool {
    let mut v: Vec<u32> = vec![0; 26];
    for c in s.chars() {
        let i = c2i(c);
        v[i] = v[i] + 1;
    }
    for c in v.iter() {
        if n == *c {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let input_file: String = clap_app!(day2 =>
                                (@arg INPUT: +required "What file has the input.")
    )
    .get_matches()
    .value_of("INPUT")
    .unwrap()
    .into();

    let file = File::open(&input_file)?;
    let lines2: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|x| has_n_chars(x, 2))
        //.inspect(|x| println!("{}", x))
        .collect();
    let twos = lines2.len();

    let lines3: Vec<_> = lines2.into_iter().filter(|x| has_n_chars(x, 3)).collect();
    let threes = lines3.len();

    println!("{} * {} = {}", threes, twos, threes * twos);

    Ok(())
}
