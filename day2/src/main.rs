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

fn has_2_or_3_same(s: &str) -> Option<(String, usize)> {
    let mut two = 1;
    let mut three = 1;
    let mut has_2_or_3 = false;

    if has_n_chars(s, 2) {
        two = 2;
        has_2_or_3 = true;
    }

    if has_n_chars(s, 3) {
        three = 3;
        has_2_or_3 = true;
    }

    if has_2_or_3 {
        Some((s.to_owned(), two * three))
    } else {
        None
    }
}

fn hamming(s1: &str, s2: &str) -> usize {
    let ret: usize = s1
        .chars()
        .zip(s2.chars())
        .map(|(x, y)| if x == y { 0 } else { 1 })
        .sum();

    ret + (s1.len() as isize - s2.len() as isize).abs() as usize
}

fn common(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .collect()
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
    let ids: Vec<(String, usize)> = BufReader::new(file)
        .lines()
        .filter_map(|l| has_2_or_3_same(&l.unwrap()))
        .collect();

    let mut twos = 0;
    let mut threes = 0;
    for id in ids.iter() {
        match id.1 {
            2 => twos += 1,
            3 => threes += 1,
            6 => {
                twos += 1;
                threes += 1
            }
            _ => (),
        }
    }

    println!("3s: {}, 2s: {}, 3s * 2s = {}", threes, twos, threes * twos);

    for i in 0..ids.len() {
        let base = &ids[i].0;
        for s in ids[i..].iter() {
            if hamming(base, &s.0) == 1 {
                {
                    println!("{}", common(&s.0, base))
                }
            }
        }
    }

    Ok(())
}
