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
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let lines2: Vec<_> = lines.iter().filter(|x| has_n_chars(x, 2)).collect();
    let twos = lines2.len();

    let mut lines3: Vec<_> = lines.iter().filter(|x| has_n_chars(x, 3)).collect();
    let threes = lines3.len();

    lines3.extend(lines2.into_iter());

    println!("3s: {}, 2s: {}, 3s * 2s = {}", threes, twos, threes * twos);

    for i in 0..lines3.len() {
        let base = lines3[i];
        for s in lines3[i..].iter() {
            match hamming(base, s) {
                1 => println!("{}", common(s, base)),
                _ => (),
            }
        }
    }

    Ok(())
}
