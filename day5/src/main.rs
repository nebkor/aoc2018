#![feature(drain_filter)]

fn main() {
    let mut input = include_str!("../input").to_owned().into_bytes();

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
                // println!(
                //     "Marking positions {},{} with {},{} for removal.",
                //     pi, i, pv as u8 as char, cv as u8 as char
                // );
            }
            pi = i;
        }

        input = input.drain_filter(|x| *x != '_' as u8).collect();
    }

    let input = (&(String::from_utf8(input).unwrap())).trim().to_owned();
    println!("There are {} units left after fully reacting.", input.len());
}
