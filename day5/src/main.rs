#![feature(drain_filter)]

fn react(polymer: String) -> String {
    let mut input = polymer.into_bytes();
    let mut prev_len = input.len() + 1;

    while input.len() < prev_len {
        prev_len = input.len();

        let mut pi: usize = 0;
        for i in 0..prev_len {
            let pv = input[pi] as i32;
            let cv = input[i] as i32;
            if (pv - cv).abs() == 32 {
                input[pi] = '_' as u8;
                input[i] = '_' as u8;
            }
            pi = i;
        }

        input = input.drain_filter(|x| *x != '_' as u8).collect();
    }
    (&(String::from_utf8(input).unwrap())).trim().to_owned()
}

fn fstring(input: String, c: char) -> String {
    let mut b = input.into_bytes();

    let s = b
        .drain_filter(|x| (*x as char).to_ascii_lowercase() != c)
        .collect();
    (&(String::from_utf8(s).unwrap())).trim().to_owned()
}

fn main() {
    let input = include_str!("../input").to_owned();

    // part 1
    let reacted = react(input.clone());
    println!(
        "There are {} units left after fully reacting.",
        reacted.len()
    );

    // part 2
    let mut min_len = input.len();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let short = fstring(input.clone(), c);

        let tlen = react(short).len();
        if tlen < min_len {
            min_len = tlen;
        }
    }

    println!("Min polymer possible is {}", min_len);
}
